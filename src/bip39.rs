use rand::{Rng, rngs::OsRng};
use bip39::{Mnemonic, Language};
use sha2::{Sha256, Digest};

pub struct Seed{
    entropy: [u8; 16],
    mut binary: String,
    mut phrase: String
}

impl Seed{
    pub fn new() -> Seed {
        let seed: Seed = Seed{
            entropy: OsRng.r#gen(),
            binary: String::new(),
            phrase: String::new()
        };
        seed.to_binary();
        seed.to_phrase();
        seed
    }

    // fn to_binary(&self) {
    //     let mut hasher = Sha256::new();
    //     hasher.update(self.entropy);
    //     let hash = hasher.finalize();
    //     let checksum = hash[0] >> 4;

    //     let mut binary_string: String = String::new();
    //     for byte in self.entropy{
    //         binary_string.push_str(&format!("{:08b}", byte));
    //     }
    //     self.binary = binary_string.push_str(&format!("{:04b}", checksum));
    // }
    fn to_binary(entropy: [u8; 16]) -> String {
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

    // fn to_phrase(&self) {
    //     let mut words: Vec<String> = Vec::new();
    //     let english_txt = include_str!("english.txt");
    //     let word_list: Vec<&str> = english_txt.lines().collect();

    //     for i in(0..self.binary.len()).step_by(11){
    //         let chunck = &self.binary[i..i+11];
    //         let index = usize::from_str_radix(chunck, 2).unwrap();
    //         let word = word_list[index];

    //         words.push(String::from(word));
    //     }
    //     self.phrase = words.join(" ");
    // }
    fn to_phrase(binary: String) -> String {
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

    fn show_representations(&self) {
        println!("Hex representation : ");
        for byte in self.entropy{
            print!("{:02x}", byte);
        }
        println!();
        println!();
        println!("Binary representation : ");
        println!("{}", self.binary);
        println!();
        println!();

        println!("Phrase representation : ");
        println!("{}", self.phrase);
        println!();
        println!();

    }
}