use anyhow::Result;
use clap::{Parser, Subcommand};
use secp256k1::{Secp256k1, SecretKey};
use std::fs;
use std::io::{self, Write};

mod address;
mod transaction;
mod wallet;

use crate::wallet::KaspaWallet;

#[derive(Parser)]
#[command(name = "kasparustwallet")]
#[command(about = "A Kaspa cryptocurrency wallet CLI", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    New {
        #[arg(short, long, default_value = "mainnet")]
        network: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Show wallet information
    Info {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "mainnet")]
        network: String,
    },
    /// Generate a new address
    Address {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "mainnet")]
        network: String,
    },
    /// Create a transaction
    Send {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "mainnet")]
        network: String,
        #[arg(short, long)]
        inputs: Vec<String>,
        #[arg(short, long)]
        outputs: Vec<String>,
        #[arg(short, long, default_value = "1000")]
        fee_rate: u64,
    },
    /// Estimate transaction fee
    EstimateFee {
        #[arg(short, long)]
        inputs: usize,
        #[arg(short, long)]
        outputs: usize,
        #[arg(short, long, default_value = "1000")]
        fee_rate: u64,
    },
    /// Validate an address
    ValidateAddress {
        #[arg(short, long)]
        address: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { network, output } => create_new_wallet(&network, output)?,
        Commands::Info {
            private_key,
            network,
        } => show_wallet_info(&private_key, &network)?,
        Commands::Address {
            private_key,
            network,
        } => generate_address(&private_key, &network)?,
        Commands::Send {
            private_key,
            network,
            inputs,
            outputs,
            fee_rate,
        } => create_transaction(&private_key, &network, inputs, outputs, fee_rate)?,
        Commands::EstimateFee {
            inputs,
            outputs,
            fee_rate,
        } => estimate_fee(inputs, outputs, fee_rate)?,
        Commands::ValidateAddress { address } => validate_address(&address)?,
    }

    Ok(())
}

fn create_new_wallet(network: &str, output: Option<String>) -> Result<()> {
    let secp = Secp256k1::new();
    let (secret_key, _public_key) = secp.generate_keypair(&mut rand::rngs::OsRng);

    let wallet = KaspaWallet::with_network(secret_key, network)?;

    let wallet_info = format!(
        "Network: {}\n\
         Private Key: {}\n\
         Public Key: {}\n\
         Address: {}\n",
        network,
        wallet.get_private_key(),
        wallet.get_public_key(),
        wallet.get_address()?
    );

    if let Some(output_path) = output {
        fs::write(&output_path, wallet_info)?;
        println!("Wallet created and saved to {}", output_path);
    } else {
        println!("New Wallet Created:");
        println!("==================");
        print!("{}", wallet_info);

        print!("Save this information securely. Press Enter to continue...");
        io::stdout().flush()?;
        let _ = io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn show_wallet_info(private_key: &str, network: &str) -> Result<()> {
    let secret_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let wallet = KaspaWallet::with_network(secret_key, network)?;

    println!("Wallet Information:");
    println!("==================");
    println!("Network: {}", network);
    println!("Private Key: {}", wallet.get_private_key());
    println!("Public Key: {}", wallet.get_public_key());
    println!("Address: {}", wallet.get_address()?);

    Ok(())
}

fn generate_address(private_key: &str, network: &str) -> Result<()> {
    let secret_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let wallet = KaspaWallet::with_network(secret_key, network)?;

    println!("Generated Address:");
    println!("==================");
    println!("{}", wallet.get_address()?);

    Ok(())
}

fn create_transaction(
    private_key: &str,
    network: &str,
    inputs: Vec<String>,
    outputs: Vec<String>,
    fee_rate: u64,
) -> Result<()> {
    let secret_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let wallet = KaspaWallet::with_network(secret_key, network)?;

    let parsed_inputs: Result<Vec<(String, u32)>> = inputs
        .iter()
        .map(|input| {
            let parts: Vec<&str> = input.split(':').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid input format: {}", input));
            }
            Ok((parts[0].to_string(), parts[1].parse()?))
        })
        .collect();

    let parsed_outputs: Result<Vec<(String, u64)>> = outputs
        .iter()
        .map(|output| {
            let parts: Vec<&str> = output.split(':').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid output format: {}", output));
            }
            Ok((parts[0].to_string(), parts[1].parse()?))
        })
        .collect();

    let transaction = wallet.create_transaction(parsed_inputs?, parsed_outputs?, fee_rate)?;

    println!("Transaction Created:");
    println!("==================");
    println!("Version: {}", transaction.version);
    println!("Inputs:");
    for (i, input) in transaction.inputs.iter().enumerate() {
        println!(
            "  {}: {}:{} (signed: {})",
            i,
            input.txid,
            input.vout,
            input.signature.is_some()
        );
    }
    println!("Outputs:");
    for (i, output) in transaction.outputs.iter().enumerate() {
        println!("  {}: {} ({} sompi)", i, output.address, output.amount);
    }

    let serialized = transaction.serialize()?;
    println!("Serialized: {}", hex::encode(&serialized));

    Ok(())
}

fn estimate_fee(inputs: usize, outputs: usize, fee_rate: u64) -> Result<()> {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);

    let wallet = KaspaWallet::new(secret_key)?;
    let fee = wallet.estimate_transaction_fee(inputs, outputs, fee_rate);

    println!("Estimated Fee:");
    println!("==============");
    println!("Inputs: {}", inputs);
    println!("Outputs: {}", outputs);
    println!("Fee Rate: {} sompkB", fee_rate);
    println!("Total Fee: {} sompi", fee);

    Ok(())
}

fn validate_address(address: &str) -> Result<()> {
    let is_valid = crate::address::validate_address(address)?;

    println!("Address Validation:");
    println!("==================");
    println!("Address: {}", address);
    println!("Valid: {}", is_valid);

    Ok(())
}
