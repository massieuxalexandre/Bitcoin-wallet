use rand::{Rng, rngs::OsRng};
use bip39::Mnemonic;
use sha2::{Sha256, Digest};
use crate::utils::{input};

pub struct Seed{
    entropy: [u8; 16],
    binary: String,
    phrase: String,
    bytes: [u8; 64]
}

impl Seed{
    pub fn new(seed_choice: String) -> Seed {
        let entropy: [u8; 16];
        let binary: String;
        let phrase: String;
        let bytes: [u8; 64];
        if seed_choice == "1" {
            println!("Generating a seed...");
            println!();
            entropy = OsRng.r#gen();
            binary = Self::entropy_to_binary(&entropy);
            phrase = Self::binary_to_phrase(&binary);
            bytes = Self::phrase_to_bytes(&phrase);
        }

        else if seed_choice == "2" {
            phrase = Self::import_seed();
            binary = Self::phrase_to_binary(&phrase);
            entropy = Self::binary_to_entropy(&binary);
            bytes = Self::phrase_to_bytes(&phrase);
        }
        else{
            panic!("Please choose between 1 or 2");
        }


        Seed{
            entropy: entropy,
            binary: binary,
            phrase: phrase,
            bytes: bytes
        }
    }

    fn entropy_to_binary(entropy: &[u8; 16]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(entropy);
        let hash = hasher.finalize();
        let checksum = hash[0] >> 4;

        let mut binary_string: String = String::new();
        for byte in entropy{
            binary_string.push_str(&format!("{:08b}", byte));
        }
        binary_string.push_str(&format!("{:04b}", checksum));
        binary_string
    }


    fn binary_to_phrase(binary: &String) -> String {
        let mut words: Vec<String> = Vec::new();
        let english_txt = include_str!("english.txt");
        let word_list: Vec<&str> = english_txt.lines().collect();

        for i in(0..binary.len()).step_by(11){
            let chunck = &binary[i..i+11];
            let index = usize::from_str_radix(chunck, 2).unwrap();
            let word = word_list[index];

            words.push(String::from(word));
        }
        words.join(" ")
    }

    fn import_seed() -> String{
        println!("Enter your seed phrase");
        let user_seed: String = input();

        match Mnemonic::parse(user_seed) {
            Ok(mnemonic) => {
                println!("Success ! The imported seed is valid");
                mnemonic.to_string()
            }
            Err(error) => {
                panic!("Invalid seed error : {}", error);
            }
        }
    }

    fn phrase_to_binary(phrase: &str) -> String {
        let mut binary_string = String::new();
        let english_txt = include_str!("english.txt");
        let word_list: Vec<&str> = english_txt.lines().collect();

        for word in phrase.split_whitespace() {
            let index = word_list.iter().position(|&w| w == word).unwrap();
            
            binary_string.push_str(&format!("{:011b}", index));
        }
        
        binary_string 
    }

    fn phrase_to_bytes(phrase: &String) -> [u8; 64]{
        let mnemonic = Mnemonic::parse(phrase).unwrap();
        let seed_bytes_vec = mnemonic.to_seed("");
        
        // 2. On les copie dans un beau tableau fixe de 64 octets
        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(&seed_bytes_vec);
        bytes
    }

    fn binary_to_entropy(binary: &str) -> [u8; 16] {
        let mut entropy = [0u8; 16];

        
        for i in 0..16 {
            let start = i * 8;
            let end = start + 8;
            let chunk = &binary[start..end];
            
            entropy[i] = u8::from_str_radix(chunk, 2).unwrap();
        }

        entropy
    }

    pub fn show_representations(&self) {
        print!("Hex representation : ");
        for byte in self.entropy{
            print!("{:02x}", byte);
        }
        println!();
        println!();

        print!("Binary representation : ");
        println!("{}", self.binary);
        println!();

        print!("Bytes representation : ");
        for byte in self.bytes {
            print!("{:02x}", byte);
        }
        println!();
        println!();

        print!("Phrase representation : ");
        println!("{}", self.phrase);
        println!();

    }

    pub fn bytes(&self) -> &[u8; 64] {
        &self.bytes
    }

    pub fn phrase(&self) -> &String {
        &self.phrase
    }
}