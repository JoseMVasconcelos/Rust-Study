use std::io;


//This code should result in a panic
fn main() {
    //creating an array "a", with five 32-bit integers
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    //creating a mutable String variable for the input
    let mut index: String = String::new();

    //reading the user input using the standard input-output crate
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    //parsing the value of the user input from String to integer
    //if the value of the user input was not a number, the code panics
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    //accessing the array using the user input index
    let element = a[index];

    //if the index chosen was a value from 0..=4, the println! should run
    //if it was any other integer, the code panics and throws an error
    println!("The value of the element at index {index} is: {element}");

}
