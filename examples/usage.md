# Example Usage

This document demonstrates how to use the Kaspa wallet functionality programmatically.

## Basic Wallet Operations

```rust
use anyhow::Result;
use secp256k1::{Secp256k1};
use kaspa_wallet::KaspaWallet;

fn main() -> Result<()> {
    // Create a new wallet
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
    
    let wallet = KaspaWallet::new(secret_key)?;
    
    // Get wallet information
    println!("Private Key: {}", wallet.get_private_key());
    println!("Public Key: {}", wallet.get_public_key());
    println!("Address: {}", wallet.get_address()?);
    
    Ok(())
}
```

## Transaction Creation

```rust
use kaspa_wallet::KaspaWallet;
use secp256k1::{Secp256k1};

fn create_payment_transaction() -> anyhow::Result<()> {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
    
    let wallet = KaspaWallet::new(secret_key)?;
    
    // Define inputs (previous UTXOs)
    let inputs = vec![
        ("abc123def456789abc123def456789abc123def456789abc123def456789abc123".to_string(), 0), // txid:vout
    ];
    
    // Define outputs (destination addresses and amounts)
    let outputs = vec![
        ("kaspa:qqpet37fwqlql7q4jczr7zj7qp5ylps2r2c0ynz6jjf368sdjnztufeghvc9x".to_string(), 587700), // 0.00587700 KAS
    ];
    
    // Create transaction with fee rate of 1000 sompkB
    let transaction = wallet.create_transaction(inputs, outputs, 1000)?;
    
    println!("Transaction created successfully!");
    println!("Inputs: {}", transaction.inputs.len());
    println!("Outputs: {}", transaction.outputs.len());
    
    // Get serialized transaction for broadcasting
    let serialized = transaction.serialize()?;
    println!("Serialized: {}", hex::encode(&serialized));
    
    Ok(())
}
```

## CLI Usage Examples

### Create a new wallet
```bash
./target/release/kaspa-wallet new --network mainnet --output wallet.txt
```

### Show wallet information
```bash
./target/release/kaspa-wallet info \
    --private-key "157dd2f579945a35f83c3ab78d8815c3148f3db3bfc284519a3e57a08c9aa2c8" \
    --network mainnet
```

### Generate address
```bash
./target/release/kaspa-wallet address \
    --private-key "157dd2f579945a35f83c3ab78d8815c3148f3db3bfc284519a3e57a08c9aa2c8" \
    --network mainnet
```

### Create transaction
```bash
./target/release/kaspa-wallet send \
    --private-key "157dd2f579945a35f83c3ab78d8815c3148f3db3bfc284519a3e57a08c9aa2c8" \
    --network mainnet \
    --inputs "abc123def456789abc123def456789abc123def456789abc123def456789abc123:0" \
    --outputs "kaspa:qqpet37fwqlql7q4jczr7zj7qp5ylps2r2c0ynz6jjf368sdjnztufeghvc9x:587700" \
    --fee-rate 1000
```

### Estimate fee
```bash
./target/release/kaspa-wallet estimate-fee --inputs 2 --outputs 1 --fee-rate 1000
```

### Validate address
```bash
./target/release/kaspa-wallet validate-address \
    --address "kaspa:qqpet37fwqlql7q4jczr7zj7qp5ylps2r2c0ynz6jjf368sdjnztufeghvc9x"
```

## Testing the Implementation

Run the test suite:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```