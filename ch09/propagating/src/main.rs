use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    example1();
}

fn example1() {
    let username = match read_username_from_file_shortest() {
        Ok(s) => s.trim().to_string(),
        Err(e) => panic!("So much for that username {}", e),
    };
    println!("Hello, {:?}", username);
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}
