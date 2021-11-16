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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Didn't get the reference? (https://en.wikipedia.org/wiki/42_(number))");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("guess lower :("),
            Ordering::Greater => println!("ack way too high man"),
            Ordering::Equal => {
                println!("Bullseye, you have attained wisdom!");
                break;
            }
        }
    }
}
