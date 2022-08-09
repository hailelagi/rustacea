fn main() {
    let v: Vec<i32> = Vec::new();
    let x = vec![1, 2];

    println!("{:?} {:?}", v, x);
    println!("Hello, world!");

    follow()
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

// Rust collections
// Vec and HashMap

// Sequences
// Maps
// Sets
// Binary Heap
