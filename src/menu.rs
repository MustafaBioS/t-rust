use crossterm::terminal::disable_raw_mode;
use std::io;
use std::process::exit;

pub fn menu() {
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

    match trimmed {
        "1" => {
            return;
        }
        "2" => {
            disable_raw_mode().unwrap();
            exit(0);
        }
        _ => {
            println!("Invalid Choice");
            disable_raw_mode().unwrap();
            exit(1)
        }
    }
}
