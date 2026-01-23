use crate::address::{generate_address, validate_address};
use crate::transaction::Transaction;
use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub struct KaspaWallet {
    secret_key: SecretKey,
    public_key: PublicKey,
    network_prefix: String,
}

impl KaspaWallet {
    pub fn new(secret_key: SecretKey) -> Result<Self> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Self {
            secret_key,
            public_key,
            network_prefix: "kaspa".to_string(), // Default to mainnet
        })
    }

    pub fn with_network(secret_key: SecretKey, network: &str) -> Result<Self> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Self {
            secret_key,
            public_key,
            network_prefix: network.to_string(),
        })
    }

    pub fn from_mnemonic(mnemonic: &str, _derivation_path: &str) -> Result<Self> {
        // This is a simplified implementation
        // In practice, you'd want to use a proper BIP39/BIP32 library
        let seed = Self::mnemonic_to_seed(mnemonic)?;
        let secret_key_bytes = &seed[..32];
        let secret_key = SecretKey::from_slice(secret_key_bytes)?;

        Self::new(secret_key)
    }

    fn mnemonic_to_seed(mnemonic: &str) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(mnemonic.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    pub fn get_address(&self) -> Result<String> {
        generate_address(&self.public_key, &self.network_prefix)
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    pub fn get_private_key(&self) -> String {
        hex::encode(self.secret_key.secret_bytes())
    }

    pub fn create_transaction(
        &self,
        inputs: Vec<(String, u32)>,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> Result<Transaction> {
        let mut tx = Transaction::new();

        for (txid, vout) in inputs {
            tx.add_input(txid, vout);
        }

        for (address, amount) in outputs {
            if !validate_address(&address)? {
                return Err(anyhow::anyhow!("Invalid address: {}", address));
            }
            tx.add_output(address, amount);
        }

        // Sign all inputs
        for i in 0..tx.inputs.len() {
            tx.sign_input(i, &self.secret_key, &self.public_key)?;
        }

        Ok(tx)
    }

    pub fn estimate_transaction_fee(
        &self,
        input_count: usize,
        output_count: usize,
        fee_rate: u64,
    ) -> u64 {
        let mut tx = Transaction::new();

        for _ in 0..input_count {
            tx.add_input("dummy".to_string(), 0);
        }

        for _ in 0..output_count {
            tx.add_output("dummy".to_string(), 0);
        }

        tx.estimate_fee(fee_rate)
    }

    pub fn validate_private_key(private_key_hex: &str) -> Result<bool> {
        let key_bytes = hex::decode(private_key_hex);
        if key_bytes.is_err() {
            return Ok(false);
        }

        let key_bytes = key_bytes?;
        if key_bytes.len() != 32 {
            return Ok(false);
        }

        Ok(SecretKey::from_slice(&key_bytes).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);

        let wallet = KaspaWallet::new(secret_key);
        assert!(wallet.is_ok());
    }

    #[test]
    fn test_address_generation() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);

        let wallet = KaspaWallet::new(secret_key).unwrap();
        let address = wallet.get_address();
        assert!(address.is_ok());

        let addr_str = address.unwrap();
        assert!(addr_str.starts_with("kaspa:"));
    }
}
