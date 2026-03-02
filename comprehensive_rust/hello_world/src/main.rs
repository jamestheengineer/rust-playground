// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}