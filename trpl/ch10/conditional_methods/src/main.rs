use std::fmt::Display;

fn main() {
    let pair1 = Pair::new(69, 96);
    pair1.cmp_display();

    let pair2 = Pair::new(String::from("alpha"), String::from("beta"));
    pair2.cmp_display();

    let pair3 = Pair::new(Blep::new(1), Blep::new(2));

    // This is an error - Blep doesn't implement Display + PartialOrd
    // pair3.cmp_display();

    // But this does work, because Blep members have Frobnicator trait
    pair3.frobnicate();

    // Oh hey, look: I can extend built-in types with a blanket implementation
    "3".lol();
}

trait LolWat {
    fn lol(&self);
}

impl<T: Display> LolWat for T {
    fn lol(&self) {
        println!("LOL WAT {}", self.to_string());
    }
}

trait Frobnicator {
    fn frobnicate(&self);
}

struct Blep {
    value: u32
}

impl Blep {
    fn new(value: u32) -> Blep {
        Blep { value }
    }
}

impl Frobnicator for Blep {
    fn frobnicate(&self) {
        println!("This one has been frobbed: {}", self.value);
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl<T: Frobnicator> Pair<T> {
    fn frobnicate(&self) {
        println!("Now frobnicating all members:");
        self.x.frobnicate();
        self.y.frobnicate();
    }
}
