use std::io;
mod wallet;
mod bip39;
mod bip32;
use crate::wallet::Wallet;

fn main() {
    println!("Welcome, let's create a personal Bitcoin wallet !");
    
    println!("1. Generate a seed");
    println!("2. import a seed");
    println!("> ");
    let mut seed_choice: String = String::new();
    io::stdin().read_line(&mut seed_choice).expect("Try again");
    println!();
    
    let wallet: Wallet = Wallet::new(String::from(seed_choice.trim()));
    println!("Your wallet is created ! Please make sure you noted your phrase seed :");
    println!("{}", wallet.seed());
    println!();

    println!("There are your master keys :");
    println!();
    println!("Master Private Key : {:#?}", wallet.master_private_key());
    println!("Master Public Key : {:#?}", wallet.master_public_key());
    println!();

    println!("Let's derive some child keys from your master private key !");
    wallet.generate_child_key(12, 30);

}
