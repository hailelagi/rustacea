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
    height: i32
}


/////////////////////
/// METHODS ////////
/////////////////////

#[derive(Debug)]
struct Circle {
    radius: u32
}

struct Polygon {
    vertix_1: u32,
    vertix_2: u32,
    vertix_3: u32 
}

impl Circle {
    fn circumference(&self) -> f64 {
        (2 as f64) * 3.142 * (self.radius as f64)
    }

    fn can_circumscribe(&self, polygon: &Polygon) -> bool {
        self.radius >= polygon.vertix_1 && 
        self.radius >= polygon.vertix_2 && 
        self.radius >= polygon.vertix_3
    }

    // associated functions
    fn scale_circle(radius: u32, scale: u32) -> Circle {
        Circle {radius: radius * scale}
    }
}

fn main() {
    // let rect = Rectangle {width: 30, height: 50};

    // pprint stdout
    // println!("area is {:#?}", area(&rect));

    // alt debug mode - very nice
    // dbg!(area(&rect));

    let circle = Circle {radius: 10};
    let triangle = Polygon{vertix_1: 5, vertix_2: 8, vertix_3: 7};

    println!("{:?}", circle.circumference());
    println!("{:?}", circle.can_circumscribe(&triangle));
    println!("{:?}", Circle::scale_circle(10, 2))
}


// borrow rectangle ds - read only more efficient
fn area(rectangle: &Rectangle) -> i32 {
    rectangle.height * rectangle.width
}

