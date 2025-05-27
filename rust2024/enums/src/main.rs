fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four is {four:#?}");
    println!("six is {six:#?}");

    let home = IpAddrBetter::V4(String::from("127.0.0.1"));

    let loopback = IpAddrBetter::V6(String::from("::1"));

    println!("home is {home:#?}");
    println!("loopback is {loopback:#?}");

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
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

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("home is {self:#?}");
    }
}