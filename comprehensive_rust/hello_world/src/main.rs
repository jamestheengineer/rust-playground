// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

#[deny(clippy::cast_possible_truncation)]
fn main() {
    let mut x = 3;
    while (x < 70000) {
        x *= 2;
    }
    println!("X probably fits in a u16, right? {}", x as u16);
}