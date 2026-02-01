use crate::address::validate_address;
use crate::error::WalletError;
use crate::wallet::KaspaWallet;
use iced::widget::{button, column, pick_list, row, text, text_input, Column, Container};
use iced::{Element, Length};
use secp256k1::SecretKey;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Send,
    Receive,
}

#[derive(Debug, Clone)]
pub enum Message {
    PrivateKeyInput(String),
    NetworkSelected(NetworkOption),
    CreateWallet,
    LoadWallet,
    RecipientInput(String),
    AmountInput(String),
    AddOutput,
    RemoveOutput(usize),
    ClearOutputs,
    SendTransaction,
    GenerateAddress,
    ValidateAddressInput(String),
    PasteFromClipboard,
    TabSelected(Tab),
    CopyAddress,
    CopyPublicKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkOption {
    Mainnet,
    Testnet10,
    Testnet11,
    Simnet,
}

impl fmt::Display for NetworkOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkOption::Mainnet => write!(f, "Mainnet"),
            NetworkOption::Testnet10 => write!(f, "Testnet-10"),
            NetworkOption::Testnet11 => write!(f, "Testnet-11"),
            NetworkOption::Simnet => write!(f, "Simnet"),
        }
    }
}

impl NetworkOption {
    fn to_str(&self) -> &'static str {
        match self {
            NetworkOption::Mainnet => "mainnet",
            NetworkOption::Testnet10 => "testnet-10",
            NetworkOption::Testnet11 => "testnet-11",
            NetworkOption::Simnet => "simnet",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputRow {
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
pub struct WalletGui {
    private_key: String,
    network: NetworkOption,
    wallet: Option<KaspaGuiWallet>,
    current_tab: Tab,
    recipient: String,
    amount: String,
    outputs: Vec<OutputRow>,
    status_message: String,
    address_result: String,
    public_key_result: String,
    validate_address_input: String,
    validation_result: String,
    copy_address_text: String,
    copy_public_key_text: String,
}

#[derive(Debug, Clone)]
pub struct KaspaGuiWallet {
    pub address: String,
    pub public_key: String,
    pub network_name: String,
}

impl WalletGui {
    fn new() -> Self {
        Self {
            private_key: String::new(),
            network: NetworkOption::Mainnet,
            wallet: None,
            current_tab: Tab::Send,
            recipient: String::new(),
            amount: String::new(),
            outputs: Vec::new(),
            status_message: String::new(),
            address_result: String::new(),
            public_key_result: String::new(),
            validate_address_input: String::new(),
            validation_result: String::new(),
            copy_address_text: String::new(),
            copy_public_key_text: String::new(),
        }
    }

    fn load_wallet(&mut self) -> Result<(), WalletError> {
        let secret_key_bytes = hex::decode(&self.private_key)?;
        let secret_key = SecretKey::from_slice(&secret_key_bytes)?;
        let wallet = KaspaWallet::with_network(secret_key, self.network.to_str())?;
        self.wallet = Some(KaspaGuiWallet {
            address: wallet.get_address(),
            public_key: wallet.get_public_key(),
            network_name: wallet.get_network_name().to_string(),
        });
        Ok(())
    }
}

fn get_clipboard_text() -> Option<String> {
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("xclip -selection clipboard -o 2>/dev/null")
        .output()
    {
        if output.status.success() {
            if let Ok(text) = String::from_utf8(output.stdout) {
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
    }

    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("xsel --clipboard --output 2>/dev/null")
        .output()
    {
        if output.status.success() {
            if let Ok(text) = String::from_utf8(output.stdout) {
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
    }

    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(text) = clipboard.get_text() {
            let trimmed = text.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }

    None
}

fn set_clipboard_text(text: &str) -> bool {
    if std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo -n '{}' | xclip -selection clipboard",
            text.replace("'", "'\\''")
        ))
        .status()
        .is_ok()
    {
        return true;
    }

    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if clipboard.set_text(text).is_ok() {
            return true;
        }
    }

    if std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo -n '{}' | xsel --clipboard --input",
            text.replace("'", "'\\''")
        ))
        .status()
        .is_ok()
    {
        return true;
    }

    false
}

pub fn run_gui() -> Result<(), iced::Error> {
    let settings = iced::Settings {
        antialiasing: true,
        ..iced::Settings::default()
    };

    iced::application(WalletGui::new, update, view)
        .settings(settings)
        .run()
}

fn update(state: &mut WalletGui, message: Message) {
    match message {
        Message::PrivateKeyInput(key) => {
            state.private_key = key;
            if KaspaWallet::validate_private_key(&state.private_key) {
                state.status_message = "Private key is valid".to_string();
            } else if state.private_key.len() == 64 {
                state.status_message = "Invalid private key format".to_string();
            } else {
                state.status_message = String::new();
            }
        }
        Message::NetworkSelected(network) => {
            state.network = network;
        }
        Message::CreateWallet => {
            let secp = secp256k1::Secp256k1::new();
            let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
            let wallet = KaspaWallet::with_network(secret_key, state.network.to_str()).unwrap();
            state.private_key = wallet.get_private_key();
            state.wallet = Some(KaspaGuiWallet {
                address: wallet.get_address(),
                public_key: wallet.get_public_key(),
                network_name: wallet.get_network_name().to_string(),
            });
            state.address_result = wallet.get_address();
            state.public_key_result = wallet.get_public_key();
            state.copy_address_text = wallet.get_address();
            state.copy_public_key_text = format!("kaspa:pk:{}", wallet.get_public_key());
            state.status_message = "New wallet created! Address generated.".to_string();
        }
        Message::LoadWallet => match state.load_wallet() {
            Ok(_) => {
                if let Some(ref wallet) = state.wallet {
                    state.address_result = wallet.address.clone();
                    state.public_key_result = wallet.public_key.clone();
                    state.copy_address_text = wallet.address.clone();
                    state.copy_public_key_text = format!("kaspa:pk:{}", wallet.public_key.clone());
                }
                state.status_message = "Wallet loaded successfully!".to_string();
            }
            Err(e) => {
                state.status_message = format!("Error loading wallet: {}", e);
            }
        },
        Message::SendTransaction => {
            if state.outputs.is_empty() {
                state.status_message = "No outputs to send".to_string();
                return;
            }
            let parsed_outputs: Result<Vec<(String, u64)>, _> = state
                .outputs
                .iter()
                .map(|o| {
                    let amount_kas: f64 = o.amount.parse().unwrap_or(0.0);
                    let amount_sompi = (amount_kas * 100_000_000.0) as u64;
                    Ok::<(String, u64), ()>((o.address.clone(), amount_sompi))
                })
                .collect();
            match parsed_outputs {
                Ok(outputs) => {
                    if let Err(e) = state.load_wallet() {
                        state.status_message = format!("Error: {}", e);
                        return;
                    }
                    let secret_key_bytes = hex::decode(&state.private_key).unwrap();
                    let secret_key = SecretKey::from_slice(&secret_key_bytes).unwrap();
                    let wallet =
                        KaspaWallet::with_network(secret_key, state.network.to_str()).unwrap();
                    match wallet.create_transaction(vec![], outputs, 1000) {
                        Ok(tx) => {
                            let serialized = tx.serialize().unwrap();
                            state.status_message =
                                format!("Transaction created: {}", hex::encode(&serialized));
                        }
                        Err(e) => {
                            state.status_message = format!("Transaction error: {}", e);
                        }
                    }
                }
                Err(_) => {
                    state.status_message = "Invalid output format".to_string();
                }
            }
        }
        Message::RecipientInput(addr) => {
            state.recipient = addr;
        }
        Message::AmountInput(amt) => {
            state.amount = amt;
        }
        Message::AddOutput => {
            if !state.recipient.is_empty() && !state.amount.is_empty() {
                if validate_address(&state.recipient).unwrap_or(false) {
                    match state.amount.parse::<f64>() {
                        Ok(amount_kas) if amount_kas > 0.0 => {
                            let amount_sompi = (amount_kas * 100_000_000.0) as u64;
                            state.outputs.push(OutputRow {
                                address: state.recipient.clone(),
                                amount: format!("{} KAS ({})", amount_kas, amount_sompi),
                            });
                            state.recipient.clear();
                            state.amount.clear();
                            state.status_message = format!(
                                "Added output {} ({} outputs total)",
                                state.outputs.len() - 1,
                                state.outputs.len()
                            );
                        }
                        Ok(_) => {
                            state.status_message = "Amount must be greater than 0".to_string();
                        }
                        Err(_) => {
                            state.status_message =
                                "Invalid amount format. Use decimal (e.g., 1.5)".to_string();
                        }
                    }
                } else {
                    state.status_message = "Invalid recipient address".to_string();
                }
            } else {
                state.status_message = "Enter recipient and amount".to_string();
            }
        }
        Message::RemoveOutput(idx) => {
            if idx < state.outputs.len() {
                state.outputs.remove(idx);
                state.status_message =
                    format!("Removed output ({} remaining)", state.outputs.len());
            }
        }
        Message::ClearOutputs => {
            state.outputs.clear();
            state.status_message = "Outputs cleared".to_string();
        }
        Message::GenerateAddress => {
            let secp = secp256k1::Secp256k1::new();
            let (secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
            let wallet = KaspaWallet::with_network(secret_key, state.network.to_str()).unwrap();
            state.private_key = wallet.get_private_key();
            state.wallet = Some(KaspaGuiWallet {
                address: wallet.get_address(),
                public_key: wallet.get_public_key(),
                network_name: wallet.get_network_name().to_string(),
            });
            state.address_result = wallet.get_address();
            state.public_key_result = wallet.get_public_key();
            state.copy_address_text = wallet.get_address();
            state.copy_public_key_text = format!("kaspa:pk:{}", wallet.get_public_key());
            state.status_message = "New wallet generated! Save your private key.".to_string();
        }
        Message::ValidateAddressInput(addr) => {
            state.validate_address_input = addr.clone();
            match validate_address(&addr) {
                Ok(true) => state.validation_result = "Valid Kaspa address".to_string(),
                Ok(false) => state.validation_result = "Invalid address format".to_string(),
                Err(e) => state.validation_result = format!("Error: {}", e),
            }
        }
        Message::PasteFromClipboard => {
            if let Some(text) = get_clipboard_text() {
                state.validate_address_input = text.clone();
                match validate_address(&text) {
                    Ok(true) => state.validation_result = "Valid Kaspa address".to_string(),
                    Ok(false) => state.validation_result = "Invalid address format".to_string(),
                    Err(e) => state.validation_result = format!("Error: {}", e),
                }
                state.status_message = "Pasted from clipboard!".to_string();
            } else {
                state.status_message = "Could not access clipboard".to_string();
            }
        }
        Message::CopyAddress => {
            if state.copy_address_text.is_empty() {
                state.status_message = "No address to copy".to_string();
            } else if set_clipboard_text(&state.copy_address_text) {
                state.status_message = "Address copied to clipboard!".to_string();
            } else {
                state.status_message = "Copy failed".to_string();
            }
        }
        Message::CopyPublicKey => {
            if state.copy_public_key_text.is_empty() {
                state.status_message = "No public key to copy".to_string();
            } else {
                if set_clipboard_text(&state.copy_public_key_text) {
                    state.status_message = "Public key copied to clipboard!".to_string();
                } else {
                    state.status_message = "Copy failed".to_string();
                }
            }
        }
        Message::TabSelected(tab) => {
            state.current_tab = tab;
        }
    }
}

fn view(state: &WalletGui) -> Element<Message> {
    let networks = vec![
        NetworkOption::Mainnet,
        NetworkOption::Testnet10,
        NetworkOption::Testnet11,
        NetworkOption::Simnet,
    ];

    let wallet_info = if let Some(wallet) = &state.wallet {
        let pk_with_prefix = format!("kaspa:pk:{}", &wallet.public_key);
        let pk_display = pk_with_prefix.clone();
        column![
            text("Wallet Information").size(20),
            text("Address:").size(14),
            row![
                text(&wallet.address).size(14).width(Length::Fill),
                button("Copy").on_press(Message::CopyAddress),
            ],
            text("Network:").size(14),
            text(&wallet.network_name).size(14),
            text("Public Key:").size(14),
            row![
                text(pk_display).size(12).width(Length::Fill),
                button("Copy").on_press(Message::CopyPublicKey),
            ],
        ]
    } else {
        column![
            text("No wallet loaded").size(20),
            text("Create a new wallet or load an existing one below").size(14),
        ]
    };

    let settings_info = column![
        text("Wallet Settings").size(20),
        text("Network:").size(14),
        pick_list(
            networks,
            Some(state.network.clone()),
            Message::NetworkSelected
        ),
        text("Private Key:").size(14),
        text_input("Enter private key (hex)", &state.private_key)
            .on_input(Message::PrivateKeyInput),
        row![
            button("Load Wallet").on_press(Message::LoadWallet),
            button("Create New Wallet").on_press(Message::CreateWallet),
        ]
        .spacing(10),
        text("Warning: Never share your private key!").size(12),
    ];

    let combined_section = column![wallet_info, text("---").size(12), settings_info,].spacing(15);

    let tab_row = row![
        button("Send")
            .on_press(Message::TabSelected(Tab::Send))
            .style(if state.current_tab == Tab::Send {
                button::primary
            } else {
                button::secondary
            }),
        button("Receive")
            .on_press(Message::TabSelected(Tab::Receive))
            .style(if state.current_tab == Tab::Receive {
                button::primary
            } else {
                button::secondary
            }),
    ]
    .spacing(10);

    let content: Column<Message> = match state.current_tab {
        Tab::Send => view_send(state),
        Tab::Receive => view_receive(state),
    };

    let status_bar = if !state.status_message.is_empty() {
        text(&state.status_message).size(14)
    } else {
        text("")
    };

    Container::new(
        column![
            text("KaspaRustWallet").size(24),
            combined_section,
            tab_row,
            content,
            status_bar,
        ]
        .spacing(20)
        .padding(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view_send(state: &WalletGui) -> Column<Message> {
    let outputs_list: Column<Message> = if state.outputs.is_empty() {
        column![text("No outputs added yet")]
    } else {
        state
            .outputs
            .iter()
            .enumerate()
            .fold(column![], |col, (idx, output)| {
                col.push(
                    row![
                        text(format!("{}: {}", idx, output.address)),
                        text(&output.amount),
                        button("Remove").on_press(Message::RemoveOutput(idx)),
                    ]
                    .spacing(10),
                )
            })
    };

    column![
        text("Send Transaction").size(20),
        text("Add recipients:").size(14),
        row![
            text_input("Recipient address", &state.recipient)
                .on_input(Message::RecipientInput)
                .width(Length::Fill),
            text_input("Amount (KAS)", &state.amount)
                .on_input(Message::AmountInput)
                .width(Length::Fill),
            button("Add").on_press(Message::AddOutput),
        ]
        .spacing(10),
        outputs_list,
        row![
            button("Send Transaction").on_press(Message::SendTransaction),
            button("Clear All").on_press(Message::ClearOutputs),
        ]
        .spacing(10),
        text("Note: Amount is in KAS. 1 KAS = 100,000,000 sompi").size(12),
    ]
}

fn view_receive(state: &WalletGui) -> Column<Message> {
    let current_address = if !state.address_result.is_empty() {
        state.address_result.clone()
    } else {
        state
            .wallet
            .as_ref()
            .map(|w| w.address.clone())
            .unwrap_or_default()
    };

    let current_public_key = if !state.public_key_result.is_empty() {
        state.public_key_result.clone()
    } else {
        state
            .wallet
            .as_ref()
            .map(|w| w.public_key.clone())
            .unwrap_or_default()
    };

    let address_section = if !current_address.is_empty() {
        let addr = current_address.clone();
        column![
            text("Address:").size(14),
            row![
                text(addr).size(14).width(Length::Fill),
                button("Copy").on_press(Message::CopyAddress),
            ],
        ]
    } else {
        column![text("No address available. Create or load a wallet first.").size(14)]
    };

    let pk_section = if !current_public_key.is_empty() {
        let _pk = current_public_key.clone();
        let pk_with_prefix = format!("kaspa:pk:{}", current_public_key);
        column![
            text("Public Key:").size(14),
            row![
                text(pk_with_prefix).size(12).width(Length::Fill),
                button("Copy").on_press(Message::CopyPublicKey),
            ],
        ]
    } else {
        column![]
    };

    column![
        text("Receive").size(20),
        button("Generate New Address").on_press(Message::GenerateAddress),
        address_section,
        pk_section,
        text("Validate Address:").size(14),
        row![
            text_input(
                "Paste address here to validate",
                &state.validate_address_input
            )
            .on_input(Message::ValidateAddressInput)
            .width(Length::Fill),
            button("Paste").on_press(Message::PasteFromClipboard),
        ],
        if !state.validation_result.is_empty() {
            text(&state.validation_result).size(14)
        } else {
            text("")
        },
    ]
}
