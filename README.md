# Kaspa Rust Wallet (kasparustwallet)

A Rust implementation of a Kaspa cryptocurrency wallet with CLI and GUI interfaces, inspired by the official [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) project.

## Features

### CLI Features
- ✅ Generate new Kaspa wallets (private/public key pairs)
- ✅ Generate Kaspa addresses (P2PKH format)
- ✅ Create and sign transactions
- ✅ Estimate transaction fees
- ✅ Validate addresses
- ✅ Support for different Kaspa networks (mainnet, testnet, etc.)

### GUI Features
- ✅ Modern native GUI using Iced framework
- ✅ Wallet creation and loading
- ✅ View wallet information (address, public key)
- ✅ Generate new addresses
- ✅ Copy address/public key to clipboard
- ✅ Validate addresses with paste support
- ✅ Support for Mainnet, Testnet-10, Testnet-11, and Simnet
- ✅ Multi-tab interface (Overview, Send, Receive, Settings)

## Installation

### Prerequisites

- Rust 1.70+
- Git
- For Linux: xclip (for clipboard support)
  ```bash
  sudo apt install xclip
  ```

### Build from source

```bash
git clone https://github.com/5d987411/kasparustwallet.git
cd kasparustwallet
cargo build --release
```

The compiled binary will be available at `target/release/kasparustwallet`.

## Usage

### GUI Mode (Recommended)

Launch the graphical wallet interface:

```bash
./kasparustwallet gui
```

#### GUI Features:

**Overview Tab**
- View wallet address and public key
- Copy address/public key with one click

**Send Tab**
- Add multiple transaction recipients
- Specify amount in sompi (1 KAS = 100,000,000 sompi)
- Create and sign transactions

**Receive Tab**
- Generate new wallet addresses
- Copy address and public key
- Validate any Kaspa address using the paste button

**Settings Tab**
- Select network (Mainnet/Testnet-10/Testnet-11/Simnet)
- Load existing wallet by private key
- Create new wallet

### CLI Mode

#### Create a new wallet
```bash
./kasparustwallet new --network mainnet

# Create wallet for testnet
./kasparustwallet new --network testnet-10

# Save wallet info to file
./kasparustwallet new --network mainnet --output wallet.txt
```

#### Show wallet information
```bash
./kasparustwallet info --private-key <private_key_hex> --network mainnet
```

#### Generate address
```bash
./kasparustwallet address --private-key <private_key_hex> --network mainnet
```

#### Create a transaction
```bash
./kasparustwallet send \
    --private-key <private_key_hex> \
    --network mainnet \
    --inputs <txid:vout> \
    --outputs <address:amount> \
    --fee-rate 1000
```

Example:
```bash
./kasparustwallet send \
    --private-key d636a23d4f49fe4e0d59fcf7a6c2ab3846ff2d3a54007b3817a11dff770d06ff \
    --network mainnet \
    --inputs "abc123...:0" "def456...:1" \
    --outputs "kaspa:qqpet37fwqlql7q4jczr7zj7qp5ylps2r2c0ynz6jjf368sdjnztufeghvc9x:587700" \
    --fee-rate 1000
```

#### Estimate transaction fee
```bash
./kasparustwallet estimate-fee --inputs 2 --outputs 1 --fee-rate 1000
```

#### Validate address
```bash
./kasparustwallet validate-address --address "kaspa:qqpet37fwqlql7q4jczr7zj7qp5ylps2r2c0ynz6jjf368sdjnztufeghvc9x"
```

## API Usage

### Creating a Wallet Programmatically

```rust
use anyhow::Result;
use secp256k1::{Secp256k1};
use kaspa_wallet::KaspaWallet;

fn main() -> Result<()> {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
    
    let wallet = KaspaWallet::new(secret_key)?;
    
    println!("Address: {}", wallet.get_address()?);
    println!("Private Key: {}", wallet.get_private_key());
    println!("Public Key: {}", wallet.get_public_key());
    
    Ok(())
}
```

### Creating a Transaction

```rust
use kaspa_wallet::KaspaWallet;
use secp256k1::{Secp256k1};

fn main() -> anyhow::Result<()> {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
    
    let wallet = KaspaWallet::new(secret_key)?;
    
    // Inputs: (txid, vout)
    let inputs = vec![
        ("abc123def456789...".to_string(), 0),
    ];
    
    // Outputs: (address, amount in sompi)
    let outputs = vec![
        ("kaspa:address...".to_string(), 587700), // 0.00587700 KAS
    ];
    
    let transaction = wallet.create_transaction(inputs, outputs, 1000)?;
    
    println!("Transaction created with {} inputs and {} outputs", 
             transaction.inputs.len(), transaction.outputs.len());
    
    Ok(())
}
```

## Address Format

Kaspa addresses use the following format:
```
kaspa:<base58_encoded_address>
```

The address generation follows these steps:
1. Take the public key (33 bytes for compressed format)
2. SHA-256 hash of the public key
3. RIPEMD-160 hash of the SHA-256 result
4. Add version byte (0x00 for P2PKH)
5. Double SHA-256 checksum
6. Base58 encoding with network prefix

## Transaction Details

- **Amount unit**: Transactions use `sompi` as the base unit (1 KAS = 100,000,000 sompi)
- **Fee rate**: Measured in `sompkB` (sompi per kilobyte)
- **Signature**: Uses ECDSA with the secp256k1 curve
- **Serialization**: Custom binary format for network transmission

## Security Notes

- **Private keys**: Never share your private key with anyone
- **Mnemonic support**: Basic mnemonic support is included (simplified implementation)
- **Network separation**: Ensure you use the correct network prefix to avoid sending funds to wrong network
- **Transaction validation**: Always verify transaction details before signing

## Networks

Supported networks:
- `mainnet` - Main Kaspa network
- `testnet-10` - Kaspa testnet (post-Crescendo)
- `testnet-11` - Kaspa testnet (future)
- Custom network prefixes can be used as needed

## Dependencies

- `secp256k1` - Elliptic curve cryptography
- `sha2` - SHA-256 hashing
- `ripemd` - RIPEMD-160 hashing  
- `bs58` - Base58 encoding/decoding
- `serde` - JSON serialization
- `clap` - Command line argument parsing
- `thiserror` - Error handling
- `hex` - Hex encoding/decoding
- `rand` - Cryptographic random number generation
- `iced` - GUI framework
- `arboard` - Cross-platform clipboard support

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This software is provided "as-is" for educational and development purposes. Use at your own risk. Always test thoroughly before using with real funds.

## Related Projects

- [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) - Official Kaspa implementation in Rust
- [kaspad](https://github.com/kaspanet/kaspad) - Kaspa node implementation in Go
- [Kaspa Documentation](https://docs.kaspa.org/) - Official Kaspa documentation