extern crate rand;

use std::io;
use std::error::Error;
// use std::fmt;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("Guess value must be between 1 and 100, got {}", value));
        }
        Ok(Guess { value })
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn get_guess() -> Result<u32, Box<Error>> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    Ok(Guess::new(guess.trim().parse()?)?.value)
}

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess");

    let guess: u32 = match get_guess() {
        Ok(num) => num,
        Err(err) => {
            println!("Guess invalid: {}", err);
            continue;
        },
    };

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
