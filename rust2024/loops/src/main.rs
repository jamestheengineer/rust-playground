/// Calculates the nth Fibonacci number iteratively.
///
/// The Fibonacci sequence starts 0, 1, 1, 2, 3, 5, ...
/// F(0) = 0
/// F(1) = 1
/// F(n) = F(n-1) + F(n-2) for n > 1
///
/// # Arguments
///
/// * `n` - The index (0-based) of the Fibonacci number to calculate.
///         Must be non-negative.
///
/// # Returns
///
/// The nth Fibonacci number as a u64.
///
/// # Panics
///
/// This function will panic if the calculated Fibonacci number exceeds
/// the maximum value representable by u64 (which happens around n=94).
/// For larger numbers, you would need a BigInt library.
///
fn fibonacci(n: u32) -> u64 {
    // Handle the base cases F(0) and F(1)
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // Initialize the first two numbers in the sequence
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // Iterate from 2 up to n, calculating each next Fibonacci number
    // The loop runs n-1 times (e.g., for n=2, it runs once; for n=3, twice)
    for _ in 2..=n {
        // Calculate the next number, handling potential overflow
        // Using checked_add would be safer for production code:
        // let next_b = a.checked_add(b).expect("Fibonacci number overflowed u64");
        // For simplicity here, we'll let it panic on overflow in debug builds
        // or wrap around in release builds without explicit checks.
        let next_b = a + b;

        // Update the previous two numbers for the next iteration
        a = b;
        b = next_b;
    }

    // After the loop, 'b' holds the nth Fibonacci number
    b
}

fn main() {
    println!("Calculating Fibonacci numbers:");

    // --- Example 1: The 0th Fibonacci number ---
    let n0 = 0;
    let fib0 = fibonacci(n0);
    println!("Fibonacci({}) = {}", n0, fib0); // Expected: 0

    // --- Example 2: The 1st Fibonacci number ---
    let n1 = 1;
    let fib1 = fibonacci(n1);
    println!("Fibonacci({}) = {}", n1, fib1); // Expected: 1

    // --- Example 3: The 2nd Fibonacci number ---
    let n2 = 2;
    let fib2 = fibonacci(n2);
    println!("Fibonacci({}) = {}", n2, fib2); // Expected: 1

    // --- Example 4: The 10th Fibonacci number ---
    let n10 = 10;
    let fib10 = fibonacci(n10);
    println!("Fibonacci({}) = {}", n10, fib10); // Expected: 55

    // --- Example 5: The 20th Fibonacci number ---
    let n20 = 20;
    let fib20 = fibonacci(n20);
    println!("Fibonacci({}) = {}", n20, fib20); // Expected: 6765

    // --- Example 6: A larger Fibonacci number ---
    // Be aware that u64 will overflow around n=94
    let n45 = 45;
    let fib45 = fibonacci(n45);
    println!("Fibonacci({}) = {}", n45, fib45); // Expected: 1134903170

    // --- Example 7: Using a loop ---
    println!("\nFirst 15 Fibonacci numbers:");
    for i in 0..15 {
        println!("F({}) = {}", i, fibonacci(i));
    }
}