fn main() {
    // Compound types
    println!("Compound types example");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // accessing by pattern matching
    let (_, y, _) = tup; 
    println!("The value of y is: {y}");
    // accessing by (.) and followed by index
    println!("The value of z is: {}", tup.2); 


    // Arrays
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let number_array: [i32; 5] = [1,2,3,4,5];
    let ones_array = [1; 5];
    println!("Value of the first month : {}", months[0]);
    println!("Value of the second number_array : {}", number_array[1]);
    println!("Value of the third ones_array: {}", ones_array[2]);
}
