//Learning the rust funcions
fn main() {
    //you call functions in Rust just like you can call functions in a lot of other languages.
    println!("Function 01 - Function Call");
    another_function();
    println!("");

    //you call functions with parameters in Rust just like in other languages
    println!("Function 02 - Parameter");
    function_with_parameters(5);
    println!("");

    //same as above
    println!("Function 03 - More than one parameter");
    print_labeled_measurement(5, 'm');
    println!("");

    //This function just exists as a way to organize the code
    println!("Function 04 - Expressions and Statements");
    expression_and_statements();
    println!("");

    //Assigning a function expression to a variable
    //The function "five()" has just a char '5' in it
    let x = five();
    println!("Function 05 - Five");
    println!("The value of x is: {x}");
    println!("");

    //Same as above, but with a sum
    let y = plus_one(7);
    println!("Function 06 - Plus One");
    println!("The value of y is: {y} ")
}

//you can declare parameter-less functions in Rust just like in other languages
fn another_function() {
    println!("Another function.");
}

//in Rust, in the function declaration you HAVE to declare the type of the parameters
fn function_with_parameters(z: i32) {
    println!("The value of z is: {z}")
}

//To define multiple parameters, use a comma to separate them
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
    Statements: A statement is an instruction that, after it performs
    its action, DOES NOT return a value.

    Expression: It evaluates a resultant value, like a math operation.

    In rust, its possible to diferenciate a Statement from a Expression
    using a semicolon.
*/
fn expression_and_statements() {
    let a = {
        let b = 4;
        b + 1
    };
    //for the expression 'b + 1', results in 'a' with a value of 5
    //but the statement 'b + 1;', results in an error
    println!("The value of a is: {a}");
}

//A function with a return value in Rust must have its returning type declared
//And the returning value is the final EXPRESSION in the body of a function
//So, just having a '5' with no semicolon, is a totally acceptable function body
fn five() -> char {
    '5'
}

//The same can be seen here, the EXPRESSION 'y + 1' returned
//Its possible to return early from a function using the return keyword just like other languages
fn plus_one(y: i32) -> i32 {
    y + 1
}
