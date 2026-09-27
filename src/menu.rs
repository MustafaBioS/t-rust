use std::io;
use crate::main;

pub fn menu() -> bool {
    println!(" ");
    println!("Main Menu");
    println!(" ");
    println!("1 - Start");
    println!("2 - Exit");
    println!(" ");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    let trimmed = inp.trim();

    if trimmed == "1" {
        true
    } else if trimmed == "2" {
        false
    } else {
        println!("Invalid Choice");
        false
    }
}