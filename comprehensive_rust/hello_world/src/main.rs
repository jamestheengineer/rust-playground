// Copyright 2025 Google LLC
// SPDX-License-Identifier: Apache-2.0

use std::cell::Cell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);

    cell.set(123);
    dbg!(cell.get());
}