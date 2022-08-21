use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
// use std::rc::Rc;

// Message passing concuurency
fn main() {
    // thread_the_needle()
    // capture();
    // channels();
    //  multi_channel();
    shared_state();
}

fn thread_the_needle() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    println!("{:?}", handle.join().unwrap());
}

fn capture() {
    let v = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || println!("vec => {:?}", v));
    handle.join().unwrap()
}

fn channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        println!("val {}", val);
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn multi_channel() {
    let (tx, rx) = mpsc::channel();
    let (tx_two, rx_two) = mpsc::channel::<String>();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }

    /*
    for received in rx_two {
        println("Never gets anything... waits foreever")
        println!("Got {}", received);
    }
    */
}

fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
