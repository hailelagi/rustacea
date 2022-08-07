fn main() {
    println!("-- ownership --");
    rule()
}

// Ownership is a set of rules that govern the memory model
// Stack [] LIFO
// Heap {} pointers

// Rules
// - Variable Scope


// string types
// char => utf-8 encoded 4 bytes
// str => stack allocated encoded bytes - string literal
// String => heap allocated growth based pointer
// clone copies heap data
// moves change stack data from one variable to another
// or from one scope to another ie. functions

// Drop trait - heap alloc
// Copy trait - stack alloc - all scalar types and tuples with scalar vals

// Ownership can be transferred with references and borrowing
// references are immutable, refs are &memory access'
// refs can be mutable - but only once.
// &mut Type


fn rule() {
    let mut s = String::from("hello");
    let mut s_2 = String::from("bye");

    do_stuff(&mut s);
    s_2 = take_stuff(s_2);

    s.push_str("!!!!");
    s_2.push_str("!!!");
    // `drop() is called freeing s and s_2`
}

fn take_stuff(mut x: String) -> String {
    x.push_str(" ,world");
    return x;
    // `drop() is called freeing x
}

fn do_stuff(s: &mut String) {
    s.push_str(" ,world");
     // `drop() is never called
}
