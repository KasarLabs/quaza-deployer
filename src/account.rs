use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        types::{
            contract::{legacy::LegacyContractClass, CompiledClass, SierraClass},
            Call, DeclareTransactionResult, Felt,
        },
        utils::get_udc_deployed_address,
    },
    macros::selector,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};
use std::{error::Error, fs::File, sync::Arc};

pub async fn declare_v1(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    path: &str,
    no_fee: bool,
) -> Result<DeclareTransactionResult, Box<dyn Error>> {
    let contract_artifact: LegacyContractClass = serde_json::from_reader(File::open(path)?)?;

    let mut declare = account.declare_legacy(Arc::new(contract_artifact));

    if no_fee {
        declare = declare.max_fee(Felt::ZERO);
    }

    let result = declare.send().await?;
    Ok(result)
}

pub async fn declare_v2(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,
    path: &str,
    compiled_path: &str,
    no_fee: bool,
) -> Result<DeclareTransactionResult, Box<dyn Error>> {
    let contract_artifact: SierraClass = serde_json::from_reader(File::open(path)?)?;
    let compiled_class: CompiledClass = serde_json::from_reader(File::open(compiled_path)?)?;
    let compiled_class_hash = compiled_class.class_hash()?;
    let flattened_class = contract_artifact.flatten()?;

    let mut declare = account.declare_v2(Arc::new(flattened_class), compiled_class_hash);

    if no_fee {
        declare = declare.max_fee(Felt::ZERO);
    }

    let result = declare.send().await?;

    Ok(result)
}

pub async fn deploy(
    account: &SingleOwnerAccount<&JsonRpcClient<HttpTransport>, &LocalWallet>,

    class_hash: Felt,
    salt: Felt,
    constructor_calldata: &[Felt],
    no_fee: bool,
) -> Result<(Felt, Felt), Box<dyn Error>> {
    let mut calldata = vec![
        class_hash,
        salt,
        Felt::ONE,
        constructor_calldata.len().into(),
    ]; // deploy from zero
    calldata.extend_from_slice(constructor_calldata);

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

    let mut deploy = account.execute_v1(vec![call]);

    if no_fee {
        deploy = deploy.max_fee(Felt::ZERO);
    }

    let result = deploy.send().await?;

    Ok((result.transaction_hash, contract_address))
}
