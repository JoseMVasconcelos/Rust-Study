fn main() {

    use_reference();

    mutable_reference();

}

fn use_reference() {
    let s1 = String::from("Hello!");

    let len = calculate_lenght(&s1);

    println!("The lenght of '{}' is {}.", s1, len);

}

fn calculate_lenght(s: &String) -> usize {
    //This function takes a "&String" instead of a "String" paramenter
    //this means that the variable is actually a reference. Basically a pointer
    //to the actual variable.
    //Because of this, this function don't own the variable, and its not droped, nor need to be returned.
    s.len()
}

//Mutable references are good if we need to alter a borrowed variable
//but they have one major restriction, if a mutable reference exists, it must be exclusive.
/*
    For example:
        Two different mutable references to the same variable cannot coexist and be used at the same time;
        A mutable reference cannot coexist and be used with normal references to the same variable;  
*/

fn mutable_reference() {
    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

//Rust does NOT allow for dangling references
//for exemple the following function WILL fail to compile
/*
fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}
*/
//This function will create a new string, and try to return a reference to it
//but, since after the function ends, the string also ceases to exist, the reference
//should not exist also. So rustc won't allow it.