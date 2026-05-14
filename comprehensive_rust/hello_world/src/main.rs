// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

use std::ffi::c_char;

unsafe extern "C" {
    // `abs` doesn't deal with pointers and doesn't have any safety requirements.
    safe fn abs(input: i32) -> i32;

    /// # Safety
    ///
    /// `s` must be a pointer to a NUL-terminated C string which is valid and
    /// not modified for the duration of this function call.
    unsafe fn strlen(s: *const c_char) -> usize;
}

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        // SAFETY: We pass a pointer to a C string literal which is valid for
        // the duration of the program.
        println!("String length: {}", strlen(c"String".as_ptr()));
    }
}