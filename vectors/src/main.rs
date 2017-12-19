fn main() {
    example1();
    example1_5();
    example2();
    example3();
    example4();
    example5();
}

fn example1() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v = {:?}", v);
}

fn example1_5() {
    // look ma no type!
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v = {:?}", v);
}

fn example2() {
    let v = vec![6, 7, 8, 9, 0];
    println!("v = {:?}", v);
}

fn example3() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let third1: &i32 = &v[2];
    let third2: Option<&i32> = v.get(2);
    let third3: &i32 = if let Some(num) = v.get(99) { num } else { &-1 };
    let third4: &i32 = match v.get(99) {
        Some(num) => num,
        None => &-1
    };
    println!("third1={:?}, third2={:?}, third3={:?}, third4={:?}",
             third1, third2, third3, third4);
}

fn example4() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

fn example5() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("frotz")),
        SpreadsheetCell::Float(123.321)
    ];
    println!("{:?}", row);
}
