fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}

#[derive(Debug)]
enum IpAddrKind { V4, V6 }

#[derive(Debug)]
struct IpAddrOld {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn example1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four = {:?}, six = {:?}", four, six);
}

fn example2() {
    let home = IpAddrOld {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrOld {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home = {:?}, loopback = {:?}", home, loopback);
}

fn example3() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call! {:?}", self);
    }
}

fn example4() {
    let m = Message::Write(String::from("hello"));
    m.call();

    Message::Quit.call();

    let m2 = Message::Move { x: 100, y: 250 };
    m2.call();

    let m3 = Message::ChangeColor(123, 456, 789);
    m3.call();
}

#[derive(Debug)]
enum UsState {
    //Alabama,
    //Alaska,
    Michigan,
    //Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn example5() {
    println!("penny {}", value_in_cents(Coin::Penny));
    println!("nickel {}", value_in_cents(Coin::Nickel));
    println!("dime {}", value_in_cents(Coin::Dime));
    println!("quarter {}", value_in_cents(Coin::Quarter(UsState::Michigan)));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn example6() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}, six = {:?}, none = {:?}", five, six, none);
}


fn detect_quarter(coin: &Coin, count: &mut u32) {
    if let &Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        *count += 1;
    }
}

fn example7() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        _ => println!("none of the above"),
    }

    if let Some(0) = some_u8_value {
        println!("SOME 0");
    }

    let mut count = 0;
    detect_quarter(&Coin::Quarter(UsState::Michigan), &mut count);
    println!("count {}", count);
    detect_quarter(&Coin::Dime, &mut count);
    println!("count {}", count);
    detect_quarter(&Coin::Nickel, &mut count);
    println!("count {}", count);
}
