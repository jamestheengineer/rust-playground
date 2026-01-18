fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Ignore everything but the last element.
    let (.., c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn main() {
    takes_tuple(('a', 777, true));
} 