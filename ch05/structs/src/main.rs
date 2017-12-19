struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    example1();
    example2();
    example3();
    example4();
}

fn example1() {
    let mut user1 = User {
        email: String::from("butts@lol.org"),
        username: String::from("butts"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("lolbutts@lol.org");
    println!("user1 email = {}", user1.email);
}

fn example2() {
    let user2 = build_user(String::from("foo@bar.com"), String::from("foo"));
    println!("user2 email = {}", user2.email);

    let user3 = User {
        email: String::from("baz@xyzzy.org"),
        username: String::from("baz"),
        ..user2
    };
    println!("user3 email = {}", user3.email);
}

fn example3() {
    let black = Color(0, 0, 0);
    println!("black 1 = {}", black.1);

    let origin = Point(0, 0, 0);
    println!("origin 1 = {}", origin.1);
}

fn example4() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect {:#?}", rect1);
    println!("rect {:?}", rect1);
    println!("area {}", area(&rect1));
}

fn build_user(email: String, username: String) -> User {
    User {
        email, username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
