enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
impl Message {
    fn some_function() {
        println!("Dardonacci")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // Using just the enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Using the enum inside a struct
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    // Using an enum with data stored in the enum
    let localhost = IpAddrKind2::V4(127, 0, 0, 1);

    // Option Enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);

    // Match expression
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Match expression + Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Using if let syntax
    // It's like the match expression but it is only evaluating the pattern that you care about
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => ()
    }

    if let Some(3) = some_value {
        println!("Three")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}