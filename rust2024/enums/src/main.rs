fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four is {four:#?}");
    println!("six is {six:#?}");
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}