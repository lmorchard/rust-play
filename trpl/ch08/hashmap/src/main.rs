use std::collections::HashMap;

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
}

fn example1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}

fn example2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

fn example3() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map={:?}", map);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //println!("field_name={}, field_value={}", field_name, field_value);
 }

fn example4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    {
        // Wrap this in a block so we get an immutable borrow but can still mutate later
        // This is probably a terrible idea
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        println!("score={:?}, none?={:?}",
                 score, scores.get("Red"));
    }

    scores.insert(String::from("Red"), 35);
    scores.insert(String::from("Yellow"), 99);

    for (key, value) in &scores {
        print!("{}: {}; ", key, value);
    }
    println!("");
}

fn example5() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn example6() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
