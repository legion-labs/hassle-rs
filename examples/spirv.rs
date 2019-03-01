use hassle_rs::*;
use rspirv::binary::Disassemble;
use rspirv::mr::load_bytes;

fn main() {
    let source = include_str!("copy.hlsl");

    match compile_hlsl("copy.hlsl", source, "copyCs", "cs_6_0", &["-spirv"], &[]) {
        Ok(spirv) => {
            let module = load_bytes(spirv).unwrap();
            println!("{}", module.disassemble());
        }
        Err(s) => panic!("Failed to compile to SPIR-V: {}", s),
    }
}
