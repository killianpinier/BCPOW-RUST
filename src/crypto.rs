
use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey},
    sha2, schnorr::signature::Verifier
};
use rand_core::OsRng;
use ripemd::{Ripemd160, Digest};
use std::error::Error;


pub const SHA256_DIGEST_LENGTH: usize = 32;
pub const RIPEMD160_DIGEST_LENGTH: usize = 20;
pub const CHECKSUM_LENGTH: usize = 4;

pub fn generate_signing_key() -> Vec<u8> {
    let sk = SigningKey::random(&mut OsRng);
    // let sk = signing_key.to_bytes();
    //println!("Len: {}, {:x?}", sk.len(), hex::encode(sk));
    sk.to_bytes().into_iter().collect()
}

pub fn get_pub_key(private_key: &[u8], compress: bool) -> Result<Vec<u8>, Box<dyn Error>> {
    let signing_key = get_signing_key(private_key)?;
    let verifying_key = VerifyingKey::from(signing_key);
    let pubk = verifying_key.to_encoded_point(compress);
    Ok(pubk.as_bytes().to_vec())
}

pub fn get_checksum(input: &[u8]) -> [u8; CHECKSUM_LENGTH] {
    let mut hash = [0u8; SHA256_DIGEST_LENGTH];
    hash = sha256(input);
    hash = sha256(&hash);
    hash[0..CHECKSUM_LENGTH].try_into().unwrap()
}

pub fn get_address(pub_key: &[u8], prefix: u8) -> String {
    let pubkey_hash = get_pubkey_hash(pub_key);
    let mut address = add_prefix(prefix, &pubkey_hash);
    get_checksum(&address).iter().for_each(|b| address.push(*b));
    bs58::encode(address).into_string()
}

pub fn sign(private_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let signing_key = get_signing_key(private_key)?;
    let signature: Signature = signing_key.sign(data);
    Ok(signature.to_vec())
}

pub fn verify_signature(pubkey: &[u8], signature: &[u8], data: &[u8]) -> Result<bool, Box<dyn Error>> {
    let verifying_key = VerifyingKey::from_sec1_bytes(pubkey)?;
    let signature = Signature::try_from(signature)?;
    Ok(verifying_key.verify(data, &signature).is_ok())
}

fn get_signing_key(bytes: &[u8]) -> Result<SigningKey, Box<dyn std::error::Error>>{
    Ok(SigningKey::from_slice(bytes)?)
}

pub fn sha256(input: &[u8]) -> [u8; SHA256_DIGEST_LENGTH] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn ripemd160(input: &[u8]) -> [u8; RIPEMD160_DIGEST_LENGTH] {
    let mut hasher = Ripemd160::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn get_pubkey_hash(pub_key: &[u8]) -> [u8; RIPEMD160_DIGEST_LENGTH] {
    let hash = sha256(pub_key);
    ripemd160(&hash)
}

fn add_prefix(prefix: u8, slice: &[u8]) -> Vec<u8> {
    let mut result = vec![prefix];
    result.extend_from_slice(slice);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_signature() {
        let private_key = generate_signing_key();
        let public_key = get_pub_key(&private_key, true).unwrap();
        let data = sha256(b"some data");
        let signature = sign(&private_key, &data).unwrap();
        
        assert_eq!(verify_signature(&public_key, &signature, &data).unwrap(), true);
    }
}