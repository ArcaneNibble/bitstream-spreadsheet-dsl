use std::{env, fmt::Display, fs::File, io, process::ExitCode};

use bittwiddler_dsl::property::{
    emit_bit_property::{self, Settings},
    parse_bit_property::{self, ParseError, TopError},
};

#[derive(Debug)]
enum Error {
    IOError(io::Error),
    ParseError(ParseError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => e.fmt(f),
            Error::ParseError(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::ParseError(e) => Some(e),
        }
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<TopError> for Error {
    fn from(value: TopError) -> Self {
        match value {
            TopError::IOError(e) => Self::IOError(e),
            TopError::ParseError(e) => Self::ParseError(e),
        }
    }
}

fn main() -> Result<ExitCode, Error> {
    let args = env::args_os().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} bitprop.txt", args[0].to_string_lossy());
        return Ok(ExitCode::FAILURE);
    }
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let bitprop_parsed = parse_bit_property::parse(f)?;
    let bitprop_ts = emit_bit_property::emit(&bitprop_parsed, &Settings::default());
    println!("{}", bitprop_ts.to_string());
    Ok(ExitCode::SUCCESS)
}
