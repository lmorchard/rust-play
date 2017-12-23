use std::fmt::Display;

fn main() {
    example1();
    // example2();
    example3();
    example4();
    example5();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn example1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This does not compile, because lifetimes of string1 & string2 differ
// fn example2() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn example3() {
    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    i.announce_and_return_part("brap");
    println!("ImportantExcerpt part {} {}", i.part, i.level());
}

fn example4() {
    let s: &'static str = "I have a static lifetime, the length of the whole program";
    println!("Hey: {}", s);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn example5() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "Hullabaloo!");
    println!("The longest string is {}", result);
}
