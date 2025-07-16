use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let absent_num: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);
    println!("Sum: {}", sum);
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //if self == Message::Write(())
    }
}
