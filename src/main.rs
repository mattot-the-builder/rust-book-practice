fn main() {

    // simple loop
    loop {
        println!("Im in loop body");
        break; // break the loop after printing message once
    }

    // returning value from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result : {result}");

    // loop with labels
    let mut count = 0;

    'upper_loop: loop {
        println!("count : {count}");
        let mut remaining = 10;

        loop {
            println!("remaining : {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'upper_loop;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while loop
    let mut while_loop_counter = 0;
    while while_loop_counter != 10 {
        println!("while_loop_counter : {while_loop_counter}");
        while_loop_counter += 1;
    }

    println!("Finish counting for while loop");
}
