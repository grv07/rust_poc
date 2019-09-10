#[derive(Debug)]
enum IpAddrKind {
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

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => {
            println!("Penny here ...");
            2
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let localhost = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", localhost);
    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", add_one(Option::Some(68)));
}
