mod utils;
mod wallet;
mod seed;

use crate::utils::{input, valid_input, menu};
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
    println!("Your wallet is created !");
    println!("Make sure to save your seed phrase securely : {}", wallet.seed().phrase());
    println!();


    let mut stop: bool = false;
    while stop == false {
        println!("Press ENTER to continue");
        let _continue: String = input();

        menu();
        let mut menu_choice: String = input();
        while valid_input(7, &menu_choice) == false {
            println!("Please choose from 1 to 7");
            menu_choice = input();
        }


        if menu_choice == "1" {
            wallet.seed().show_representations();
        }
        
        else if menu_choice == "2" {
            wallet.master_private_key();
        }

        else if menu_choice == "3" {
            wallet.master_public_key();
        }

        else if menu_choice == "4" {
            wallet.master_chain_code();
        }

        // 

        else if menu_choice == "7" {
            stop = true;
        }
    }


}
