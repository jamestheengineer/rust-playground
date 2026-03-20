// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn identity(x: &i32) -> &i32 {
    x
}

fn main() {
    let mut x = 123;

    let out = identity(&x);

    // x = 5; // 🛠️❌ `x` is still borrowed!

    dbg!(out);
}