fn main() {

    let s = String::from("Hello world!");

    let hello = &s[0..5];
    let world = &s[6..12];

    let a = [1, 2, 3, 4, 5];
    let array_slice = &a[1..3];
    assert_eq!(array_slice, &[2, 3]);

    /*
    These two are equal:
        let slice1 = &s[0..2];
        let slice2 = &s[..2];
    */

    /*
    These two are also equal:
        let len = s.len();

        let slice1 = &s[3..len];
        let slice2 = &s[3..];
    */

    /*
    These are also equal:
        let len = s.len();

        let slice1 = &s[0..len];
        let slice2 = &s[..];
    */
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}