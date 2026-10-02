use hmac::{Hmac, Mac};
use sha2::Sha512;
use k256::SecretKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use num_bigint::BigUint;
use crate::seed::Seed;
use crate::utils::{input};

pub struct Wallet{
    seed: Seed,
    master_private_key: [u8; 32],
    master_public_key: [u8; 33],
    master_chain_code: [u8; 32]
}

impl Wallet{
    pub fn new(seed_choice: String) -> Wallet{  
        let seed: Seed = Seed::new(seed_choice);
        let (master_private_key, master_public_key, master_chain_code) = Self::extract_master_keys(seed.bytes());
        Wallet{
                seed: seed,
                master_private_key: master_private_key,
                master_public_key: master_public_key,
                master_chain_code: master_chain_code
            }
    
              
    }

    fn extract_master_keys(seed_bytes: &[u8; 64]) -> ([u8; 32], [u8; 33], [u8; 32]) {
        let mut hmac = Hmac::<Sha512>::new_from_slice(b"Bitcoin seed").expect("Error HMAC");
        hmac.update(seed_bytes);
        let bytes = hmac.finalize().into_bytes();

        let mut private_key: [u8; 32] = [0u8; 32];
        let mut chain_code: [u8; 32] = [0u8; 32];

        private_key.copy_from_slice(&bytes[0..32]);
        chain_code.copy_from_slice(&bytes[32..64]);

        let secret_key = SecretKey::from_slice(&private_key).expect("Error private key");
        let public_key_point = secret_key.public_key();
        let compressed_public = public_key_point.to_encoded_point(true);
        let mut public_key = [0u8; 33];
        public_key.copy_from_slice(compressed_public.as_bytes());



        (private_key, public_key, chain_code)
    }

    pub fn generate_child_key(&self, parent_private_key: &[u8; 32], parent_public_key: &[u8; 33], parent_chain_code: &[u8; 32], index: u32) -> ([u8; 32], [u8; 33], [u8; 32]) {
        let mut data: Vec<u8> = Vec::new();
        
        data.extend_from_slice(parent_public_key);
        data.extend_from_slice(&index.to_be_bytes());

        let mut hmac = Hmac::<Sha512>::new_from_slice(parent_chain_code.as_slice()).expect("Error HMAC");
        hmac.update(&data);
        let bytes = hmac.finalize().into_bytes();


        let left_number = BigUint::from_bytes_be(&bytes[0..32]);
        let parent_number = BigUint::from_bytes_be(parent_private_key.as_slice());
        
        let n_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
        let n = BigUint::parse_bytes(n_hex.as_bytes(), 16).expect("Erreur parsing n");


        let child_number = (left_number + parent_number) % n;
        let child_number_bytes = child_number.to_bytes_be();
        
        let mut child_private_key = [0u8; 32];
        let start_index: usize = 32 - child_number_bytes.len();
        (&mut child_private_key[start_index..]).copy_from_slice(&child_number_bytes);

        let mut child_chain_code = [0u8; 32];
        child_chain_code.copy_from_slice(&bytes[32..64]);


        let secret_key = SecretKey::from_slice(&child_private_key).expect("Error private key");
        let public_key_point = secret_key.public_key();
        let compressed_public = public_key_point.to_encoded_point(true);
        
        let mut child_public_key = [0u8; 33];
        child_public_key.copy_from_slice(compressed_public.as_bytes());

        (child_private_key, child_public_key, child_chain_code)
    }


    pub fn seed(&self) -> &Seed{
        &self.seed
    }

    pub fn master_private_key(&self) {
        print!("Master private key : ");
        for i in 0..self.master_private_key.len() {
            print!("{:02x}", &self.master_private_key[i]);
        }
        println!();
        println!();
    }

    pub fn master_public_key(&self) {
        print!("Master public key : ");
        for i in 0..self.master_public_key.len() {
            print!("{:02x}", &self.master_public_key[i]);
        }
        println!();
        println!();
    }

    pub fn master_chain_code(&self) {
        print!("Master chain code : ");
        for i in 0..self.master_chain_code.len() {
            print!("{:02x}", &self.master_chain_code[i]);
        }
        println!();
        println!();
    }

    pub fn derive(&self) {
        println!("Enter the index of the child key you want to derive : ");
        let mut index_input: String = input();
        while index_input.parse::<u32>().is_err() {
            println!("Please enter a valid index (0 to 4294967295)");
            index_input = input();
        }
        let index: u32 = index_input.parse::<u32>().unwrap();

        let (child_private_key, child_public_key, child_chain_code) = self.generate_child_key(&self.master_private_key, &self.master_public_key, &self.master_chain_code, index);

        print!("Child private key : ");
        for i in 0..child_private_key.len() {
            print!("{:02x}", &child_private_key[i]);
        }
        println!();
        println!();

        print!("Child public key : ");
        for i in 0..child_public_key.len() {
            print!("{:02x}", &child_public_key[i]);
        }
        println!();
        println!();

        print!("Child chain code : ");
        for i in 0..child_chain_code.len() {
            print!("{:02x}", &child_chain_code[i]);
        }
        println!();
        println!();
    }


    pub fn derive_level_m(&self) {
        println!("Enter the depth level M : ");
        let mut m_input: String = input();
        while m_input.parse::<usize>().is_err() {
            println!("Please enter a valid positive number for M");
            m_input = input();
        }
        let m: usize = m_input.parse::<usize>().unwrap();

        println!("Enter the index N for the derivation path : ");
        let mut index_input: String = input();
        while index_input.parse::<u32>().is_err() {
            println!("Please enter a valid index (0 to 4294967295)");
            index_input = input();
        }
        let index: u32 = index_input.parse::<u32>().unwrap();


        let mut current_priv = self.master_private_key;
        let mut current_pub = self.master_public_key;
        let mut current_chain = self.master_chain_code;


        for _level in 1..=m {
            let (p, pub_k, c) = self.generate_child_key(&current_priv, &current_pub, &current_chain, index);
            current_priv = p;
            current_pub = pub_k;
            current_chain = c;
            
        }

        print!("Derived private key at level {} : ", m);
        for i in 0..current_priv.len() {
            print!("{:02x}", &current_priv[i]);
        }
        println!();
        println!();

        print!("Derived public key at level {} : ", m);
        for i in 0..current_pub.len() {
            print!("{:02x}", &current_pub[i]);
        }
        println!();
        println!();

        print!("Derived chain code at level {} : ", m);
        for i in 0..current_chain.len() {
            print!("{:02x}", &current_chain[i]);
        }
        println!();
        println!();
    }


}


