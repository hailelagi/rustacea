use std::fmt::Display;

fn main() {
    // println!("Hello, world!");

    // let nums = vec![1, 29, 42, 4, 5];
    // let mut largest = nums[0];

    // for n in nums {
    //     if n > largest {
    //         largest = n;
    //     }
    // }

    // println!("{}", largest)
    let l: Vec<i32> = vec![1, 35, 0, 5, 52, 2525, 32];
    let x = vec![1, 3, 8, 2, 2];

    println!("{}", largest(&l));
    println!("{}", largest(&x))
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// Generic Data Types
struct Point<T> {
    x: T,
    y: f32,
}

struct PrecisePoint<T, U> {
    real: T,
    imaginary: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> PrecisePoint<T, U> {
    fn x() -> i32 {
        return 5;
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Stuff<T> {
    Some(T),
    None,
}
// largest is generic over T
fn large<T>(list: &[T]) -> T {
    let p = Point { x: 1, y: 2.0 };

    let mut largest = list[0];

    for &e in list {
        if largest > e {
            largest = e
        }
    }

    largest
}

// Traits are abstract methods over a set of types
// analogous to behaviours in elixir.
// or rather generic classes in typescript
// via a mixin

// traits must be brought into scope
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Sumit {
    x: u32,
    y: f64
}

impl Summary for Sumit {
        fn summarize(&self) -> String {
            format!("you may sum: {} and {}", self.x, self.y)
        }
}
// trait bound implements Summary
pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound explicit syntax - same type
pub fn notif<T: Summary + Display>(item: &T) {
    println!("notif {}", item.summarize());
}

// where sugar

fn some_function<T, U>(t: &T, u: &U) -> impl Summary
where
    T: Display + Clone,
    U: Clone,
{
    Sumit{x: 5, y: 32.3}
}

// conditional methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
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