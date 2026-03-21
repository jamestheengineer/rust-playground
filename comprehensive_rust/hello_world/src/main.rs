// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn multiple(a: &i32, b: &i32) -> &i32 {
    todo!("Return either `a` or `b`")
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    let r = multiple(&a, &b);

    // Which one is still borrowed?
    // Should either mutation be allowed?
    a += 7;
    b += 7;

    dbg!(r);
}