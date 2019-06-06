use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

extern crate cc;

fn main() {
    // Compile interrupt handlers
    cc::Build::new()
        .file("src/interrupthandlers.c")
        .compile("interrupthandlers.a");

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("device.x"))
        .unwrap()
        .write_all(include_bytes!("device.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}
