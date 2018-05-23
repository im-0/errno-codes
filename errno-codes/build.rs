extern crate phf_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize)]
struct JSON {
    by_id: std::collections::BTreeMap<String, String>,
    by_num: std::collections::BTreeMap<std::os::raw::c_int, String>,
}

fn gen(json_path: &str) {
    use std::io::Write;

    let json = std::fs::read_to_string(json_path).unwrap();
    let json: JSON = serde_json::from_str(&json).unwrap();

    let lslash = json_path.find('/').unwrap();
    let rdot = json_path.rfind('.').unwrap();
    let gen_f_name = &json_path[lslash + 1..rdot];
    let mut gen_f_name = gen_f_name.replace('/', ".");
    gen_f_name.push_str(".rs");

    let gen_f_name = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join(gen_f_name);
    let mut file = std::io::BufWriter::new(std::fs::File::create(&gen_f_name).unwrap());

    // By identifier.
    let mut by_id = phf_codegen::Map::new();
    json.by_id.iter().for_each(|(ident, rust_code)| {
        by_id.entry(ident.as_str(), rust_code.as_str());
    });

    write!(
        &mut file,
        "/// Map string identifier to information about errno code.\n"
    ).unwrap();
    write!(&mut file, "pub static BY_ID: phf::Map<&'static str, ErrnoCode> = ").unwrap();
    by_id.build(&mut file).unwrap();
    write!(&mut file, ";\n\n").unwrap();

    // By number.
    let mut by_num = phf_codegen::Map::new();
    json.by_num.iter().for_each(|(num, rust_code)| {
        by_num.entry(*num, rust_code.as_str());
    });

    write!(&mut file, "/// Map errno number to information about errno code.\n").unwrap();
    write!(
        &mut file,
        "pub static BY_NUM: phf::Map<std::os::raw::c_int, ErrnoCodes> = "
    ).unwrap();
    by_num.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();

    file.flush().unwrap();
}

fn main() {
    for arch in &[
        "alpha",
        "arc",
        "arm",
        "arm64",
        "c6x",
        "h8300",
        "hexagon",
        "ia64",
        "m68k",
        "microblaze",
        "mips",
        "nds32",
        "nios2",
        "openrisc",
        "parisc",
        "powerpc",
        "riscv",
        "s390",
        "sh",
        "sparc",
        "um",
        "unicore32",
        "x86",
        "xtensa",
    ] {
        gen(&format!("src/unix/linux/{}.json", arch));
    }
}
