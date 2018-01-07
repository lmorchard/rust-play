fn main() {
    example1();
    example2();
    example3();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn example1() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn example2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        // .map(|i| i.to_string())
        .map(ToString::to_string)
        .collect();

    println!("Example2 {:?}", list_of_strings);
}

fn return_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn example3() {
    let closure = return_closure();
    let result = closure(2);
    println!("Result {}", result);
}
