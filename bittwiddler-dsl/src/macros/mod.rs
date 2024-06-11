//! Proc macros to be used on field accessor related structs

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Field, FnArg, ImplItem, ItemImpl, ItemStruct, Meta,
};

pub fn bittwiddler_hierarchy_level(item: TokenStream) -> TokenStream {
    let struct_inp = match syn::parse2::<ItemStruct>(item.clone()) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };

    let ident = &struct_inp.ident;
    let (fields, fields_are_named) = match struct_inp.fields {
        syn::Fields::Named(x) => (x.named.into_iter(), true),
        syn::Fields::Unnamed(x) => (x.unnamed.into_iter(), false),
        syn::Fields::Unit => (Punctuated::<Field, Comma>::new().into_iter(), false),
    };
    let fields_dump = fields.enumerate().map(|(field_i, f)| {
        let field_name = if fields_are_named {
            f.ident.as_ref().unwrap().to_string()
        } else {
            field_i.to_string()
        };
        let field_access = if fields_are_named {
            let ident = f.ident;
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
    });

    quote! {
        #item

        #[cfg(feature = "alloc")]
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

        #[cfg(feature = "alloc")]
        impl ::bittwiddler_core::prelude::HumanLevelThatHasState for #ident {
            fn _human_dump_my_state(&self, _dump: &mut dyn ::bittwiddler_core::prelude::HumanSinkForStatePieces) {
                #(#fields_dump)*
            }
        }
    }
}

pub fn bittwiddler_properties(item: TokenStream) -> TokenStream {
    let mut impl_inp = match syn::parse2::<ItemImpl>(item) {
        Ok(x) => x,
        Err(e) => return e.to_compile_error(),
    };

    let target_ty = &impl_inp.self_ty;
    let mut prop_idx = 0usize;
    let mut fields_strs = Vec::new();
    let mut sublevels_str = Vec::new();
    let mut make_subfields = Vec::new();
    let mut make_sublevels = Vec::new();
    for item in &mut impl_inp.items {
        if let ImplItem::Fn(impl_fn) = item {
            let mut is_prop = false;
            for (attr_i, attr) in impl_fn.attrs.iter().enumerate() {
                if let Meta::Path(p) = &attr.meta {
                    if p.is_ident("property") {
                        impl_fn.attrs.remove(attr_i);
                        is_prop = true;
                        break;
                    }
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
                    ).unwrap()  // fixme unwrap
                }
            });
            let make_obj = if has_self {
                quote! {::bittwiddler_core::prelude::BoxReexport::new(self.#ident(#(#args_parse_bits),*))}
            } else {
                quote! {Self::#ident(#(#args_parse_bits),*)}
            };
            let make_match = quote! {
                #prop_idx => ::bittwiddler_core::prelude::BoxReexport::new(#make_obj),
            };
            if is_prop {
                make_subfields.push(make_match);
            } else {
                make_sublevels.push(make_match);
            }

            prop_idx += 1;
        }
    }

    quote! {
        #impl_inp

        #[cfg(feature = "alloc")]
        impl ::bittwiddler_core::prelude::HumanLevelDynamicAccessor for #target_ty {
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
            ) -> ::bittwiddler_core::prelude::BoxReexport<dyn ::bittwiddler_core::prelude::PropertyAccessorDyn> {
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
                    _ => unreachable!(),
                }
            }

            fn _human_descend_sublevel(
                &self,
                idx: ::core::primitive::usize,
                _params: &[&::core::primitive::str],
            ) -> ::bittwiddler_core::prelude::BoxReexport<dyn ::bittwiddler_core::prelude::HumanLevelDynamicAccessor> {
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
                    _ => unreachable!(),
                }
            }
        }
    }
}