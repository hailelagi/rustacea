use std::time::Duration;
use std:: thread;

#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

// closures can capture values from surrounding scope

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| {self.most_stocked()})
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_prefer_one = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_prefer_one);

    println!("The user with pref {:?} gets {:?}", user_prefer_one, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with pref {:?} gets {:?}", user_pref2, giveaway2);

    let mut x = vec![1, 2, 3, 4, 5];

    for i in x.iter().map(|j| j * 3 ) {
        if i == 6 {
            print!("{}", i)
        }
    }
    /*
    let expensive_closure = |num: u32| -> u32 {
        println!("sleepy sleep, tired..");
        thread::sleep(Duration::from_secs(4));
        num
    };
    */

    // print!("{:?}", expensive_closure(5));
}

// Iterators - lazy
// Iterator[trait].iter()
// iterator consumers (next ...) and adapters (map, filter, fold)

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

// into_iter()  T, &T & mut T
// iter() => &T
// iter_mut() => & mut T

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
