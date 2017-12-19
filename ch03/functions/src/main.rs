fn main() {
    println!("Hello, world!");
    example1(5, 6);
    example2();
    example3();
}

fn example1(x: i32, y: i32) {
    println!("Example 1: x = {}; y = {}", x, y);
}

fn example2() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("Example 2: x = {}, y = {}", x, y);
}

fn five() -> i32 {
    5
}

fn example3() {
    let x = five();
    println!("Example 3: x = {}", x);
}
