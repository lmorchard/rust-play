extern crate iterators;

use iterators::Counter;

fn main() {
    example1();
    example2();
    example3();
    example4();
}

fn example1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("v1_iter = {:?}", v1_iter);
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn example2() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total = {}", total);
}

fn example3() {
    let v1: Vec<i32> = vec![1, 2, 3]
        .iter()
        .map(|x| x + 1)
        .collect();
    println!("v1 = {:?}", v1);
}

fn example4() {
    let counter = Counter::new();
    for item in counter {
        println!("item = {}", item);
    }
}
