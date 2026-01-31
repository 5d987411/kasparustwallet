use crate::error::WalletError;
use bs58;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

pub fn generate_address(public_key: &PublicKey, network_prefix: &str) -> String {
    let pubkey_bytes = public_key.serialize();

    let mut hasher = Sha256::new();
    hasher.update(&pubkey_bytes);
    let sha256_hash = hasher.finalize();

    let mut digest = ripemd::Ripemd160::new();
    digest.update(&sha256_hash);
    let pubkey_hash = digest.finalize();

    let mut payload = Vec::new();
    payload.push(0x00);
    payload.extend_from_slice(&pubkey_hash);

    let checksum = compute_checksum(&payload);
    payload.extend_from_slice(&checksum);

    let address = bs58::encode(payload).into_string();
    format!("{}:{}", network_prefix, address)
}

fn compute_checksum(payload: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let first_hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(first_hash);
    let second_hash = hasher.finalize();

    second_hash[..4].to_vec()
}

pub fn validate_address(address: &str) -> Result<bool, WalletError> {
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

    let decoded_bytes = match decoded {
        Ok(bytes) => bytes,
        Err(_) => return Ok(false),
    };

    if decoded_bytes.len() < 21 {
        return Ok(false);
    }

    let payload = &decoded_bytes[..decoded_bytes.len() - 4];
    let checksum = &decoded_bytes[decoded_bytes.len() - 4..];

    let expected_checksum = compute_checksum(payload);

    Ok(checksum == expected_checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::Secp256k1;

    #[test]
    fn test_address_generation() {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rngs::OsRng);

        let address = generate_address(&public_key, "kaspa");
        assert!(address.starts_with("kaspa:"));
    }

    #[test]
    fn test_address_validation() {
        assert!(validate_address("kaspa:abc").is_ok());
        assert_eq!(validate_address("kaspa:abc").unwrap(), false);
    }
}
