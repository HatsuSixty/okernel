use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/c_o1heap/o1heap.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("--target=i686-unknown-none")
        .clang_arg("-ffreestanding")
        .use_core()
        .header("src/c_o1heap/o1heap.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("o1heap.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=src/c_o1heap/o1heap.c");
    cc::Build::new()
        .compiler("i686-elf-gcc")
        .std("c11")
        .opt_level(3)
        .flag("-ffreestanding")
        .flag("-nostdlib")
        .file("src/c_o1heap/o1heap.c")
        .include("src/c_o1heap")
        .compile("o1heap");
}
