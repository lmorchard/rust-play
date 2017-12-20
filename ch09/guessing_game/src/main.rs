extern crate rand;

use std::io;
use std::error::Error;
use std::fmt;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug)]
struct GuessError {
    details: String
}
impl GuessError {
    fn new(msg: &str) -> GuessError {
        GuessError { details: msg.to_string() }
    }
}
impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for GuessError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, GuessError> {
        if value < 1 || value > 100 {
            return Result(GuessError::new(
                format!("Guess value must be between 1 and 100, got {}", value)
            ));
        }

        Result(Guess { value })
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: Guess = match guess.trim().parse() {
      Ok(num)=> Guess::new(num),
      Err(_) => continue,
    };

    let guess = guess.value();

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
