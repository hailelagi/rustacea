fn main() {
    println!("Hello, world!");
    println!("{}", convert_to_celcius(32));
    println!("{}", convert_to_fahrenheit(0));
}

fn convert_to_celcius(temp: i32) -> i32 {
    (temp - 32) * (5 / 9)
}

fn convert_to_fahrenheit(temp: i32) -> i32 {
    (temp * (9 / 5)) + 32
}
