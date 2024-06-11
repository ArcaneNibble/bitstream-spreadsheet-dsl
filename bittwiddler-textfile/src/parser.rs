//! This is a very simple implementation with no attempt to optimize string compares

use std::{
    error::Error,
    fmt::Display,
    io::{self, BufRead, BufReader},
};

use bittwiddler_core::prelude::*;

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    message: String,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error on line {}: {}", self.line, self.message)
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub enum TopError {
    ParseError(ParseError),
    IoError(io::Error),
}
impl Display for TopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopError::ParseError(e) => e.fmt(f),
            TopError::IoError(e) => e.fmt(f),
        }
    }
}
impl Error for TopError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TopError::ParseError(e) => Some(e),
            TopError::IoError(e) => Some(e),
        }
    }
}
impl From<ParseError> for TopError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}
impl From<io::Error> for TopError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

fn get_args(line_i: usize, level: &str) -> Result<(&str, Vec<&str>), ParseError> {
    let mut ret = Vec::new();

    if let Some((bare_ident, args)) = level.split_once('[') {
        if let Some(args) = args.strip_suffix(']') {
            for arg in args.split(',') {
                let arg = arg.trim();

                let arg = if let Some((_name, arg)) = arg.split_once('=') {
                    arg.trim()
                } else {
                    arg
                };

                ret.push(arg);
            }
            Ok((bare_ident, ret))
        } else {
            Err(ParseError {
                line: line_i,
                message: "unclosed brackets".into(),
            }
            .into())
        }
    } else {
        Ok((level, ret))
    }
}

pub fn parse<B: BitArray + HumanLevelDynamicAccessor, R: io::Read>(
    r: R,
    bitstream: &mut B,
) -> Result<(), TopError> {
    let r = BufReader::new(r);

    for (line_i, l) in r.lines().enumerate() {
        let l = l?;
        let l = l.trim();

        if l.len() == 0 {
            continue;
        }

        if let Some((property, value)) = l.rsplit_once('=') {
            let mut level: &dyn HumanLevelDynamicAccessor = bitstream;
            let mut boxes = Vec::new();
            let mut property = property.trim();
            let value = value.trim();

            while let Some((this_level, other_levels)) = property.split_once('.') {
                let (this_level_ident, args) = get_args(line_i, this_level)?;

                let idx = level
                    ._human_sublevels()
                    .iter()
                    .position(|x| *x == this_level_ident)
                    .ok_or(ParseError {
                        line: line_i,
                        message: format!("\'{}\' is not valid", this_level_ident),
                    })?;
                let x = level
                    ._human_descend_sublevel(idx, &args)
                    .map_err(|_| ParseError {
                        line: line_i,
                        message: "arg was malformed".into(),
                    })?;
                boxes.push(x);
                level = &*boxes[boxes.len() - 1];
                property = other_levels;
            }

            let (property_ident, args) = get_args(line_i, property)?;
            let idx = level
                ._human_fields()
                .iter()
                .position(|x| *x == property_ident)
                .ok_or(ParseError {
                    line: line_i,
                    message: format!("\'{}\' is not valid", property_ident),
                })?;
            let x = level
                ._human_construct_field(idx, &args)
                .map_err(|_| ParseError {
                    line: line_i,
                    message: "arg was malformed".into(),
                })?;
            x._human_string_set(bitstream, value)
                .map_err(|_| ParseError {
                    line: line_i,
                    message: "value was malformed".into(),
                })?;
        } else {
            return Err(ParseError {
                line: line_i,
                message: "missing '='".into(),
            }
            .into());
        }
    }

    Ok(())
}
