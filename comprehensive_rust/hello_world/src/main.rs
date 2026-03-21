// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32 {
    if c { a } else { b }
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    let r = pick(true, &a, &b);

    // Which one is still borrowed?
    // Should either mutation be allowed?
    // a += 7;
    // b += 7;

    dbg!(r);
}