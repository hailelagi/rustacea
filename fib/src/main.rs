fn main() {
    println!("Hello, world!");
    println!("{}", fib(32));
}

/* Prefer loops over recursion in rust */
fn fib(n: i32) -> i32 {
    if n <= 3 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

