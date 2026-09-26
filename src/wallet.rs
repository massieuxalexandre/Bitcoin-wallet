use hmac::{Hmac, Mac};
use sha2::Sha512;
use k256::SecretKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use num_bigint::BigUint;
use crate::seed::Seed;

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

    pub fn generate_child_key(parent_private_key: &[u8; 32], parent_public_key: &[u8; 33], parent_chain_code: &[u8; 32], index: u32) -> ([u8; 32], [u8; 33], [u8; 32]) {
        let mut data: Vec<u8> = Vec::new();
        
        data.extend_from_slice(parent_public_key);
        data.extend_from_slice(&index.to_be_bytes());

        let mut hmac = Hmac::<Sha512>::new_from_slice(parent_chain_code.as_slice()).expect("Error HMAC");
        hmac.update(&data);
        let bytes = hmac.finalize().into_bytes();


        let left_number = BigUint::from_bytes_be(&bytes[0..32]);
        let parent_number = BigUint::from_bytes_be(parent_private_key.as_slice());
        
        let n_hex = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
        let n = BigUint::parse_bytes(n_hex.as_slice(), 16).expect("Erreur parsing n");
        

        let child_number = (left_number + parent_number) % n;
        let child_number_bytes = child_number.to_bytes_be();
        
        let mut child_private_key = [0u8; 32];
        let start_index = 32 - child_number_bytes.len();
        child_private_key[start_index..].copy_from_slice(&child_number_bytes);


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


}


