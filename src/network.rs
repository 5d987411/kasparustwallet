use crate::error::WalletResult;

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub prefix: String,
    pub rpc_url: String,
}

impl NetworkConfig {
    pub fn mainnet() -> Self {
        Self {
            name: "mainnet".to_string(),
            prefix: "kaspa".to_string(),
            rpc_url: "127.0.0.1:16110".to_string(),
        }
    }

    pub fn testnet10() -> Self {
        Self {
            name: "testnet-10".to_string(),
            prefix: "kaspa".to_string(),
            rpc_url: "127.0.0.1:16210".to_string(),
        }
    }

    pub fn testnet11() -> Self {
        Self {
            name: "testnet-11".to_string(),
            prefix: "kaspa".to_string(),
            rpc_url: "127.0.0.1:16310".to_string(),
        }
    }

    pub fn simnet() -> Self {
        Self {
            name: "simnet".to_string(),
            prefix: "kaspa".to_string(),
            rpc_url: "127.0.0.1:16410".to_string(),
        }
    }

    pub fn from_name(name: &str) -> WalletResult<Self> {
        match name.trim() {
            "mainnet" => Ok(Self::mainnet()),
            "testnet-10" => Ok(Self::testnet10()),
            "testnet-11" => Ok(Self::testnet11()),
            "testnet" => Ok(Self::testnet11()),
            "simnet" => Ok(Self::simnet()),
            _ => Err(crate::error::WalletError::Network(format!(
                "Unknown network: {}",
                name
            ))),
        }
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
}
