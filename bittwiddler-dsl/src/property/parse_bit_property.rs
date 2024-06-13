use std::error::Error;
use std::fmt::Display;
use std::io::{self, BufRead, BufReader};

use crate::is_valid_ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitProperty {
    pub name: String,
    pub documentation: Option<String>,
    pub variants: Vec<Variant>,
    pub catchall_variant: Option<Variant>,
    pub default_variant_idx: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub pattern: String,
    pub keep_bits: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    NoFirstPropertyName,
    InvalidLine {
        lineno: usize,
        line: String,
    },
    InvalidIdent {
        lineno: usize,
        ident: String,
    },
    InvalidPattern {
        lineno: usize,
        pat: String,
    },
    PatternBitCountMismatch {
        lineno: usize,
        expected_bits: usize,
        current_bits: usize,
    },
    MultipleDefaultVariants {
        lineno: usize,
    },
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NoFirstPropertyName => {
                write!(f, "pattern encountered, but no property name")
            }
            ParseError::InvalidLine { lineno, line } => {
                write!(f, "invalid line \"{}\" on line {}", line, lineno)
            }
            ParseError::InvalidIdent { lineno, ident } => {
                write!(f, "invalid ident \"{}\" on line {}", ident, lineno)
            }
            ParseError::InvalidPattern { lineno, pat } => {
                write!(f, "invalid pattern \"{}\" on line {}", pat, lineno)
            }
            ParseError::PatternBitCountMismatch {
                lineno,
                expected_bits,
                current_bits,
            } => write!(
                f,
                "bit count mismatch on line {}, expected {} got {}",
                lineno, expected_bits, current_bits
            ),
            ParseError::MultipleDefaultVariants { lineno } => {
                write!(f, "multiple variants marked as default on line {}", lineno)
            }
        }
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub enum TopError {
    IOError(io::Error),
    ParseError(ParseError),
}
impl Display for TopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopError::IOError(e) => e.fmt(f),
            TopError::ParseError(e) => e.fmt(f),
        }
    }
}
impl Error for TopError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TopError::IOError(e) => Some(e),
            TopError::ParseError(e) => Some(e),
        }
    }
}
impl From<io::Error> for TopError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<ParseError> for TopError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

const CATCHALL_PATTERN: &'static str = "catchall";

struct ParseVariantNameResult<'a> {
    name: &'a str,
    keep_bits: bool,
    is_default: bool,
}
fn parse_variant_name(var_name: &str) -> ParseVariantNameResult {
    let (var_name, is_default) = match var_name.strip_prefix("*") {
        Some(s) => (s, true),
        None => (var_name, false),
    };

    let (var_name, keep_bits) = match var_name.strip_suffix("()") {
        Some(s) => (s, true),
        None => (var_name, false),
    };

    ParseVariantNameResult {
        name: var_name,
        keep_bits,
        is_default,
    }
}

fn check_pattern(var_pat: &str) -> bool {
    var_pat == CATCHALL_PATTERN
        || !var_pat.contains(|x| match x {
            '0' | '1' | 'x' | 'X' => false,
            _ => true,
        })
}

pub fn parse<R: io::Read>(r: R) -> Result<BitProperty, TopError> {
    let buf_r = BufReader::new(r);
    let mut wip_property: Option<BitProperty> = None;
    let mut lineno = 0;
    let mut pat_bits = None;
    let mut documentation: Option<String> = None;

    for l in buf_r.lines() {
        lineno += 1;
        let l = l?;
        let l = l.trim();
        if l.len() == 0 {
            continue;
        }
        if l.starts_with("#") || l.starts_with("-") {
            continue;
        }

        if let Some(doc_line) = l.strip_prefix("///") {
            if wip_property.is_some() {
                return Err(ParseError::InvalidLine {
                    lineno,
                    line: l.to_owned(),
                }
                .into());
            }

            if let Some(ref mut doc) = documentation {
                doc.push('\n');
                doc.push_str(doc_line.trim());
            } else {
                documentation = Some(doc_line.trim().to_owned());
            }
        } else if let Some((var_pat, var_name_doc)) = l.split_once(&[' ', '\t']) {
            let var_pat = var_pat.trim();
            let var_name_doc = var_name_doc.trim();
            // let var_name = var_name_doc.trim();
            let (var_name, documentation) =
                if let Some((var_name, documentation)) = var_name_doc.split_once(&[' ', '\t']) {
                    (var_name.trim(), Some(documentation.trim().to_owned()))
                } else {
                    (var_name_doc, None)
                };

            if let Some(wip) = wip_property.as_mut() {
                let ParseVariantNameResult {
                    name: var_name,
                    keep_bits,
                    is_default,
                } = parse_variant_name(var_name);
                if !is_valid_ident(var_name) {
                    return Err(ParseError::InvalidIdent {
                        lineno,
                        ident: var_name.to_owned(),
                    }
                    .into());
                }

                if !check_pattern(var_pat) {
                    return Err(ParseError::InvalidPattern {
                        lineno,
                        pat: var_pat.to_owned(),
                    }
                    .into());
                }
                if var_pat != CATCHALL_PATTERN {
                    if let Some(pat_bits) = pat_bits {
                        if pat_bits != var_pat.len() {
                            return Err(ParseError::PatternBitCountMismatch {
                                lineno,
                                expected_bits: pat_bits,
                                current_bits: var_pat.len(),
                            }
                            .into());
                        }
                    } else {
                        pat_bits = Some(var_pat.len());
                    }
                }

                let mut var = Variant {
                    name: var_name.to_owned(),
                    pattern: var_pat.to_owned(),
                    keep_bits,
                    documentation,
                };
                if var_pat == CATCHALL_PATTERN {
                    var.keep_bits = true;
                    wip.catchall_variant = Some(var);
                } else {
                    wip.variants.push(var);
                }

                if is_default {
                    if wip.default_variant_idx.is_some() {
                        return Err(ParseError::MultipleDefaultVariants { lineno }.into());
                    }
                    wip.default_variant_idx = Some(wip.variants.len() - 1);
                }
            } else {
                return Err(ParseError::NoFirstPropertyName.into());
            }
        } else {
            if wip_property.is_some() {
                return Err(ParseError::InvalidLine {
                    lineno,
                    line: l.to_owned(),
                }
                .into());
            }

            if !is_valid_ident(l) {
                return Err(ParseError::InvalidIdent {
                    lineno,
                    ident: l.to_owned(),
                }
                .into());
            }

            wip_property = Some(BitProperty {
                name: l.to_owned(),
                documentation: documentation.take(),
                variants: Vec::new(),
                catchall_variant: None,
                default_variant_idx: None,
            })
        }
    }

    wip_property.ok_or(ParseError::NoFirstPropertyName.into())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_field() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitproperty.txt");
        let f = File::open(p).unwrap();
        let result = parse(f).unwrap();

        assert_eq!(
            result,
            BitProperty {
                name: "Property1".into(),
                documentation: None,
                variants: vec![
                    Variant {
                        name: "ChoiceZero".into(),
                        pattern: "0000".into(),
                        keep_bits: false,
                        documentation: None
                    },
                    Variant {
                        name: "ChoiceOne".into(),
                        pattern: "0001".into(),
                        keep_bits: false,
                        documentation: None
                    },
                    Variant {
                        name: "ChoiceTwo".into(),
                        pattern: "0010".into(),
                        keep_bits: false,
                        documentation: None
                    },
                    Variant {
                        name: "ChoiceThree".into(),
                        pattern: "0011".into(),
                        keep_bits: false,
                        documentation: None
                    },
                    Variant {
                        name: "ChoiceWithX".into(),
                        pattern: "01xX".into(),
                        keep_bits: true,
                        documentation: None
                    }
                ],
                catchall_variant: Some(Variant {
                    name: "CatchallChoice".into(),
                    pattern: "catchall".into(),
                    keep_bits: true,
                    documentation: None
                }),
                default_variant_idx: Some(4)
            },
        );
    }

    #[test]
    fn test_parse_bad_ident() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-badident.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::InvalidIdent {
                    lineno: 2,
                    ident: "Prop1#".to_owned()
                }
            );
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_bad_pattern() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-badpat.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::InvalidPattern {
                    lineno: 2,
                    pat: "0002".to_owned()
                }
            );
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_bad_bit_count_mismatch() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-bitcount-mismatch.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::PatternBitCountMismatch {
                    lineno: 3,
                    expected_bits: 4,
                    current_bits: 3
                }
            );
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_bad_no_name() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-noname.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(e, ParseError::NoFirstPropertyName);
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_bad_invalid_line() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-invalidline.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(
                e,
                ParseError::InvalidLine {
                    lineno: 2,
                    line: "Prop2".into()
                }
            );
        } else {
            panic!("wrong error");
        }
    }

    #[test]
    fn test_parse_bad_multiple_default() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/bitprop-multiple-default.txt");
        let f = File::open(p).unwrap();
        let result = parse(f);

        if let Err(TopError::ParseError(e)) = result {
            assert_eq!(e, ParseError::MultipleDefaultVariants { lineno: 4 });
        } else {
            panic!("wrong error");
        }
    }
}
