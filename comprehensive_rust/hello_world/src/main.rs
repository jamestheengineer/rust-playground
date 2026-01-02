/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        n = collatz_step(n);
        length += 1;
    }
    length
}
fn collatz_step(n: i32) -> i32 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}