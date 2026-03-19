// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn borrows(x: &i32) {
    dbg!(x);
}

fn main() {
    let mut val = 123;

    // Borrow `val` for the function call.
    borrows(&val);

    // Borrow has ended and we're free to mutate.
    val += 5;
}