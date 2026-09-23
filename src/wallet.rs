// use hmac::{Hmac, Mac};
// use k256::SecretKey;
// use k256::elliptic_curve::sec1::ToEncodedPoint;
// use num_bigint::BigUint;
// use std::str::FromStr;
// use std::io;
use crate::{bip32::extract_master_keys, bip39::Seed};

pub struct Wallet{
    seed: Seed,
    master_private_key: [u8; 32],
    master_public_key: [u8; 33],
    master_chain_code: [u8; 32]
}

impl Wallet{
    pub fn new(seed_choice: String) -> Wallet{  
        let seed: Seed = Seed::new(seed_choice);
        let (master_private_key, master_chain_code, master_public_key) = extract_master_keys(seed.bytes());
        Wallet{
                seed: seed,
                master_private_key: master_private_key,
                master_public_key: master_public_key,
                master_chain_code: master_chain_code
            }
    
              
    }


    pub fn seed(&self) -> &Seed{
        &self.seed
    }

    pub fn master_private_key(&self) -> &[u8; 32] {
        &self.master_private_key
    }

    pub fn master_public_key(&self) -> &[u8; 33] {
        &self.master_public_key
    }


}


