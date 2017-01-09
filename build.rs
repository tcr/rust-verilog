extern crate libflate;

use std::io;
use std::fs::File;
use std::path::Path;
use std::env;
use libflate::gzip::Decoder;

fn main() {
    let out_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    let mut input = File::open(out_dir.join("src/verilog_parser.rs.gz")).unwrap();

    if let Ok(target) = File::open(out_dir.join("src/verilog_parser.rs")) {
        let m_input = input.metadata().unwrap().modified().unwrap();
        let m_target = target.metadata().unwrap().modified().unwrap();
        if m_input < m_target {
            return;
        }
    }

    println!("cargo:warning=Unpacking updated verilog_parser.rs.gz");
    let mut output = File::create(out_dir.join("src/verilog_parser.rs")).unwrap();
    let mut decoder = Decoder::new(&mut input).unwrap();
    io::copy(&mut decoder, &mut output).unwrap();
}
