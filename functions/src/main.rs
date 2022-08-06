fn main() {
    println!("Hello, world!");
    println!("{:?}", another_func(52));
    println!("{:?}", another(52, 52));
}

// this is ok because x + 2 is an expression that evaluates
fn another_func(x: i32) -> i32 {
    x + 2
}

// this is NOT okay because the ; implies a statement which returns the unit tuple ()
/*
fn another_func(x: i32) -> i32 {
    x + 2
}
*/

// this is fine too if you want to be explicit
fn another(x: i32, y: i32) -> i32 {
    return x + y + 2
}
