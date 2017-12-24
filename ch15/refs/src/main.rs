use std::ops::Deref;

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
}

fn example1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {:?}, y = {:?}", x, y);
}

fn example2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {:?}, y = {:?}", x, y);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn example3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {:?}, y = {:?}", x, y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn example4() {
    let m = MyBox::new(String::from("Rust"));
    // Without dereference coercion
    hello(&(*m)[..]);
    // With dereference coercion
    hello(&m);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn example5() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    let e = CustomSmartPointer { data: String::from("drop early") };
    println!("CustomSmartPointers created. c = {:?}, d = {:?}, e = {:?}", c, d, e);
    drop(e);
    println!("Almost done...");
    // println!("Error caused here, because e moved into drop()! e = {:?}", e);
}
