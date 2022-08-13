use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {}", query);
    print!("in {}", file_path)
}

// Error handling

// Match Option Enum - Some or None
// Match Result Enum Ok Err

// unwrap
// expect an error
// or question mark an error - via from convert errors to caller

