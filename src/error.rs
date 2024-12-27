use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to parse address: {0}")]
    AddressParse(String),
    #[error("Failed to parse address: {0}")]
    AlloyContractError(#[from] alloy::contract::Error),
    #[error(transparent)]
    Env(#[from] envy::Error),
    #[error("Failed to parse address: {0}")]
    FromHexError(#[from] alloy::hex::FromHexError),
    #[error("Invalid URL")]
    InvalidUrl(),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
