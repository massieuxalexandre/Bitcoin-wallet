use hmac::{Hmac, Mac};
use sha2::Sha512;
use k256::SecretKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;

pub fn extract_master_keys(seed_bytes: &[u8; 64]) -> ([u8; 32], [u8; 32], [u8; 33]) {
    let mut hmac = Hmac::<Sha512>::new_from_slice(b"Bitcoin seed").expect("Erreur HMAC");
    hmac.update(seed_bytes);
    let bytes = hmac.finalize().into_bytes();

    let mut private_key: [u8; 32] = [0u8; 32];
    let mut chain_code: [u8; 32] = [0u8; 32];

    private_key.copy_from_slice(&bytes[0..32]);
    chain_code.copy_from_slice(&bytes[32..64]);

    let secret_key = SecretKey::from_slice(&private_key).expect("Erreur de clé privée");
    let public_key_point = secret_key.public_key();
    let compressed_public = public_key_point.to_encoded_point(true);
    let mut public_key = [0u8; 33];
    public_key.copy_from_slice(compressed_public.as_bytes());



    (private_key, chain_code, public_key)
}


