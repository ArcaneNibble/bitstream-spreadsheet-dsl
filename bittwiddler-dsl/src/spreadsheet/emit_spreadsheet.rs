use std::{error::Error, fmt::Display};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::parse_spreadsheet::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {}
impl Display for EmitError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for EmitError {}

pub fn emit(tile: &Tile) -> Result<TokenStream, EmitError> {
    let tile_name_id = Ident::new(&tile.name, Span::call_site());

    Ok(quote! {
        pub mod #tile_name_id {

        }
    })
}
