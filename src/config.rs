use dotenv::dotenv;
use serde::Deserialize;
use starknet::core::types::Felt;
use std::env;

pub const STRK_SALT: Felt =
    Felt::from_hex_unchecked("0x048a38cb46716a7cc3a7b5132309388f298bc49c53f5de377bb5401d877b7f89");

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_admin_url: String,
    pub rpc_starknet_url: String,

    pub deployer_secret_key: Felt,
    pub starknet_account_address: Felt,
}

pub fn load_config() -> Config {
    dotenv().ok();

    Config {
        rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
        rpc_admin_url: env::var("RPC_ADMIN_URL").expect("RPC_ADMIN_URL must be set"),
        rpc_starknet_url: env::var("RPC_STARKNET_URL").expect("RPC_STARKNET_URL must be set"),

        deployer_secret_key: Felt::from_hex_unchecked(
            &env::var("DEPLOYER_SECRET_KEY").expect("DEPLOYER_SECRET_KEY must be set"),
        ),

        starknet_account_address: Felt::from_hex_unchecked(
            &env::var("STARKNET_ACCOUNT_ADDRESS").expect("STARKNET_ACCOUNT_ADDRESS must be set"),
        ),
    }
}
