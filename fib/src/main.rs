use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Create a new atomic counter with an initial value of 0
    let counter = AtomicUsize::new(0);

    // Perform an atomic increment operation on the counter
    counter.fetch_add(1, Ordering::SeqCst);

    // Get the current value of the counter
    let value = counter.load(Ordering::SeqCst);

    println!("Counter value: {}", value);
}