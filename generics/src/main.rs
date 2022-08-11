fn main() {
    // println!("Hello, world!");

    // let nums = vec![1, 29, 42, 4, 5];
    // let mut largest = nums[0];

    // for n in nums {
    //     if n > largest {
    //         largest = n;
    //     }
    // }

    // println!("{}", largest)
    let l: Vec<i32> = vec![1, 35, 0, 5, 52, 2525, 32];
    let x = vec![1, 3, 8, 2, 2];

    println!("{}", largest(&l));
    println!("{}", largest(&x))
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// Generic Data Types
struct Point<T> {
    x: T,
    y: i32
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Stuff<T> {
    Some(T),
    None
}
// largest is generic over T
fn large<T>(list: &[T]) -> T {
    let p = Point{x: 1, y: 2};

    let mut largest = list[0];

    for &e in list {
        if largest > e {
            largest = e
        }
    }

    largest
}

