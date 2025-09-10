// Author: Steven Dee
// This file is released into the public domain.

use std::ffi::{c_char, c_int};

extern "C" {
    pub fn readpassphrase(
        prompt: *const c_char,
        buf: *mut c_char,
        bufsiz: usize,
        flags: c_int,
    ) -> *mut c_char;
}
