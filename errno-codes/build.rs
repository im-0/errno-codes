extern crate phf_codegen;

macro_rules! gen_linux {
    ($arch:expr) => {{
        use std::io::Write;

        let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join(concat!("linux-", $arch, ".rs"));
        let mut file = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());

        // By identifier.
        write!(
            &mut file,
            "/// Map string identifier to information about errno code.\n"
        ).unwrap();
        write!(&mut file, "pub static BY_ID: phf::Map<&'static str, ErrnoCode> = ").unwrap();
        include!(concat!("src/unix/linux/", $arch, "-codegen-by-id.rs"))
            .build(&mut file)
            .unwrap();
        write!(&mut file, ";\n\n").unwrap();

        // By number.
        write!(&mut file, "/// Map errno number to information about errno code.\n").unwrap();
        write!(
            &mut file,
            "pub static BY_NUM: phf::Map<std::os::raw::c_int, ErrnoCodes> = "
        ).unwrap();
        include!(concat!("src/unix/linux/", $arch, "-codegen-by-num.rs"))
            .build(&mut file)
            .unwrap();
        write!(&mut file, ";\n").unwrap();
    }};
}

fn main() {
    gen_linux!("alpha");
    gen_linux!("arc");
    gen_linux!("arm");
    gen_linux!("arm64");
    gen_linux!("c6x");
    gen_linux!("h8300");
    gen_linux!("hexagon");
    gen_linux!("ia64");
    gen_linux!("m68k");
    gen_linux!("microblaze");
    gen_linux!("mips");
    gen_linux!("nds32");
    gen_linux!("nios2");
    gen_linux!("openrisc");
    gen_linux!("parisc");
    gen_linux!("powerpc");
    gen_linux!("riscv");
    gen_linux!("s390");
    gen_linux!("sh");
    gen_linux!("sparc");
    gen_linux!("um");
    gen_linux!("unicore32");
    gen_linux!("x86");
    gen_linux!("xtensa");
}
