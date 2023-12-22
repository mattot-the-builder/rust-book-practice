fn main() {
    // mutable variables
    println!("Mutable variables example");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("Now value of x is: {x}");

    // constants
    println!("Constant example");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant is : {THREE_HOURS_IN_SECONDS}");

    // shadowing
    println!("Shadowing example");
    let y = 5;
    println!("The value of y is : {y}");

    {
        let y = y * 2;
        println!("The shadowed value in the inner scope is : {y}");
    }

    println!("The value of y after the inner scope is : {y}");

    let spaces = "   ";
    println!("Original value of spaces is : {spaces}");
    let spaces = spaces.len();
    println!("The value of shadowed spaces is : {spaces}");
}
