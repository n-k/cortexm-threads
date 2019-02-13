use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // assemble the `asm.s` file
    Build::new().file("asm.s").compile("asm");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=asm.s");

    Ok(())
}
