//Since the majority of this chapter explains how the compiler
//works, and most of the examples result in code panics, or compiler errors,
//I will not be putting them here for the most part

fn main() {
    string_clone();

    let s: String = String::from("Hello!");
    take_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn string_clone() {
    let s1 = String::from("Hello!");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
