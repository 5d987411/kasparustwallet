use crate::error::WalletError;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub txid: String,
    pub vout: u32,
    pub signature: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }

    pub fn add_input(&mut self, txid: String, vout: u32) {
        self.inputs.push(TxInput {
            txid,
            vout,
            signature: None,
            public_key: None,
        });
    }

    pub fn add_output(&mut self, address: String, amount: u64) {
        self.outputs.push(TxOutput { address, amount });
    }

    pub fn serialize(&self) -> Result<Vec<u8>, WalletError> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&self.version.to_le_bytes());

        buffer.push(self.inputs.len() as u8);
        for input in &self.inputs {
            let txid_bytes = hex::decode(&input.txid)?;
            buffer.extend_from_slice(&txid_bytes);
            buffer.extend_from_slice(&input.vout.to_le_bytes());
        }

        buffer.push(self.outputs.len() as u8);
        for output in &self.outputs {
            let address_bytes = output.address.as_bytes();
            buffer.push(address_bytes.len() as u8);
            buffer.extend_from_slice(address_bytes);
            buffer.extend_from_slice(&output.amount.to_le_bytes());
        }

        buffer.extend_from_slice(&self.lock_time.to_le_bytes());

        Ok(buffer)
    }

    pub fn get_signature_hash(&self, input_index: usize) -> Result<Vec<u8>, WalletError> {
        let mut tx_copy = self.clone();

        for (i, input) in tx_copy.inputs.iter_mut().enumerate() {
            if i == input_index {
                input.signature = None;
                input.public_key = None;
            } else {
                input.signature = Some("dummy".to_string());
                input.public_key = Some("dummy".to_string());
            }
        }

        let serialized = tx_copy.serialize()?;

        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        Ok(hasher.finalize().to_vec())
    }

    pub fn sign_input(
        &mut self,
        input_index: usize,
        secret_key: &SecretKey,
        public_key: &PublicKey,
    ) -> Result<(), WalletError> {
        if input_index >= self.inputs.len() {
            return Err(crate::error::WalletError::Transaction(
                "Input index out of bounds".to_string(),
            ));
        }

        let signature_hash = self.get_signature_hash(input_index)?;
        let message = Message::from_digest_slice(&signature_hash)?;

        let secp = Secp256k1::new();
        let signature = secp.sign_ecdsa(&message, secret_key);

        self.inputs[input_index].signature = Some(hex::encode(signature.serialize_der()));
        self.inputs[input_index].public_key = Some(hex::encode(public_key.serialize()));

        Ok(())
    }

    pub fn estimate_fee(&self, fee_rate: u64) -> u64 {
        let base_size = 10;
        let input_size = 32 + 4 + 73 + 33;
        let output_size = 8 + 1 + 34;

        let total_size =
            base_size + (self.inputs.len() * input_size) + (self.outputs.len() * output_size);
        (total_size as u64 + 999) / 1000 * fee_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new();
        assert_eq!(tx.version, 1);
        assert!(tx.inputs.is_empty());
        assert!(tx.outputs.is_empty());
    }

    #[test]
    fn test_add_input_output() {
        let mut tx = Transaction::new();
        tx.add_input("abc123".to_string(), 0);
        tx.add_output("kaspa:xyz".to_string(), 1000);

        assert_eq!(tx.inputs.len(), 1);
        assert_eq!(tx.outputs.len(), 1);
    }
}
