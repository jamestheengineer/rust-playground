fn main() {
    println!("Hello 🌍!");

    println!("result: {}", interproduct(120, 100, 248));

    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
