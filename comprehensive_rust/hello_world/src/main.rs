// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    let x = 42;
    let y = x;
    dbg!(x); // would not be accessible if not Copy
    dbg!(y);
}