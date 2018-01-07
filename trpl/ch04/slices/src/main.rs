fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("word {}, {}", word, first_word(&s[6..]));

    let a = [10, 20, 30, 40, 50];
    println!("array {:?}", &a[2..4]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
