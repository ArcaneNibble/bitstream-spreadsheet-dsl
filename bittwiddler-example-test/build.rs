use bittwiddler_dsl::property::{emit_bit_property, parse_bit_property};
use bittwiddler_dsl::spreadsheet::{emit_spreadsheet, parse_spreadsheet};
use std::io::Write;
use std::{env, fs::File, path::PathBuf};

fn main() {
    {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("../bittwiddler-dsl/tests/bitproperty.txt");
        let f = File::open(p).unwrap();
        let result = parse_bit_property::parse(f).unwrap();
        let mut settings = emit_bit_property::Settings::default();
        settings.enable_no_std = true;
        settings.alloc_feature_gate = Some("alloc".to_string());
        let result_ts = emit_bit_property::emit(&result, &settings);

        let mut p = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        p.push("bitproperty-out.rs");
        let mut f = File::create(p).unwrap();
        write!(f, "{}", result_ts).unwrap();
        println!("cargo:rerun-if-changed=bitproperty-out.rs");
    }
    {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("../bittwiddler-dsl/tests/testtile.ods");
        let tiles = parse_spreadsheet::parse(p).unwrap();

        let mut p = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        p.push("tiles-out.rs");
        let mut f = File::create(p).unwrap();
        for tile in &tiles {
            let outp_toks = emit_spreadsheet::emit(tile).unwrap();
            write!(f, "{}", outp_toks).unwrap();
        }
        println!("cargo:rerun-if-changed=tiles-out.rs");
    }
}
