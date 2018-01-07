use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // example1();
    example2();
}

fn example1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };
    println!("File {:?}", f);
}

fn example2() {
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("farb/hello.txt").expect("dun goofed on hello.txt");
    println!("File {:?}", f);
}
