//! Proc macros to be used on field accessor related structs

use std::borrow::Borrow;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parser, punctuated::Punctuated, token::Comma, Expr, Field, FnArg, ImplItem, ItemImpl,
    ItemStruct, Lit, Meta, MetaNameValue, PathArguments, ReturnType, Type,
};

fn is_bittwiddler_attr(meta: &Meta, attr: &str) -> bool {
    if let Meta::Path(p) = meta {
        if p.leading_colon.is_none() && p.segments.len() == 2 {
            let seg0 = &p.segments[0];
            let seg1 = &p.segments[1];
            if seg0.arguments == PathArguments::None && seg1.arguments == PathArguments::None {
                seg0.ident == "bittwiddler" && seg1.ident == attr
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

struct NoStdSettings {
    alloc_feature_gate: Option<String>,
}

fn parse_no_std_settings(attrs: &Punctuated<MetaNameValue, Comma>) -> NoStdSettings {
    let mut alloc_feature_gate = None;

    for attr in attrs {
        if attr.path.is_ident("alloc_feature_gate") {
            if let Expr::Lit(x) = &attr.value {
                if let Lit::Str(x) = &x.lit {
                    alloc_feature_gate = Some(x.value());
                }
            }
        }
    }

    NoStdSettings { alloc_feature_gate }
}

pub fn bittwiddler_hierarchy_level(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_inp = match Punctuated::<MetaNameValue, Comma>::parse_terminated.parse2(attr) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };
    let mut struct_inp = match syn::parse2::<ItemStruct>(item) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };

    let settings = parse_no_std_settings(&attr_inp);
    let alloc_feature_gate = if let Some(alloc_feature) = settings.alloc_feature_gate {
        quote! {
            #[cfg(feature = #alloc_feature)]
        }
    } else {
        TokenStream::new()
    };

    let ident = &struct_inp.ident;
    let (fields, fields_are_named): (Box<dyn Iterator<Item = &mut Field>>, bool) =
        match struct_inp.fields {
            syn::Fields::Named(ref mut x) => (Box::new(x.named.iter_mut()), true),
            syn::Fields::Unnamed(ref mut x) => (Box::new(x.unnamed.iter_mut()), false),
            syn::Fields::Unit => (Box::new([].iter_mut()), false),
        };
    let fields_dump = fields
        .enumerate()
        .map(|(field_i, f)| {
            for (attr_i, attr) in f.attrs.iter().enumerate() {
                if is_bittwiddler_attr(&attr.meta, "skip") {
                    f.attrs.remove(attr_i);
                    return quote! {};
                }
            }

            let field_name = if fields_are_named {
                f.ident.as_ref().unwrap().to_string()
            } else {
                field_i.to_string()
            };
            let field_access = if fields_are_named {
                let ident = &f.ident;
                quote! {&self.#ident}
            } else {
                quote! {&self.#field_i}
            };
            quote! {{
                if ::bittwiddler_core::prelude::StatePiece::_should_add_piece(#field_access) {
                    ::bittwiddler_core::prelude::HumanSinkForStatePieces::add_state_piece(
                        _dump,
                        #field_name,
                        &::bittwiddler_core::prelude::StatePiece::to_human_string(#field_access)
                    );
                }
            }}
        })
        .collect::<Vec<_>>();

    quote! {
        #struct_inp

        #alloc_feature_gate
        impl ::bittwiddler_core::prelude::StatePiece for #ident {
            fn _should_add_piece(&self) -> ::core::primitive::bool {
                false
            }
            fn to_human_string(&self) -> ::bittwiddler_core::prelude::CowReexport<'static, ::core::primitive::str> {
                ::core::convert::Into::into("")
            }
            fn from_human_string(s: &str) -> ::core::result::Result<Self, ()>
            where
                Self: Sized
            {
                ::core::result::Result::Err(())
            }
        }

        #alloc_feature_gate
        impl ::bittwiddler_core::prelude::HumanLevelThatHasState for #ident {
            fn _human_dump_my_state(&self, _dump: &mut dyn ::bittwiddler_core::prelude::HumanSinkForStatePieces) {
                #(#fields_dump)*
            }
        }
    }
}

pub fn bittwiddler_properties(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_inp = match Punctuated::<MetaNameValue, Comma>::parse_terminated.parse2(attr) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };
    let mut impl_inp = match syn::parse2::<ItemImpl>(item) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };

    let settings = parse_no_std_settings(&attr_inp);
    let alloc_feature_gate = if let Some(alloc_feature) = settings.alloc_feature_gate {
        quote! {
            #[cfg(feature = #alloc_feature)]
        }
    } else {
        TokenStream::new()
    };

    let target_ty = &impl_inp.self_ty;
    let generics = &impl_inp.generics;
    let target_ty_ident = if let Type::Path(p) = target_ty.borrow() {
        &p.path.segments.last().unwrap().ident
    } else {
        panic!("Requrires impl on a path");
    };
    let automagic_trait_id = format_ident!("{}AutomagicRequiredFunctions", target_ty_ident);

    let mut prop_idx = 0usize;
    let mut sublevel_idx = 0usize;
    let mut fields_strs = Vec::new();
    let mut sublevels_str = Vec::new();
    let mut make_subfields = Vec::new();
    let mut make_sublevels = Vec::new();
    let mut automagic_trait_fns = Vec::new();
    let mut iter_all_subfields = Vec::new();
    let mut iter_all_sublevels = Vec::new();
    for item in &mut impl_inp.items {
        if let ImplItem::Fn(impl_fn) = item {
            let mut is_prop = false;
            for (attr_i, attr) in impl_fn.attrs.iter().enumerate() {
                if is_bittwiddler_attr(&attr.meta, "property") {
                    impl_fn.attrs.remove(attr_i);
                    is_prop = true;
                    break;
                }
            }
            let mut is_conditional = false;
            for (attr_i, attr) in impl_fn.attrs.iter().enumerate() {
                if is_bittwiddler_attr(&attr.meta, "conditional") {
                    impl_fn.attrs.remove(attr_i);
                    is_conditional = true;
                    break;
                }
            }

            let mut num_args = impl_fn.sig.inputs.len();
            let mut has_self = false;
            if num_args >= 1 {
                if let FnArg::Receiver(_) = impl_fn.sig.inputs[0] {
                    has_self = true;
                    num_args -= 1;
                }
            }

            let ident = &impl_fn.sig.ident;
            let output_ty = if let ReturnType::Type(_, ty) = &impl_fn.sig.output {
                ty
            } else {
                panic!("Function must return something");
            };

            // string name
            if is_prop {
                fields_strs.push(ident.to_string());
            } else {
                sublevels_str.push(ident.to_string());
            }

            // construct a specific sublevel
            let args_parse_bits = (0..num_args).map(|arg_i| {
                quote! {
                    ::bittwiddler_core::prelude::StatePiece::from_human_string(
                        &_params[#arg_i]
                    )?
                }
            });
            let make_obj = if has_self {
                quote! {self.#ident(#(#args_parse_bits),*)}
            } else {
                quote! {Self::#ident(#(#args_parse_bits),*)}
            };
            let thing_idx = if is_prop {
                quote! {#prop_idx}
            } else {
                quote! {#sublevel_idx}
            };
            if is_prop {
                make_subfields.push(quote! {
                    #thing_idx => ::core::result::Result::Ok(::bittwiddler_core::prelude::BoxReexport::new(::bittwiddler_core::prelude::BoxReexport::new(#make_obj))),
                });
            } else {
                make_sublevels.push(quote! {
                    #thing_idx => ::core::result::Result::Ok(::bittwiddler_core::prelude::BoxReexport::new(#make_obj)),
                });
            }

            // this needs to be impl-ed by user
            if is_conditional || num_args > 0 {
                let automagic_fn_ident = format_ident!("_automagic_construct_all_{}", ident);
                automagic_trait_fns.push(quote! {
                    fn #automagic_fn_ident(&self) -> impl ::core::iter::Iterator<Item = #output_ty>;
                });
            }

            // construct *all* sublevels
            let coerce = if is_prop {
                quote! {
                    ::bittwiddler_core::prelude::BoxReexport::new(
                        ::bittwiddler_core::prelude::BoxReexport::new(obj),
                    )
                        as ::bittwiddler_core::prelude::BoxReexport<
                            dyn ::bittwiddler_core::prelude::PropertyAccessorDyn,
                        >
                }
            } else {
                quote! {
                    ::bittwiddler_core::prelude::BoxReexport::new(obj)
                    as ::bittwiddler_core::prelude::BoxReexport<
                        dyn ::bittwiddler_core::prelude::HumanLevelDynamicAccessor,
                    >
                }
            };
            let make_all_of_this_prop = if is_conditional || num_args > 0 {
                let automagic_fn_ident = format_ident!("_automagic_construct_all_{}", ident);
                quote! {
                    #thing_idx => ::bittwiddler_core::prelude::BoxReexport::new(
                        #automagic_trait_id::#automagic_fn_ident(self).map(|obj| {
                            #coerce
                        })
                    ),
                }
            } else {
                quote! {
                    #thing_idx => ::bittwiddler_core::prelude::BoxReexport::new(::core::iter::IntoIterator::into_iter([{
                        let obj = #make_obj;
                        #coerce
                    }])),
                }
            };
            if is_prop {
                iter_all_subfields.push(make_all_of_this_prop);
            } else {
                iter_all_sublevels.push(make_all_of_this_prop);
            }

            if is_prop {
                prop_idx += 1;
            } else {
                sublevel_idx += 1;
            }
        }
    }

    quote! {
        #impl_inp

        #alloc_feature_gate
        impl #generics ::bittwiddler_core::prelude::HumanLevelDynamicAccessor for #target_ty {
            fn _human_fields(&self) -> &'static [&'static ::core::primitive::str] {
                &[
                    #(#fields_strs),*
                ]
            }
            fn _human_sublevels(&self) -> &'static [&'static ::core::primitive::str] {
                &[
                    #(#sublevels_str),*
                ]
            }

            fn _human_construct_field(
                &self,
                idx: ::core::primitive::usize,
                _params: &[&::core::primitive::str]
            ) -> ::core::result::Result<::bittwiddler_core::prelude::BoxReexport<dyn ::bittwiddler_core::prelude::PropertyAccessorDyn>, ()> {
                match idx {
                    #(#make_subfields)*
                    _ => unreachable!()
                }
            }
            fn _human_construct_all_fields<'s>(
                &'s self,
                idx: usize,
            ) -> ::bittwiddler_core::prelude::BoxReexport<
                dyn ::core::iter::Iterator<
                        Item = ::bittwiddler_core::prelude::BoxReexport<
                            dyn ::bittwiddler_core::prelude::PropertyAccessorDyn,
                        >,
                    > + 's,
            > {
                match idx {
                    #(#iter_all_subfields)*
                    _ => unreachable!(),
                }
            }

            fn _human_descend_sublevel(
                &self,
                idx: ::core::primitive::usize,
                _params: &[&::core::primitive::str],
            ) -> ::core::result::Result<::bittwiddler_core::prelude::BoxReexport<dyn ::bittwiddler_core::prelude::HumanLevelDynamicAccessor>, ()> {
                match idx {
                    #(#make_sublevels)*
                    _ => unreachable!()
                }
            }
            fn _human_construct_all_sublevels<'s>(
                &'s self,
                idx: ::core::primitive::usize,
            ) -> ::bittwiddler_core::prelude::BoxReexport<
                dyn ::core::iter::Iterator<
                        Item = ::bittwiddler_core::prelude::BoxReexport<
                            dyn ::bittwiddler_core::prelude::HumanLevelDynamicAccessor,
                        >,
                    > + 's,
            > {
                match idx {
                    #(#iter_all_sublevels)*
                    _ => unreachable!(),
                }
            }
        }

        #alloc_feature_gate
        trait #automagic_trait_id {
            #(#automagic_trait_fns)*
        }
    }
}
