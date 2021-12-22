fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    // constants
    const THREE_HOURS_IN_SECS: u32 = 60 * 60 * 3;
    println!("three hours later... {}", THREE_HOURS_IN_SECS);

    //shadowing
    // can apply different data types
    // creates new variable `y` at runtime 
    let y = 6;
    let y = y + 1;


    {
        let y = y * 2;
        println!("The val in inner scope {}", y);
        println!("Also wtf you can create arbitrary scope?? loololol");
    }

    println!("The val {}", y);

}
