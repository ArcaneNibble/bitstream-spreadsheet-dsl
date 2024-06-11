use std::{borrow::Borrow, env, fmt::Display, fs, io, process::ExitCode, str::FromStr};

use bittwiddler_dsl::macros::*;
use proc_macro2::{LexError, TokenStream};

#[derive(Debug)]
enum Error {
    IOError(io::Error),
    LexError(LexError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => e.fmt(f),
            Error::LexError(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::LexError(e) => Some(e),
        }
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<LexError> for Error {
    fn from(value: LexError) -> Self {
        Self::LexError(value)
    }
}

fn main() -> Result<ExitCode, Error> {
    let args = env::args_os().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("Usage: {} struct|impl dsl.txt", args[0].to_string_lossy());
        return Ok(ExitCode::FAILURE);
    }
    let filename = &args[2];
    let file_str = fs::read_to_string(filename)?;
    let file_toks = TokenStream::from_str(&file_str)?;

    let outp_toks = match args[1].to_string_lossy().borrow() {
        "struct" => bittwiddler_hierarchy_level(TokenStream::new(), file_toks),
        "impl" => bittwiddler_properties(TokenStream::new(), file_toks),
        _ => {
            println!("Invalid mode {}", args[1].to_string_lossy());
            return Ok(ExitCode::FAILURE);
        }
    };
    println!("{}", outp_toks.to_string());
    Ok(ExitCode::SUCCESS)
}
