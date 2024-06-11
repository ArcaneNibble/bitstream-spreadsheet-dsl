//! Proc macros to be used on field accessor related structs

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, ItemStruct};

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
            fn _should_add_piece(&self) -> bool {
                false
            }
            fn to_human_string(&self) -> ::bittwiddler_core::prelude::CowReexport<'static, ::core::primitive::str> {
                ::core::convert::Into::into("")
            }
            fn from_human_string(s: &str) -> ::core::result::Result<Self, ()>
            where
                Self: Sized
            {
                Err(())
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
