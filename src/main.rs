fn main() {
    println!("Control Flow");

    // if
    let is_true: bool = true;

    if is_true {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let number = 5;
    if number < 7 {
        println!("Number is less than 7");
    } else {
        println!("Number is greater than or equal to 7");
    }

    // using if in let statement
    let condition = true;
    let a = if condition { 5 } else { 6 }; // must return same type

    println!("The value of a : {a}");
}
