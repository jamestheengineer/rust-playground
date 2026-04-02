// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    let primes = vec![2, 3, 5, 7];
    println!("primes: {primes:?}");
    let prime_squares: Vec<_> = primes.into_iter().map(|p| p * p).collect();
    println!("prime_squares: {prime_squares:?}");
}