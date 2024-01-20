//Learning the ability to control the flow of the code in Rust
//I will be separating each demo in different functions
fn main() {
    println!("Demo 01 - If Expressions");
    if_expressions();
    println!("");

    println!("Demo 02 - Multiples conditions");
    multiples_conditions();
    println!("");

    println!("Demo 03 - Let If");
    let_if();
    println!("");

    println!("Demo 04 - Loops!");
    looping();
    println!("");

    println!("Demo 05 - Loops in loops");
    nested_loops();
    println!("");

    println!("Demo 06 - While");
    conditional_loops();
    println!("");
    
    println!("Demo 07 - For loop");
    for_loop();
    println!("");

    println!("Demo 08 - For loop in range");
    for_loop_in_range();
}

fn if_expressions() {
    let number: i32 = 3;
    
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    //important to note, Rust DOES NOT convert non-boolean types to boolean
    //'number = 3; if number {}' will NOT work
}

fn multiples_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    let condition = true;
    //both of the values MUST be of the same type
    //'if condition { 5 } else { 'six' }' will NOT work
    let number = if condition { 5 } else { 6 };

    println!("The value of number on condition {condition} is {number}");
}

fn looping() {
    let mut counter = 0;
    const MAX_COUNTER: i32 = 3;

    let result = loop {
        counter += 1;
        
        if counter == MAX_COUNTER {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

fn nested_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Left while's grasp!")
}

fn for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_loop_in_range() {
    for number in (0..=5).rev() {
        println!("{number}!");
    }
    println!("Left the for loop!");
}