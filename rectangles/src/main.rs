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

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let rect = Rectangle {width: 30, height: 50};

    // pprint stdout
    println!("area is {:#?}", area(&rect));

    // alt debug mode - very nice
    dbg!(area(&rect));
}


// borrow rectangle ds - read only more efficient
fn area(rectangle: &Rectangle) -> i32 {
    rectangle.height * rectangle.width
}