use alloy::{
    primitives::{keccak256, Address, FixedBytes},
    providers::RootProvider,
    transports::http::Http,
};
use log::info;
use reqwest::Client;

use crate::{error::AppError, ENSName::ENSNameInstance, ENSRegistry::ENSRegistryInstance};

/// This struct represents the ENS name and avatar for an address.
#[derive(Clone, Debug)]
pub struct Ens {
    pub name: Option<String>,
    pub image: Option<String>,
}

impl Ens {
    /// This function hashes the ENS name.
    fn namehash(name: &str) -> Vec<u8> {
        if name.is_empty() {
            return vec![0u8; 32];
        }
        let mut hash = vec![0u8; 32];
        for label in name.rsplit('.') {
            hash.append(&mut keccak256(label.as_bytes()).to_vec());
            hash = keccak256(hash.as_slice()).to_vec();
        }
        hash
    }

    /// This function gets the ENS name and avatar for an address.
    pub async fn get_ens(
        address: Address,
        mainnet_client: &ENSRegistryInstance<Http<Client>, RootProvider<Http<Client>>>,
    ) -> Result<Ens, AppError> {
        let name = Self::get_ens_name(address, mainnet_client).await?;
        let mut image = None;
        if let Some(name_str) = &name {
            image = Self::get_ens_avatar(name_str).await?;
        }
        Ok(Ens { name, image })
    }

    /// Gets the ENS avatar URL for a given name
    async fn get_ens_avatar(name: &str) -> Result<Option<String>, AppError> {
        let url = format!("https://metadata.ens.domains/mainnet/avatar/{}", name);
        match reqwest::get(&url).await {
            Ok(response) => {
                if response.status() == 200 {
                    Ok(Some(url))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// This function gets the ENS name for an address.
    pub async fn get_ens_name(
        address: Address,
        mainnet_client: &ENSRegistryInstance<Http<Client>, RootProvider<Http<Client>>>,
    ) -> Result<Option<String>, AppError> {
        info!("Getting ENS name for {}", address);
        let address_hash = Self::namehash(&Self::prepare_name(address));
        let resolver_address =
            Self::get_resolver_address(address, &address_hash, mainnet_client).await?;

        if resolver_address != Address::ZERO {
            let alloy_contract = ENSNameInstance::new(resolver_address, mainnet_client.provider());
            let name = alloy_contract
                .name(FixedBytes::from_slice(address_hash.as_slice()))
                .call()
                .await?
                ._0;
            info!("ResolvedENS name: {:?}", name);
            Ok(Some(name))
        } else {
            Ok(None)
        }
    }

    /// This function gets the resolver address for an address hash.
    async fn get_resolver_address(
        address: Address,
        address_hash: &[u8],
        mainnet_client: &ENSRegistryInstance<Http<Client>, RootProvider<Http<Client>>>,
    ) -> Result<Address, AppError> {
        let resolver_address = mainnet_client
            .resolver(FixedBytes::from_slice(address_hash))
            .call()
            .await?
            ._0;

        if resolver_address == Address::ZERO {
            info!("No resolver found for {}", address);
        } else {
            info!("Resolver found for {}: {}", address, resolver_address);
        }

        Ok(resolver_address)
    }

    /// This function prepares the name for the ENS resolver.
    fn prepare_name(address: Address) -> String {
        let addr_str = address.to_string().to_lowercase();
        format!("{}.addr.reverse", addr_str.trim_start_matches("0x"))
    }
}
