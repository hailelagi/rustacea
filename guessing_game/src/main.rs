use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(" -- GUESS THE NUMBER --");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("The meaning of life is?: \n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => Guess(num),
            Err(_) => {
                println!("Didn't get the reference? (https://en.wikipedia.org/wiki/42_(number))");
                continue;
            }
        };

        match guess.value().cmp(&secret_num) {
            Ordering::Less => println!("guess too low, go up! :("),
            Ordering::Greater => println!("ack way too high man"),
            Ordering::Equal => {
                println!("Bullseye, you have attained wisdom!");
                break;
            }
        }
    }
}

pub struct Guess(i32);

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess(value)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
