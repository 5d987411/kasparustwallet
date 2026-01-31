use crate::address::{generate_address, validate_address};
use crate::error::WalletError;
use crate::network::NetworkConfig;
use crate::transaction::Transaction;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub struct KaspaWallet {
    secret_key: SecretKey,
    public_key: PublicKey,
    network_config: NetworkConfig,
}

impl KaspaWallet {
    pub fn new(secret_key: SecretKey, network_config: NetworkConfig) -> Self {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Self {
            secret_key,
            public_key,
            network_config,
        }
    }

    pub fn with_network(secret_key: SecretKey, network: &str) -> Result<Self, WalletError> {
        let network_config = NetworkConfig::from_name(network)?;
        Ok(Self::new(secret_key, network_config))
    }

    pub fn get_address(&self) -> String {
        generate_address(&self.public_key, self.network_config.get_prefix())
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
        _fee_rate: u64,
    ) -> Result<Transaction, WalletError> {
        let mut tx = Transaction::new();

        for (txid, vout) in inputs {
            tx.add_input(txid, vout);
        }

        for (address, amount) in outputs {
            if !validate_address(&address)? {
                return Err(crate::error::WalletError::InvalidAddressFormat);
            }
            tx.add_output(address, amount);
        }

        for i in 0..tx.inputs.len() {
            tx.sign_input(i, &self.secret_key, &self.public_key)?;
        }

        Ok(tx)
    }

    pub fn estimate_transaction_fee(input_count: usize, output_count: usize, fee_rate: u64) -> u64 {
        let mut tx = Transaction::new();

        for _ in 0..input_count {
            tx.add_input("dummy".to_string(), 0);
        }

        for _ in 0..output_count {
            tx.add_output("dummy".to_string(), 0);
        }

        tx.estimate_fee(fee_rate)
    }

    pub fn validate_private_key(private_key_hex: &str) -> bool {
        let key_bytes = hex::decode(private_key_hex);
        if key_bytes.is_err() {
            return false;
        }

        let key_bytes = match key_bytes {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        if key_bytes.len() != 32 {
            return false;
        }

        SecretKey::from_slice(&key_bytes).is_ok()
    }

    pub fn get_network_name(&self) -> &str {
        &self.network_config.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);

        let network_config = NetworkConfig::mainnet();
        let wallet = KaspaWallet::new(secret_key, network_config);
        assert!(!wallet.get_address().is_empty());
    }

    #[test]
    fn test_address_generation() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);

        let network_config = NetworkConfig::mainnet();
        let wallet = KaspaWallet::new(secret_key, network_config);
        let address = wallet.get_address();

        assert!(address.starts_with("kaspa:"));
    }

    #[test]
    fn test_private_key_validation() {
        assert!(!KaspaWallet::validate_private_key("invalid"));
        assert!(!KaspaWallet::validate_private_key(""));
        assert!(!KaspaWallet::validate_private_key("123"));
        assert!(KaspaWallet::validate_private_key(
            "0000000000000000000000000000000000000000000000000000000000000001"
        ));
    }
}
