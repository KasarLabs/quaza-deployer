use starknet::{
    core::types::{Felt, TransactionExecutionStatus, TransactionStatus},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};

pub async fn wait_for_confirmation(
    provider: &JsonRpcClient<HttpTransport>,
    tx_hash: Felt,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match provider.get_transaction_status(tx_hash).await {
            Ok(TransactionStatus::AcceptedOnL2(TransactionExecutionStatus::Succeeded))
            | Ok(TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Succeeded)) => {
                return Ok(());
            }
            Ok(TransactionStatus::AcceptedOnL2(TransactionExecutionStatus::Reverted))
            | Ok(TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Reverted)) => {
                return Err(format!("Transaction reverted").into());
            }
            Ok(TransactionStatus::Rejected) => {
                return Err(format!("Transaction rejected").into());
            }
            Err(_) | Ok(TransactionStatus::Received) => {
                println!("Waiting for transaction to be processed...");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
