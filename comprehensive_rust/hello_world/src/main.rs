// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

use std::thread;

fn foo() {
    let s = String::from("Hello");
    thread::scope(|scope| {
        scope.spawn(|| {
            dbg!(s.len());
        });
    });
}

fn main() {
    foo();
}