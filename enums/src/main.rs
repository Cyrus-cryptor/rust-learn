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
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //if self == Message::Write(())
    }
}


enum usState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel.
    Dieme,
    Quarter(usState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //Coin::Penny => 1,
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dieme => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {state} ")
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => some(i + 1),
    }
}

// struct QuitMessage;

// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String);

// struct ChangeColorMessage(i32,i32,i32)

