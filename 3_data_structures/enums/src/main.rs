enum IpAddrKind {
        V4,
        V6,
    }
use IpAddrKind::{V4, V6};

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let goog = IpAddr::V4(String::from("8.8.8.8"));
    let mwrite = Message::Write(String::from("hello"));
    let mmove = Message::Move { x: 1, y: 2 };
    let mcolor = Message::ChangeColor(255, 0, 0);
    let mquit = Message::Quit;
}