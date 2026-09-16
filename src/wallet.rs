use rand::{Rng, rngs::OsRng};
use bip39::{Mnemonic, Language};
use sha2::{Sha256, Digest};
use bip32::{XPrv, ChildNumber, DerivationPath};
use std::str::FromStr;
use std::io;

pub struct Wallet{
    master_private_key: u128,
    seed: String,
    master_public_key: u128
}

impl Wallet{
    pub fn new(seed_choice: String) -> Wallet{  
        if seed_choice == "1"{
            Wallet{
                master_private_key: 0,
                seed: Self::generate_seed(),
                master_public_key: 0
            }
        }
        else if seed_choice == "2"{
            Wallet{
                master_private_key: 0,
                seed: Self::import_seed(),
                master_public_key: 0
            }
        }
        else{
            panic!("Please choose between 1 or 2");
        }
    
              
    }

    fn generate_seed() -> String{
        println!("Generating a seed...");
        println!();
        let entropy: [u8; 16] = OsRng.r#gen();

        println!("All representation of the seed :");
        println!("Bytes representation : {:?}", entropy);
        println!();
        print!("Hex representation : ");
        for byte in entropy{
            print!("{:02x}", byte);
        }
        println!();
        println!();

        let mut hasher = Sha256::new();
        hasher.update(entropy);
        let hash = hasher.finalize();
        let checksum = hash[0] >> 4;

        let mut binary_string: String = String::new();
        for byte in entropy{
            binary_string.push_str(&format!("{:08b}", byte));
        }
        binary_string.push_str(&format!("{:04b}", checksum));
        println!("Binary representation : {}", binary_string);

        println!();
        println!("Sets of 11 bits :");
        let mut words_seed: Vec<String> = Vec::new();
        let word_list: &[&str; 2048] = Language::English.word_list();
        for i in(0..binary_string.len()).step_by(11){
            let chunck = &binary_string[i..i+11];
            let index = usize::from_str_radix(chunck, 2).unwrap();
            let word = word_list[index];
            print!("{} ", chunck);
            
            words_seed.push(String::from(word));
        }
        println!();
        println!();


        words_seed.join(" ")
        
    }

    fn import_seed() -> String{
        println!("Enter your 12 words seed : ");
        let mut input_seed: String = String::new();
        io::stdin().read_line(&mut input_seed).expect("Try again");
        let input_seed: String = String::from(input_seed.trim());
        println!();

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



    pub fn seed(&self) -> &String{
        &self.seed
    }
}


