use std::io;

mod wallet;
use crate::wallet::Wallet;

fn main() {
    println!("Welcome, let's create a personal Bitcoin wallet !");
    
    println!("1. Generate a seed");
    println!("2. import a seed");
    let mut seed_choice: String = String::new();
    io::stdin().read_line(&mut seed_choice).expect("Try again");
    println!();
    
    let wallet: Wallet = Wallet::new(String::from(seed_choice.trim()));
    println!("Your wallet is created ! Please make sure you noted your phrase seed :");
    println!("{}", wallet.seed());
    println!();


}
