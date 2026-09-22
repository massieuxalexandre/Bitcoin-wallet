// use hmac::{Hmac, Mac};
// use k256::SecretKey;
// use k256::elliptic_curve::sec1::ToEncodedPoint;
// use num_bigint::BigUint;
// use std::str::FromStr;
// use std::io;
use crate::bip39::Seed;

pub struct Wallet{
    seed: Seed,
    master_private_key: [u8; 32],
    master_public_key: [u8; 32],
    chain_code: [u8; 32]
}

impl Wallet{
    pub fn new(seed_choice: String) -> Wallet{  
        let seed: Seed = Seed::new(seed_choice);
        let phrase: String = String::from(seed.phrase());

        // let mnemonic = Mnemonic::parse(&phrase).unwrap();
        // let bytes = mnemonic.to_seed("");

        // let master_private = XPrv::new(&bytes).expect("Error while creating the Master Private Key");
        // let master_public = master_private.public_key();

        Wallet{
                seed: seed,
                master_private_key: [0; 32],
                master_public_key: [0; 32],
                chain_code: [0; 32]
            }
    
              
    }


    pub fn seed(&self) -> &Seed{
        &self.seed
    }


}


