use std::{env, fmt::Display, process::ExitCode};

use bittwiddler_dsl::spreadsheet::{emit_spreadsheet, parse_spreadsheet};

#[derive(Debug)]
enum Error {
    ParseError(parse_spreadsheet::TopError),
    EmitError(emit_spreadsheet::EmitError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => e.fmt(f),
            Error::EmitError(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ParseError(e) => Some(e),
            Error::EmitError(e) => Some(e),
        }
    }
}
impl From<parse_spreadsheet::TopError> for Error {
    fn from(value: parse_spreadsheet::TopError) -> Self {
        Self::ParseError(value)
    }
}
impl From<emit_spreadsheet::EmitError> for Error {
    fn from(value: emit_spreadsheet::EmitError) -> Self {
        Self::EmitError(value)
    }
}

fn main() -> Result<ExitCode, Error> {
    let args = env::args_os().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} spreadsheet.ods", args[0].to_string_lossy());
        return Ok(ExitCode::FAILURE);
    }

    let filename = &args[1];
    let tiles = parse_spreadsheet::parse(filename)?;

    for tile in &tiles {
        let outp_toks = emit_spreadsheet::emit(tile)?;
        println!("{}", outp_toks.to_string());
    }

    Ok(ExitCode::SUCCESS)
}
