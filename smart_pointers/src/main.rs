use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;

/*
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
*/

// Smart pointer is a regular pointer with metadata

// Box Rc Ref RefMut - RefCell
// Arc

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// Box impls Deref and Drop
// Box useful for:
// indirection/heap alloc

// moving large data
// unkown compile time types
// trait guarding impl

fn main() {
    /* 
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", l);
    */

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    let c = CustomerSmartPointer{data: String::from("c")};
    let d = CustomerSmartPointer{data: String::from("d")};

    println!("Customer smart pointers in scope");
    assert_eq!(5, *y);

    drop(d);
    drop(c);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// Stack based Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

struct CustomerSmartPointer {
    data: String
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("dropping self {}", self.data);
    }
}

// Reference Counting
