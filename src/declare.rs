use std::error::Error;
use std::fs::File;

use serde_json::json;
use starknet::core::types::{
    contract::legacy::LegacyContractClass, CompressedLegacyContractClass, DeclareTransactionResult,
    Felt,
};

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BroadcastedDeclareTransactionV0 {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// The class to be declared
    pub contract_class: CompressedLegacyContractClass,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    result: T,
    id: u64,
}

pub async fn declare_v0(
    url: &str,
    path: &str,
    sender_address: Felt,
) -> Result<DeclareTransactionResult, Box<dyn Error>> {
    let contract_artifact: LegacyContractClass = serde_json::from_reader(File::open(path)?)?;
    let compressed_class = contract_artifact.compress()?;

    let tx = BroadcastedDeclareTransactionV0 {
        sender_address,
        max_fee: Felt::ZERO,
        signature: vec![],
        contract_class: compressed_class,
        is_query: false,
    };

    let body = &json!({
        "jsonrpc": "2.0",
        "method": "madara_addDeclareV0Transaction",
        "params": [tx],
        "id": 0
    });

    let client = reqwest::Client::new();
    client
        .post(url)
        .json(body)
        .send()
        .await?
        .json::<JsonRpcResponse<DeclareTransactionResult>>()
        .await
        .map(|res| res.result)
        .map_err(Into::into)
}
