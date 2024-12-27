use std::str::FromStr;

use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, RootProvider},
    transports::http::Http,
};
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

use crate::{
    error::AppError,
    ENSRegistry::{self, ENSRegistryInstance},
    EnsAppArgs,
};

/// Environment variables
#[derive(Deserialize)]
pub struct Env {
    rpc_url_mainnet: String,
    ens_contract_address: String,
}

/// App data
pub struct AppData {
    pub env: Env,
    pub args: EnsAppArgs,
}

/// App
pub struct App {
    pub ens_client: ENSRegistryInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl App {
    pub fn new(env: Env) -> Result<Self, AppError> {
        let ens_client = Self::build_ens_client(&env.rpc_url_mainnet, &env.ens_contract_address)?;

        Ok(Self { ens_client })
    }

    /// Builds the ENS client
    fn build_ens_client(
        rpc_url: &str,
        contract_address: &str,
    ) -> Result<ENSRegistryInstance<Http<Client>, RootProvider<Http<Client>>>, AppError> {
        let provider =
            ProviderBuilder::new().on_http(rpc_url.parse().map_err(|_e| AppError::InvalidUrl())?);

        let alloy_contract = ENSRegistry::new(
            Address::from_str(contract_address)
                .map_err(|e| AppError::AddressParse(e.to_string()))?,
            provider.clone(),
        );

        Ok(alloy_contract)
    }

    /// Initializes the app
    pub fn init() -> Result<AppData, AppError> {
        // Read the .env file from the current directory or parents
        dotenvy::dotenv().ok();
        // Parse the env vars
        let env = envy::from_env::<Env>()?;
        // Parse the CLI args
        let args = EnsAppArgs::parse();
        Ok(AppData { env, args })
    }
}
