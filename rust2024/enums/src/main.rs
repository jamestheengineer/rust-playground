fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four is {four:#?}");
    println!("six is {six:#?}");

    let home = IpAddrBetter::V4(String::from("127.0.0.1"));

    let loopback = IpAddrBetter::V6(String::from("::1"));

    println!("home is {home:#?}");
    println!("loopback is {loopback:#?}");
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrBetter {
    V4(String),
    V6(String),
}
