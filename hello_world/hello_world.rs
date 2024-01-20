use std::{thread, time};

fn main(){
    let five_second = time::Duration::from_millis(5000);
    let now = time::Instant::now();
    println!("Hello World!");

    thread::sleep(five_second);

    assert!(now.elapsed() >= five_second);
}

