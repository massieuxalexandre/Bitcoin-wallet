use std::io::{self, Write};

pub fn input() -> String{
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Try again");
    println!();

    String::from(input.trim())

}

pub fn valid_input(range: u8, input: &String) -> bool {
    let mut choices: Vec<String> = Vec::new();
    for i in 1..=range {
        choices.push(i.to_string());
    }

    choices.contains(&input)

}

pub fn menu(){
    println!("Main menu");
    println!("1. Show all representations of your seed");
    println!("2. Show master private key");
    println!("3. Show master public key");
    println!("4. Show master chain code");
    println!("5. Derive");
    println!("6. Derive");
    println!("7. Quit");
}