use hmac::{Hmac, Mac};
// use k256::SecretKey;
// use k256::elliptic_curve::sec1::ToEncodedPoint;
// use num_bigint::BigUint;
// use std::str::FromStr;
use std::io;
use crate::bip39::Seed;

pub struct Wallet{
    seed: Seed,
    master_private_key: [u8; 32],
    master_public_key: [u8; 32]
}

impl Wallet{
    pub fn new(seed_choice: String) -> Wallet{  
        let mut seed: Seed = if seed_choice == "1"{
            println!("Generating a seed...");
            println!();
            Seed::new(seed_choice)
        }
        else if seed_choice == "2"{
            println!("Enter your seed");
            println!("> ");
            let mut input_seed: String = String::new();
            io::stdin().read_line(&mut input_seed).expect("Try again");
            let input_seed: String = String::from(input_seed.trim());
            println!();
            Seed::new(seed_choice)
        }
        else{
            panic!("Please choose between 1 or 2");
        };

        let mnemonic = Mnemonic::parse(&phrase).unwrap();
        let bytes = mnemonic.to_seed("");

        let master_private = XPrv::new(&bytes).expect("Error while creating the Master Private Key");
        let master_public = master_private.public_key();

        Wallet{
                master_private_key: master_private,
                seed: seed,
                master_public_key: master_public
            }
    
              
    }


    fn import_seed() -> String{


        match Mnemonic::parse(input_seed) {
            Ok(mnemonic) => {
                println!("Success ! The imported seed is valid");
                mnemonic.to_string()
            }
            Err(error) => {
                panic!("Invalid seed error : {}", error);
            }
        }
    }



    // pub fn seed(&self) -> &Seed{
    //     self.seed
    // }

    // pub fn master_private_key(&self) -> &XPrv{
    //     &self.master_private_key
    // }

    // pub fn master_public_key(&self) -> &XPub{
    //     &self.master_public_key
    // }

}


