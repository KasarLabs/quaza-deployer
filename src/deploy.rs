use starknet::{
    accounts::{Account, SingleOwnerAccount},
    contract::ContractFactory,
    core::{
        types::{Call, Felt},
        utils::get_udc_deployed_address,
    },
    macros::selector,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};
use std::error::Error;

pub async fn deploy(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    class_hash: Felt,
    salt: Felt,
    constructor_calldata: &[Felt],
) -> Result<(Felt, Felt), Box<dyn Error>> {
    let mut calldata = vec![class_hash, salt, constructor_calldata.len().into()];
    calldata.extend_from_slice(constructor_calldata);
    calldata.push(Felt::ONE); // deploy from zero

    let contract_address = get_udc_deployed_address(
        salt,
        class_hash,
        &starknet::core::utils::UdcUniqueness::NotUnique,
        constructor_calldata,
    );

    let call = Call {
        to: account.address(),
        selector: selector!("deploy_contract"),
        calldata,
    };

    let result = account
        .execute_v1(vec![call])
        .max_fee(Felt::ZERO)
        .send()
        .await?;

    Ok((result.transaction_hash, contract_address))
}

pub async fn deploy_v1(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    class_hash: Felt,
    salt: Felt,
    constructor_calldata: &[Felt],
    no_fee: bool,
) -> Result<(Felt, Felt), Box<dyn Error>> {
    let contract_address = get_udc_deployed_address(
        salt,
        class_hash,
        &starknet::core::utils::UdcUniqueness::NotUnique,
        &constructor_calldata,
    );

    let contract_factory = ContractFactory::new(class_hash, account);

    let mut deploy = contract_factory.deploy_v1(constructor_calldata.to_vec(), salt, false);

    if no_fee {
        deploy = deploy.max_fee(Felt::ZERO);
    }

    let result = deploy.send().await?;

    Ok((result.transaction_hash, contract_address))
}
