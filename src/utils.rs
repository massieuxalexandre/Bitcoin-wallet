use std::io::{self, Write};

pub fn input() -> String{
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Try again");
    println!();

    String::from(input.trim())

}