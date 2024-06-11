use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::parse_bit_property::BitProperty;

pub struct Settings {
    pub enable_no_std: bool,
    pub alloc_feature_gate: Option<String>,
    pub emit_string_formatter: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            enable_no_std: false,
            alloc_feature_gate: None,
            emit_string_formatter: true,
        }
    }
}

pub fn emit(prop: &BitProperty, settings: &Settings) -> TokenStream {
    let alloc = if settings.enable_no_std {
        quote! {alloc}
    } else {
        quote! {std}
    };

    let prop_name_ident = Ident::new(&prop.name, Span::call_site());
    let prop_var_idents = prop
        .variants
        .iter()
        .chain(prop.catchall_variant.iter())
        .map(|x| Ident::new(&x.name, Span::call_site()))
        .collect::<Vec<_>>();
    let num_bits = if prop.variants.len() == 0 {
        0
    } else {
        prop.variants[0].pattern.len()
    };

    let mut prop_variants_decl = Vec::new();
    let mut var_decode_matches = Vec::new();
    let mut var_decode_str_matches = Vec::new();
    let mut var_encode_matches = Vec::new();
    let mut var_encode_str_matches = Vec::new();
    for (i, var) in prop
        .variants
        .iter()
        .chain(prop.catchall_variant.iter())
        .enumerate()
    {
        let var_ident = &prop_var_idents[i];
        let field_name_str: &String = &var.name;
        if var.keep_bits {
            prop_variants_decl.push(quote! {#var_ident([::core::primitive::bool; #num_bits])})
        } else {
            prop_variants_decl.push(quote! {#var_ident})
        }

        // decode
        let match_pat = if i == prop.variants.len() {
            quote! {_}
        } else {
            let match_bits = var.pattern.chars().map(|x| match x {
                '0' => quote! {false},
                '1' => quote! {true},
                'x' | 'X' => quote! {_},
                _ => unreachable!(),
            });
            quote! {[#(#match_bits),*]}
        };

        if var.keep_bits {
            var_decode_matches.push(quote! { #match_pat => Self::#var_ident(*bits) });
        } else {
            var_decode_matches.push(quote! { #match_pat => Self::#var_ident });
        }

        // decode from string
        if var.keep_bits {
            var_decode_str_matches.push(quote! { #field_name_str => {
                let mut bits = [false; #num_bits];
                let mut chars = ::core::primitive::str::chars(bits_s);
                for i in 0..#num_bits {
                    let b = ::core::iter::Iterator::next(&mut chars);
                    let b = ::core::option::Option::unwrap(b);
                    match b {
                        '0' => {
                            bits[i] = false;
                        }
                        '1' => {
                            bits[i] = true;
                        }
                        _ => {
                            return ::core::result::Result::Err(());
                        }
                    }
                }
                if ::core::iter::Iterator::next(&mut chars) != ::core::option::Option::Some(')') {
                    return ::core::result::Result::Err(());
                }
                if ::core::iter::Iterator::next(&mut chars) != ::core::option::Option::None {
                    return ::core::result::Result::Err(());
                }
                ::core::result::Result::Ok(Self::#var_ident(bits))
            }});
        } else {
            var_decode_str_matches
                .push(quote! { #field_name_str => ::core::result::Result::Ok(Self::#var_ident), });
        }

        // encode
        let enc_out = if i == prop.variants.len() {
            quote!(*bits)
        } else {
            let enc_bits = var.pattern.chars().enumerate().map(|(bit_i, x)| match x {
                '0' => quote! {false},
                '1' => quote! {true},
                'x' => {
                    if var.keep_bits {
                        quote! {bits[#bit_i]}
                    } else {
                        quote! {false}
                    }
                }
                'X' => {
                    if var.keep_bits {
                        quote! {bits[#bit_i]}
                    } else {
                        quote! {true}
                    }
                }
                _ => unreachable!(),
            });

            quote! {[#(#enc_bits),*]}
        };

        if var.keep_bits {
            var_encode_matches.push(quote! { Self::#var_ident(bits) => #enc_out });
        } else {
            var_encode_matches.push(quote! { Self::#var_ident => #enc_out });
        }

        // encode to text
        if var.keep_bits {
            let strbuf_capacity = field_name_str.len() + num_bits + 2;
            var_encode_str_matches.push(quote! { Self::#var_ident(bits) => {
                let mut s = ::#alloc::string::String::with_capacity(#strbuf_capacity);
                ::#alloc::string::String::push_str(&mut s, #field_name_str);
                ::#alloc::string::String::push_str(&mut s, "(");
                for &b in bits {
                    ::#alloc::string::String::push_str(&mut s, if b {"1"} else {"0"});
                }
                ::#alloc::string::String::push_str(&mut s, ")");
                ::#alloc::borrow::Cow::Owned(s)
            }});
        } else {
            var_encode_str_matches.push(
                quote! { Self::#var_ident => ::#alloc::borrow::Cow::Borrowed(#field_name_str) },
            );
        }
    }

    let maybe_impl_default = if let Some(default_idx) = prop.default_variant_idx {
        let default_ident = &prop_var_idents[default_idx];
        let default_array = if prop.variants[default_idx].keep_bits {
            let def_bits = prop.variants[default_idx].pattern.chars().map(|x| match x {
                '0' | 'x' => quote! {false},
                '1' | 'X' => quote! {true},
                _ => unreachable!(),
            });
            quote! {([#(#def_bits),*])}
        } else {
            quote! {}
        };
        quote! {
            impl ::core::default::Default for #prop_name_ident {
                fn default() -> Self {
                    Self::#default_ident #default_array
                }
            }
        }
    } else {
        quote! {}
    };

    let alloc_feature_gate = if let Some(alloc_feature_gate) = settings.alloc_feature_gate.as_ref()
    {
        quote! {#[cfg(feature = #alloc_feature_gate)]}
    } else {
        quote! {}
    };

    let maybe_string_formatter = if settings.emit_string_formatter {
        quote! {
            #alloc_feature_gate
            impl<A: ::bittwiddler_core::prelude::PropertyAccessor> ::bittwiddler_core::prelude::PropertyLeafWithStringConv<[::core::primitive::bool; #num_bits], A> for #prop_name_ident {
                fn to_string(&self, _: &A) -> ::#alloc::borrow::Cow<'static, str> {
                    match self {
                        #(#var_encode_str_matches),*
                    }
                }

                fn from_string(s: &str, _: &A) -> ::core::result::Result<Self, ()> {
                    let (s, bits_s) = ::core::option::Option::unwrap_or(::core::primitive::str::split_once(s, '('), (s, ""));
                    match s {
                        #(#var_decode_str_matches)*
                        _ => ::core::result::Result::Err(()),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum #prop_name_ident {
            #(#prop_variants_decl),*
        }

        impl ::bittwiddler_core::prelude::PropertyLeaf<[::core::primitive::bool; #num_bits]> for #prop_name_ident {
            fn from_bits(bits: &[::core::primitive::bool; #num_bits]) -> Self {
                match bits {
                    #(#var_decode_matches),*
                }
            }

            fn to_bits(&self) -> [::core::primitive::bool; #num_bits] {
                match self {
                    #(#var_encode_matches),*
                }
            }
        }

        #maybe_string_formatter

        #maybe_impl_default
    }
}
