use anyhow::Result;
use bs58;
use ripemd::Ripemd160;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

pub struct KaspaAddress {
    pub address: String,
    pub public_key: PublicKey,
}

pub fn generate_address(public_key: &PublicKey, network_prefix: &str) -> Result<String> {
    let pubkey_bytes = public_key.serialize();

    let mut hasher = Sha256::new();
    hasher.update(&pubkey_bytes);
    let sha256_hash = hasher.finalize();

    let mut ripemd_hasher = Ripemd160::new();
    ripemd_hasher.update(sha256_hash);
    let pubkey_hash = ripemd_hasher.finalize();

    let mut payload = Vec::new();
    payload.push(0x00); // Version byte for P2PKH
    payload.extend_from_slice(&pubkey_hash);

    let checksum = compute_checksum(&payload);
    payload.extend_from_slice(&checksum);

    let address = bs58::encode(payload).into_string();
    let formatted_address = format!("{}:{}", network_prefix, address);

    Ok(formatted_address)
}

fn compute_checksum(payload: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let first_hash = hasher.finalize();

    hasher = Sha256::new();
    hasher.update(first_hash);
    let second_hash = hasher.finalize();

    second_hash[..4].to_vec()
}

pub fn validate_address(address: &str) -> Result<bool> {
    if !address.contains(':') {
        return Ok(false);
    }

    let parts: Vec<&str> = address.split(':').collect();
    if parts.len() != 2 {
        return Ok(false);
    }

    let encoded_part = parts[1];

    let decoded = bs58::decode(encoded_part).into_vec();
    if decoded.is_err() {
        return Ok(false);
    }

    let decoded_bytes = decoded?;
    if decoded_bytes.len() < 21 {
        return Ok(false);
    }

    let payload = &decoded_bytes[..decoded_bytes.len() - 4];
    let checksum = &decoded_bytes[decoded_bytes.len() - 4..];

    let expected_checksum = compute_checksum(payload);

    Ok(checksum == expected_checksum)
}
