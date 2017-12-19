fn main() {
    example1();
    example2();
    example3();
    example4();
}

fn example1() {
    let data = "initial contents";
    let s1 = data.to_string();

    let s2 = "another string".to_string();

    let s3 = String::from("one more string");

    let hello = String::from("こんにちは");

    let mut s4 = String::new();
    let s4_sub = "foo";
    s4.push_str(&s4_sub);
    s4.push('b');
    s4.push('a');
    s4.push('r');

    println!("s1={:?}, s2={:?}, s3={:?}, hello={:?}, s4={:?}",
             s1, s2, s3, hello, s4);
}

fn example2() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2; // Note that s1 has been moved and can no longer be used
    println!("s1={:?}", s1);
}

fn example3() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s={:?}", s);
}

fn example4() {
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!("");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("");
}
