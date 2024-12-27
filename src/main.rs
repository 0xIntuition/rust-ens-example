#![allow(clippy::result_large_err)]
use std::str::FromStr;

use alloy::{primitives::Address, sol};
use app::App;
use clap::Parser;
use ens::Ens;
use error::AppError;
use serde::{Deserialize, Serialize};

mod app;
mod ens;
mod error;

// Codegen to interact with the ENS contract.
sol!(
    #[derive(Debug, Deserialize, Serialize)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface ENSRegistry {
        function resolver(bytes32 node) external view returns (address);
    }
);

// Codegen to interact with the ENSName contract.
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface ENSName {
        function name(bytes32 node) external view returns (string);
    }
}

/// CLI arguments for the ENS app
#[derive(Parser, Clone, Debug)]
pub struct EnsAppArgs {
    #[arg(short, long)]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_data = App::init()?;
    let app = App::new(app_data.env)?;
    let ens = Ens::get_ens(Address::from_str(&app_data.args.address)?, &app.ens_client).await?;
    if let Some(name) = &ens.name {
        println!("ENS name: {}", name);
    }
    if let Some(image) = &ens.image {
        println!("ENS image: {}", image);
    }
    Ok(())
}
