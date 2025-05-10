fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let len = s.len();
    println!("{}", len);

    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let len = s.len();

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];

    println!("{}", slice);
}
