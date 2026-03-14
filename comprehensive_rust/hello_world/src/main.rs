// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for elem in &vec {
        vec.push(elem * 2);
    }
}