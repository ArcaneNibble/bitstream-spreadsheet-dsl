use std::{collections::HashMap, error::Error, fmt::Display};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::parse_spreadsheet::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {
    MissingSymMap {
        row: u32,
        col: u32,
        sym: String,
    },
    MissingBit {
        instance_idx: usize,
        missing_bit_idx: usize,
        code_ident: String,
    },
}
impl Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::MissingSymMap { row, col, sym } => {
                write!(f, "missing sym \"{}\" at ({}, {})", sym, row, col)
            }
            EmitError::MissingBit {
                instance_idx,
                missing_bit_idx,
                code_ident,
            } => write!(
                f,
                "missing bit {} for instance {} of {}",
                missing_bit_idx, instance_idx, code_ident
            ),
        }
    }
}
impl Error for EmitError {}

pub fn emit(tile: &Tile) -> Result<TokenStream, EmitError> {
    let tile_name_id = Ident::new(&tile.name, Span::call_site());

    // loop over everything looking for max numbers
    #[derive(Debug)]
    struct PropertySizeInfo {
        num_bits: usize,
        num_instances: usize,
    }
    let mut property_info: HashMap<&String, PropertySizeInfo> = HashMap::new();
    for row in 0..tile.grid.len() {
        for col in 0..tile.grid[row].len() {
            if let Some(bit) = &tile.grid[row][col] {
                let prop_code_ident = tile.spreadsheet_sym_map.get(&bit.spreadsheet_sym).ok_or(
                    EmitError::MissingSymMap {
                        row: row as u32,
                        col: col as u32,
                        sym: bit.spreadsheet_sym.clone(),
                    },
                )?;

                property_info
                    .entry(prop_code_ident)
                    .and_modify(|x| {
                        x.num_bits = usize::max(x.num_bits, bit.bit_idx + 1);
                        x.num_instances =
                            usize::max(x.num_instances, bit.instance_address.unwrap_or(0) + 1);
                    })
                    .or_insert(PropertySizeInfo {
                        num_bits: bit.bit_idx + 1,
                        num_instances: bit.instance_address.unwrap_or(0) + 1,
                    });
            }
        }
    }

    // loop over everything again, writing down the bit positions
    let mut property_coords = HashMap::new();
    for (prop_code_ident, prop_sz_info) in property_info.into_iter() {
        property_coords.insert(
            prop_code_ident,
            vec![vec![None; prop_sz_info.num_bits]; prop_sz_info.num_instances],
        );
    }
    for row in 0..tile.grid.len() {
        for col in 0..tile.grid[row].len() {
            if let Some(bit) = &tile.grid[row][col] {
                let prop_code_ident = tile.spreadsheet_sym_map.get(&bit.spreadsheet_sym).unwrap();
                property_coords.entry(prop_code_ident).and_modify(|x| {
                    x[bit.instance_address.unwrap_or(0)][bit.bit_idx] = Some((row, col));
                });
            }
        }
    }

    // emit the code
    let output_coords_code = property_coords
        .into_iter()
        .map(|(prop_code_ident, coords)| {
            let const_ident = Ident::new(&prop_code_ident, Span::call_site());

            let coords_for_each_instance = coords.iter().enumerate().map(|(instance_idx, inst_coords)| {
                let coords = inst_coords.into_iter().enumerate().map(|(bit_idx, c)| {
                    if c.is_none() {
                        return Err(EmitError::MissingBit { instance_idx, missing_bit_idx: bit_idx, code_ident: prop_code_ident.clone() })
                    }
                    let (row, col) = c.unwrap();
                    Ok(quote! {
                        ::bittwiddler_core::prelude::Coordinate::new(#col, #row)
                    })
                }).collect::<Result<Vec<_>, _>>();

                if let Err(e) = coords {
                    return Err(e);
                }
                let coords = coords.unwrap();

                Ok(quote!{&[#(#coords),*]})
            }).collect::<Result<Vec<_>, _>>();

            if let Err(e) = coords_for_each_instance {
                return Err(e);
            }
            let coords_for_each_instance = coords_for_each_instance.unwrap();

            if coords.len() == 1 {
                Ok(quote! {
                    pub const #const_ident: &'static [::bittwiddler_core::prelude::Coordinate] = #(#coords_for_each_instance)*;
                })
            } else {
                Ok(quote! {
                    pub const #const_ident: &'static [&'static [::bittwiddler_core::prelude::Coordinate]] = &[#(#coords_for_each_instance),*];
                })
            }
        }).collect::<Result<Vec<_>, _>>()?;

    let width = tile.grid[0].len();
    let height = tile.grid.len();

    Ok(quote! {
        pub mod #tile_name_id {
            pub const W: ::core::primitive::usize = #width;
            pub const H: ::core::primitive::usize = #height;

            #(#output_coords_code)*
        }
    })
}
