/* --- ungrouped data e.g
fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}
*/

// structuring data using tuples
/*
fn main() {
    let rect = (30, 50);
    println!("area is {}", area(rect));
}

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
*/


// GAPS using/impl'ing traits
// Understanding rust's module system/ crates/ cargo usage
// revise testing

use rand::prelude::*;

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn color_point() {
    let black = Color(255, 255, 38);
    let origin = Point(1, 10, 0);
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

enum IpAddr {
    V4(String),
    V6(String),
}

/////////////////////
/// METHODS ////////
/////////////////////

#[derive(Debug)]
struct Circle {
    radius: u32,
}

struct Polygon {
    vertix_1: u32,
    vertix_2: u32,
    vertix_3: u32,
}

impl Circle {
    fn circumference(&self) -> f64 {
        (2 as f64) * 3.142 * (self.radius as f64)
    }

    fn can_circumscribe(&self, polygon: &Polygon) -> bool {
        self.radius >= polygon.vertix_1
            && self.radius >= polygon.vertix_2
            && self.radius >= polygon.vertix_3
    }

    // associated functions
    fn scale_circle(radius: u32, scale: u32) -> Circle {
        Circle {
            radius: radius * scale,
        }
    }
}

impl Rectangle {
    fn fuck() -> i32 {
        0
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(area(&rect));
    // pprint stdout
    println!("area is {:#?}", area(&rect));

    // alt debug mode - very nice

    let circle = Circle { radius: 10 };
    let triangle = Polygon {
        vertix_1: 5,
        vertix_2: 8,
        vertix_3: 7,
    };

    // println!("{:?}", circle.circumference());
    // println!("{:?}", circle.can_circumscribe(&triangle));
    // println!("{:?}", Circle::scale_circle(10, 2))

    println!("{:?}", value_in_cents(Coin::Wtf));
}

// borrow rectangle ds - read only more efficient
fn area(rectangle: &Rectangle) -> i32 {
    rectangle.height * rectangle.width
}

enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter,
}

fn value_in_cents(maybe_coin: Option<Coin>) -> u8 {
    match maybe_coin {
        Some(coin) => match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        },
        None => 0,
    }
}


fn oki() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("got a value! {}", max);
    }
}

fn oki_match() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}