use std::fmt::Display;
use std::path::Path;
use std::{collections::HashMap, error::Error};

use calamine::{open_workbook, Data, DataType, Ods, OdsError, Reader};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileBit {
    pub spreadsheet_sym: String,
    pub instance_address: Option<usize>,
    pub bit_idx: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub name: String,
    pub grid: Vec<Vec<Option<TileBit>>>,
    pub spreadsheet_sym_map: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedNone,
    InvalidCellContentType {
        row: u32,
        col: u32,
    },
    MalformedCellContents {
        row: u32,
        col: u32,
        contents: String,
    },
    InvalidSymMapCell {
        row: u32,
    },
    DuplicateSymMapSym {
        row: u32,
        ident: String,
    },
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedNone => {
                write!(f, "unexpectedly got a None while parsing spreadsheet")
            }
            ParseError::InvalidCellContentType { row, col } => write!(
                f,
                "invalid cell content type at ({}, {}) (must be string or empty)",
                row, col
            ),
            ParseError::MalformedCellContents { row, col, contents } => write!(
                f,
                "malformed cell contents \"{}\" at ({}, {})",
                contents, row, col
            ),
            ParseError::InvalidSymMapCell { row } => {
                write!(f, "invalid spreadsheet symbol map entry on row {}", row)
            }
            ParseError::DuplicateSymMapSym { row, ident } => {
                write!(
                    f,
                    "duplicate spreadsheet symbol \"{}\" on row {}",
                    ident, row
                )
            }
        }
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub enum TopError {
    OdsError(OdsError),
    ParseError(ParseError),
}
impl Display for TopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopError::OdsError(e) => e.fmt(f),
            TopError::ParseError(e) => e.fmt(f),
        }
    }
}
impl Error for TopError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TopError::OdsError(e) => Some(e),
            TopError::ParseError(e) => Some(e),
        }
    }
}
impl From<OdsError> for TopError {
    fn from(value: OdsError) -> Self {
        Self::OdsError(value)
    }
}
impl From<ParseError> for TopError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

fn parse_cell(
    row: u32,
    col: u32,
    cell: &Data,
    cell_parse_re: &Regex,
) -> Result<Option<TileBit>, ParseError> {
    match cell {
        Data::String(s) => {
            let s = s.trim();
            if let Some(captures) = cell_parse_re.captures_at(s, 0) {
                let spreadsheet_sym = captures
                    .name("spreadsheet_sym")
                    .unwrap()
                    .as_str()
                    .to_owned();
                let instance_address = captures
                    .name("instance_address")
                    .map(|x| x.as_str().parse::<usize>().unwrap());
                let bit_idx = captures
                    .name("bit_idx")
                    .map(|x| x.as_str().parse::<usize>().unwrap());
                Ok(Some(TileBit {
                    spreadsheet_sym,
                    instance_address,
                    bit_idx: bit_idx.unwrap_or(0),
                }))
            } else {
                Err(ParseError::MalformedCellContents {
                    row,
                    col,
                    contents: s.to_owned(),
                })
            }
        }
        Data::Empty => Ok(None),
        _ => Err(ParseError::InvalidCellContentType { row, col }),
    }
}

fn cell_to_string(cell: Option<&Data>) -> Option<String> {
    if let Some(cell) = cell {
        if let Data::String(s) = cell {
            Some(s.to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn parse<P: AsRef<Path>>(p: P) -> Result<Vec<Tile>, TopError> {
    let cell_parse_re =
        Regex::new("^(?<spreadsheet_sym>[^\\[\\]]+)(?:\\[(?<instance_address>[0-9+]+)\\])??(?:\\[(?<bit_idx>[0-9]+)\\])?$")
            .unwrap();
    let mut spreadsheet: Ods<_> = open_workbook(p)?;
    let mut tiles = Vec::new();

    for (sheet, data) in spreadsheet.worksheets() {
        let (end_row, end_col) = data.end().ok_or(ParseError::UnexpectedNone)?;
        let mut start_row_of_map = 0;

        let mut tile_data = Vec::new();
        for row in 0..=end_row {
            let mut this_row = Vec::new();
            let mut all_empty = true;
            for col in 0..=end_col {
                let cell_data = data
                    .get_value((row, col))
                    .ok_or(ParseError::UnexpectedNone)?;

                if !cell_data.is_empty() {
                    all_empty = false;
                }
                if let Some(s) = cell_data.as_string() {
                    if s == "XXX" {
                        break;
                    }
                }

                let cell_data = parse_cell(row, col, cell_data, &cell_parse_re)?;
                this_row.push(cell_data);
            }

            if all_empty {
                start_row_of_map = row + 1;
                break;
            }

            tile_data.push(this_row);
        }

        let mut spreadsheet_sym_map = HashMap::new();
        for row in start_row_of_map..=end_row {
            let spreadsheet_sym = cell_to_string(data.get_value((row, 0)));
            let code_name = cell_to_string(data.get_value((row, 1)));
            if let (Some(spreadsheet_sym), Some(code_name)) = (spreadsheet_sym, code_name) {
                let ret = spreadsheet_sym_map.insert(spreadsheet_sym.clone(), code_name.clone());

                if ret.is_some() {
                    return Err(ParseError::DuplicateSymMapSym {
                        row,
                        ident: spreadsheet_sym,
                    }
                    .into());
                }
            } else {
                return Err(ParseError::InvalidSymMapCell { row }.into());
            }
        }

        tiles.push(Tile {
            name: sheet,
            grid: tile_data,
            spreadsheet_sym_map,
        })
    }

    Ok(tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_tile() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/testtile.ods");
        let result = parse(p).unwrap();

        assert_eq!(result.len(), 1);

        let tile = &result[0];
        assert_eq!(tile.name, "test_tile");
        assert_eq!(tile.spreadsheet_sym_map.len(), 5);

        let p1_map = tile.spreadsheet_sym_map.get("P1").unwrap();
        assert_eq!(p1_map, "PROPERTY_ONE");

        let p2_map = tile.spreadsheet_sym_map.get("P2").unwrap();
        assert_eq!(p2_map, "PROPERTY_TWO");

        let p3_map = tile.spreadsheet_sym_map.get("P3").unwrap();
        assert_eq!(p3_map, "PROPERTY_THREE");

        let p4_map = tile.spreadsheet_sym_map.get("P4").unwrap();
        assert_eq!(p4_map, "PROPERTY_FOUR");

        let p5_map = tile.spreadsheet_sym_map.get("P5").unwrap();
        assert_eq!(p5_map, "PROPERTY_FIVE");

        let grid = &tile.grid;
        assert_eq!(
            grid,
            &vec![
                vec![
                    Some(TileBit {
                        spreadsheet_sym: "P1".into(),
                        instance_address: None,
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P1".into(),
                        instance_address: None,
                        bit_idx: 1,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P1".into(),
                        instance_address: None,
                        bit_idx: 2,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P1".into(),
                        instance_address: None,
                        bit_idx: 3,
                    }),
                ],
                vec![
                    Some(TileBit {
                        spreadsheet_sym: "P5".into(),
                        instance_address: None,
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P5".into(),
                        instance_address: None,
                        bit_idx: 1,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P5".into(),
                        instance_address: None,
                        bit_idx: 2,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P5".into(),
                        instance_address: None,
                        bit_idx: 3,
                    }),
                ],
                vec![
                    Some(TileBit {
                        spreadsheet_sym: "P2".into(),
                        instance_address: Some(0),
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P2".into(),
                        instance_address: Some(1),
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P2".into(),
                        instance_address: Some(2),
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P2".into(),
                        instance_address: Some(3),
                        bit_idx: 0,
                    }),
                ],
                vec![
                    Some(TileBit {
                        spreadsheet_sym: "P3".into(),
                        instance_address: None,
                        bit_idx: 0,
                    }),
                    Some(TileBit {
                        spreadsheet_sym: "P4".into(),
                        instance_address: None,
                        bit_idx: 0,
                    }),
                    None,
                    None,
                ],
            ]
        );
    }

    #[test]
    fn test_parse_tile_bad_cell() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/testtile-badcell.ods");
        let result = parse(p);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(e, ParseError::InvalidCellContentType { row: 2, col: 0 });
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_tile_malformed_cell() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/testtile-malformedcell.ods");
        let result = parse(p);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::MalformedCellContents {
                    row: 2,
                    col: 0,
                    contents: "P2[0][0".to_owned()
                }
            );
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_tile_invalid_sym_map() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/testtile-invalidsymmap.ods");
        let result = parse(p);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(e, ParseError::InvalidSymMapCell { row: 5 });
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_tile_dup_sym_map() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/testtile-dupsymmap.ods");
        let result = parse(p);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::DuplicateSymMapSym {
                    row: 6,
                    ident: "P1".to_owned()
                }
            );
        } else {
            panic!("wrong error");
        }
    }
}
