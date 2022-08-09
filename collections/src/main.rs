use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let x = vec![1, 2];

    println!("{:?} {:?}", v, x);
    println!("Hello, world!");

    // follow();
    // unicode();
    dictionary();
    panic!()
}

pub fn follow() {
    let mut v = vec![1, 2, 3, 4];

    for i in &mut v {
        // dereferencing
        *i = *i + 1;
        println!("{}", i)
    }

    for x in v.iter_mut() {
        *x += 1;
    }

    println!("{:?}", v)
}

fn unicode() {
    // .chars returns the unicode char code point
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in  "नमस्ते".bytes() {
        println!("{}", b)
    }
}

fn dictionary() -> HashMap<String, u8> {
    let mut dict: HashMap<String, u8> = HashMap::new();

    dict.insert(String::from("hey"), 1);
    dict.insert(String::from("bye"), 0);

    match dict.get("hey") {
        Some(v) => println!("{}", v),
        None => ()
    }

    dict.get("hey").unwrap();

    return dict
}

// Rust collections
// Vec and HashMap

// Sequences
// Maps
// Sets
// Binary Heap
