// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    dbg!(a);
    dbg!(b);
}