fn main() {
    println!("Hello, world!");

    test_function();

    let sum = sum_numbers(1,2);
    println!("Sum: {sum}");

    // printing function output directly without storing 
    println!("Sum: {}", sum_numbers(2,3));
}

// simple function
fn test_function() {
    println!("Im from test_function() ");
}

// function with return value
fn sum_numbers(x: i32, y: i32) -> i32 {
    x + y // don't need ; for expression (return value)
}
