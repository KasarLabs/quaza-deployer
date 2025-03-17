use starknet::{
    accounts::{Account, SingleOwnerAccount},
    contract::ContractFactory,
    core::{
        codec::Encode,
        types::{ByteArray, Call, Felt, U256},
        utils::get_udc_deployed_address,
    },
    macros::selector,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use std::error::Error;

pub fn constructor_call_data(name: &str, symbol: &str, owner: Felt) -> Vec<Felt> {
    let mut constructor_calldata = vec![];
    ByteArray::from(name)
        .encode(&mut constructor_calldata)
        .unwrap();
    ByteArray::from(symbol)
        .encode(&mut constructor_calldata)
        .unwrap();
    constructor_calldata.push(owner);

    constructor_calldata
}

pub async fn deploy_token(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    class_hash: Felt,
    name: &str,
    symbol: &str,
    owner: Felt,
    salt: Felt,
) -> Result<(Felt, Felt), Box<dyn Error>> {
    let constructor_calldata = constructor_call_data(name, symbol, owner);

    let contract_address = get_udc_deployed_address(
        salt,
        class_hash,
        &starknet::core::utils::UdcUniqueness::NotUnique,
        &constructor_calldata,
    );

    let contract_factory = ContractFactory::new(class_hash, account);

    let deployment = contract_factory
        .deploy_v1(constructor_calldata, salt, false)
        .max_fee(Felt::ZERO)
        .send()
        .await?;

    Ok((deployment.transaction_hash, contract_address))
}

pub async fn mint(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    token_address: &Felt,
    recipient: &Felt,
    amount: &U256,
) -> Result<Felt, Box<dyn Error>> {
    let calldata = vec![*recipient, amount.low().into(), amount.high().into()];

    let call = Call {
        to: *token_address,
        selector: selector!("mint"),
        calldata,
    };

    let mint_tx = account
        .execute_v1(vec![call])
        .max_fee(Felt::ZERO)
        .send()
        .await?;

    Ok(mint_tx.transaction_hash)
}

pub async fn transfer(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    token_address: &Felt,
    recipient: &Felt,
    amount: &U256,
) -> Result<Felt, Box<dyn Error>> {
    let calldata = vec![*recipient, amount.low().into(), amount.high().into()];

    let call = Call {
        to: *token_address,
        selector: selector!("transfer"),
        calldata,
    };

    let transfer_tx = account
        .execute_v1(vec![call])
        .max_fee(Felt::ZERO)
        .send()
        .await?;

    Ok(transfer_tx.transaction_hash)
}
