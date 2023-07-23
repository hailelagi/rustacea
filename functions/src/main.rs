// fn main() {
//     println!("Hello, world!");
//     // println!("{:?}", another_func(52));
//     // println!("{:?}", another(52, 52));
//     // lift_off()

//     let mut v = new_vec();

//     print_vec(&mut v);
//     print_vec(&mut v);
// }

// fn print_vec(v: &mut Vec<i32>) {
//     v.push(1);

//     for i in v.iter() {
//         println!("{}", i);
//     }
// }

// // this is ok because x + 2 is an expression that evaluates
// fn another_func(x: i32) -> i32 {
//     x + 2
// }

// fn new_vec() -> Vec<i32> {
//     let mut v = Vec::new();
//     v.push(1);
//     v.push(2);

//     return v;
// }

// // this is NOT okay because the ; implies a statement which returns the unit tuple ()
// /*
// fn another_func(x: i32) -> i32 {
//     x + 2
// }
// */

// // this is fine too if you want to be explicit
// fn another(x: i32, y: i32) -> i32 {
//     return x + y + 2
// }

// fn lift_off() {
//     for _ in 0..4 {
//         println!("go go go go");
//     }
//     println!("LIFTOFF!!!");
// }

// fn x(n: i32) -> i32 {
//   println!("{}", n);
//   n
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     println!("{:?}", handle);

//     handle.join().unwrap();
// }

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}