fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}

fn example1(){
    let number = 7;
    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }
}

fn example2() {
    let number = 5;
    if number != 0 {
        println!("Condition was true!");
    }
}

fn example3() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn example4() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Example 4: number = {}", number);
}

fn example5() {
    //loop {
    //    println!("AGAIN");
    //}

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn example6() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element = {}", element);
    }
}

fn example7() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
