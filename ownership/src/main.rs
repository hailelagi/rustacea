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
// char => unicode scalar value - encoded 4 bytes
// str => stack allocated encoded bytes - string literal slice of String
// contained in programs binary, hence the &str reference.
// String => heap allocated growth based pointer of utf-8 bytes
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

    println!("s is at {:p}", &s);
    println!("s_2 is at {:p}", &s_2);

    do_stuff(&mut s);

    // declared s_2 is moved into take_stuff
    s_2 = take_stuff(s_2);
    println!("s_2 is now at {:p}", &s_2);

    s.push_str("!!!!");
    s_2.push_str("!!!");

    println!("s is finally at {:p}", &s);
    println!("s 2 is finally at {:p}", &s_2);
    // `drop() is called freeing s and s_2`

    println!("{}", own());
}

fn take_stuff(mut x: String) -> String {
    x.push_str(
        "
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation
        a realllyyy loongggg possilbly large string to trigger heap reallocation",
    );

    println!("moved at {:p}", &x);
    return x;
    // `drop() is called freeing x
}

fn do_stuff(s: &mut String) {
    s.push_str(" ,world");
    // `drop() is never called
}

fn own() -> String {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s2 is {}", s2);
    println!("{}", s3);

    return format!("{}-{}-{}", s1, s2, s3);
}

// Slices - https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type
// slices are pointer based views into the len up to cap of an array
