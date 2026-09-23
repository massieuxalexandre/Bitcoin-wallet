mod utils;
mod wallet;
mod bip39;
mod bip32;

use crate::utils::{input, valid_input};
use crate::wallet::Wallet;

fn main() {
    println!("Welcome, let's create a personal Bitcoin wallet !");
    
    println!("1. Generate a seed");
    println!("2. Import a seed");
    let mut seed_choice: String = input();
    while valid_input(2, &seed_choice) == false{
        println!("Please choose between 1 or 2");
        seed_choice = input();
    }
    
    let wallet: Wallet = Wallet::new(seed_choice);
    println!("Your wallet is created ! There are all the representation of your seed");
    println!("Make sure to save your seed phrase securely : ");
    println!();
    wallet.seed().show_representations();

    println!("There are your master keys :");
    println!();
    println!("Master Private Key : {:02x?}", wallet.master_private_key());
    println!("Master Public Key : {:02x?}", wallet.master_public_key());
    println!();

    // println!("Let's derive some child keys from your master private key !");
    // wallet.generate_child_key(12, 30);

}
