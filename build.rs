// Author: Steven Dee
// This file is released into the public domain.

fn main() {
    cc::Build::new()
        .file("csrc/readpassphrase.c")
        .include("csrc")
        .compile("readpassphrase");
    println!("cargo:rerun-if-changed=csrc/readpassphrase.c");
    println!("cargo:rerun-if-changed=csrc/readpassphrase.h");
}
