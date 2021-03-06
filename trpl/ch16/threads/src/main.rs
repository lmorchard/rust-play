use std::thread;

fn main() {
    example1();
    example2();
}

fn example1() {
    // thread::spawn(|| {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    handle.join();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
