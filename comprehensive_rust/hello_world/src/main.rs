// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

struct Key {
    id: u32,
    metadata: Option<String>,
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;
#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}
impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}   

fn main() {
    let a = Citation {author: String::from("Smith"), year: 2020};
    let b = Citation {author: String::from("Nelson"), year: 2021};
    assert!(a < b);
}