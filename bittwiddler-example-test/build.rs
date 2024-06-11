use bittwiddler_dsl::property::{emit_bit_property, parse_bit_property};
use std::io::Write;
use std::{env, fs::File, path::PathBuf};

fn main() {
    {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("../bittwiddler-dsl/tests/bitproperty.txt");
        let f = File::open(p).unwrap();
        let result = parse_bit_property::parse(f).unwrap();
        let mut settings = emit_bit_property::Settings::default();
        settings.enable_no_std = false;
        let result_ts = emit_bit_property::emit(&result, &settings);

        let mut p = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        p.push("bitproperty-out.rs");
        let mut f = File::create(p).unwrap();
        write!(f, "{}", result_ts).unwrap();
        println!("cargo:rerun-if-changed=bitproperty-out.rs");
    }
}
