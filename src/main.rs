mod account;
mod config;
mod declare;
mod deploy;
mod token;
mod utils;

use config::STRK_SALT;
use starknet::{
    accounts::{
        Account, AccountFactory, ConnectedAccount, ExecutionEncoding, OpenZeppelinAccountFactory,
        SingleOwnerAccount,
    },
    core::types::{
        BlockId, BlockTag, BlockWithTxHashes, Call, DeclareTransactionResult,
        DeployAccountTransactionResult, Felt, MaybePendingBlockWithTxHashes, U256,
    },
    macros::selector,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
    signers::{LocalWallet, Signer, SigningKey},
};
use utils::wait_for_confirmation;

#[tokio::main]
async fn main() {
    let config = config::load_config();
    let signing_key = SigningKey::from_secret_scalar(config.deployer_secret_key);
    let signer = LocalWallet::from(signing_key);
    let public_key = signer.get_public_key().await.unwrap().scalar();

    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.rpc_url).unwrap()));

    let chain_id = provider.chain_id().await.unwrap();
    println!(
        "Chain ID: {}",
        String::from_utf8_lossy(&chain_id.to_bytes_be())
    );

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: udc_class_hash,
    } = declare::declare_v0(
        &config.rpc_admin_url,
        "./contracts/UDC.json",
        Felt::from(0x01),
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("UDC declared with class hash: 0x{:x}", udc_class_hash);

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: oz_v0_deploy_class_hash,
    } = declare::declare_v0(
        &config.rpc_admin_url,
        "./contracts/DeployerAccountSepolia.json",
        Felt::from(0x01),
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "account V0 with deploy declared with class hash: 0x{:x}",
        oz_v0_deploy_class_hash
    );

    let account_factory =
        OpenZeppelinAccountFactory::new(oz_v0_deploy_class_hash, chain_id, &signer, &provider)
            .await
            .unwrap();
    let DeployAccountTransactionResult {
        transaction_hash: tx_hash,
        contract_address: oz_v0_deploy_address,
    } = account_factory
        .deploy_v1(Felt::from_hex_unchecked("0x01")) // salt
        .max_fee(Felt::ZERO)
        .nonce(Felt::ZERO)
        .send()
        .await
        .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Deployed account at address: 0x{:x}", oz_v0_deploy_address);

    let mut account = SingleOwnerAccount::new(
        &provider,
        &signer,
        oz_v0_deploy_address,
        chain_id,
        ExecutionEncoding::Legacy,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let (tx_hash, udc_address) = deploy::deploy(
        &account,
        udc_class_hash,
        Felt::from_hex_unchecked("0x00"), // salt
        &[],
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Deployed UDC at address: 0x{:x}", udc_address);

    // declare account class v1 for the deployer
    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: account_class_hash,
    } = account::declare_v2(
        &account,
        "./contracts/account/contract_class.json",
        "./contracts/account/compiled_contract_class.json",
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Account declared with class hash: 0x{:x}",
        account_class_hash
    );

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: eic_class_hash,
    } = account::declare_v2(
        &account,
        "./contracts/eic/contract_class.json",
        "./contracts/eic/compiled_contract_class.json",
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("EIC class declared with class hash: 0x{:x}", eic_class_hash);

    let account_factory =
        OpenZeppelinAccountFactory::new(account_class_hash, chain_id, &signer, &provider)
            .await
            .unwrap();
    let DeployAccountTransactionResult {
        transaction_hash: tx_hash,
        contract_address: account_address,
    } = account_factory
        .deploy_v1(Felt::from_hex_unchecked("0x01")) // salt
        .max_fee(Felt::ZERO)
        .nonce(Felt::ZERO)
        .send()
        .await
        .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Deployed account at address: 0x{:x}", account_address);
    let mut account = SingleOwnerAccount::new(
        &provider,
        &signer,
        account_address,
        chain_id,
        ExecutionEncoding::New,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: strk_class_hash,
    } = account::declare_v1(&account, "./contracts/StrkOrigin.json", true)
        .await
        .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Original STRK class declared with class hash: 0x{:x}",
        strk_class_hash
    );

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: token_class_hash,
    } = account::declare_v2(
        &account,
        "./contracts/token/contract_class.json",
        "./contracts/token/compiled_contract_class.json",
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Token declared with class hash: 0x{:x}", token_class_hash);

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: argent_class_hash,
    } = account::declare_v2(
        &account,
        "./contracts/argent/contract_class.json",
        "./contracts/argent/compiled_contract_class.json",
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Argent account declared with class hash: 0x{:x}",
        argent_class_hash
    );

    // deploy STRK with the same salt and class hash as the original STRK class to get the same address
    // it's necessary to deploy the STRK contract directly from the deployer account without UDC to be
    // governor of the STRK contract
    let (tx_hash, strk_token_address) =
        account::deploy(&account, strk_class_hash, STRK_SALT, &[Felt::ZERO], true)
            .await
            .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Token STRK deployed at address: 0x{:x}", strk_token_address);

    let calldata = vec![
        token_class_hash, // new class hash
        eic_class_hash,   // class hash of the updater storage
        Felt::from(0x00), // init vector length and data for EIC
        Felt::from(0x00), // is the final implementation
    ];
    let call = Call {
        to: strk_token_address,
        selector: selector!("add_implementation"),
        calldata: calldata.clone(),
    };
    let tx_hash = account
        .execute_v1(vec![call])
        .max_fee(Felt::ZERO)
        .send()
        .await
        .unwrap()
        .transaction_hash;
    wait_for_confirmation(&provider, tx_hash).await.unwrap();

    let call = Call {
        to: strk_token_address,
        selector: selector!("upgrade_to"),
        calldata,
    };
    let tx_hash = account
        .execute_v1(vec![call])
        .max_fee(Felt::ZERO)
        .send()
        .await
        .unwrap()
        .transaction_hash;
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("STRK class hash successfully updated");

    let (tx_hash, quaza_token_address) = token::deploy_token(
        &account,
        token_class_hash,
        "Quaza Token",
        "QUAZA",
        account_address,
        Felt::from_hex_unchecked("0x02"),
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Token QUAZA deployed at address: 0x{:x}",
        quaza_token_address
    );

    let one_token: U256 = U256::from(10u128.pow(18));

    let tx_hash = token::mint(
        &account,
        &strk_token_address,
        &account_address,
        &(U256::from(10u128) * one_token),
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Minted 10 STRK tokens");

    let tx_hash = token::mint(
        &account,
        &quaza_token_address,
        &account_address,
        &(U256::from(10u128) * one_token),
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Minted 10 QUAZA tokens");

    let (block_hash, state_root) = loop {
        match provider.get_block_with_tx_hashes(BlockId::Number(0)).await {
            Ok(MaybePendingBlockWithTxHashes::Block(BlockWithTxHashes {
                block_hash,
                new_root,
                ..
            })) => break (block_hash, new_root),
            _ => {
                println!("Waiting for block 0 finalization...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    };
    println!("Block 0 hash: 0x{:x}", block_hash);

    let DeclareTransactionResult {
        transaction_hash: tx_hash,
        class_hash: counter_class_hash,
    } = account::declare_v2(
        &account,
        "./contracts/counter/contract_class.json",
        "./contracts/counter/compiled_contract_class.json",
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Counter declared with class hash: 0x{:x}",
        counter_class_hash
    );
    let (tx_hash, counter_address) = account::deploy(
        &account,
        counter_class_hash,
        Felt::from_hex_unchecked("0x00"), // salt
        &[],
        true,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!("Counter deployed at address: 0x{:x}", counter_address);
    let call = Call {
        to: counter_address,
        selector: selector!("increment"),
        calldata: vec![],
    };
    let nonce = account.get_nonce().await.unwrap();
    for i in 0..10 {
        let tx_hash = account
            .execute_v1(vec![call.clone()])
            .max_fee(Felt::ZERO)
            .nonce(nonce + Felt::from(i))
            .send()
            .await
            .unwrap()
            .transaction_hash;
    }
    println!("Counter incremented 10 times");

    println!("Do you want to deploy the core contract? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "y" {
        return;
    }

    // deploy starknet core contract
    println!("Deploying core contract...");
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.rpc_starknet_url).unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();
    let mut account = SingleOwnerAccount::new(
        &provider,
        &signer,
        config.starknet_account_address,
        chain_id,
        ExecutionEncoding::New,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let core_contract_class_hash = Felt::from_hex_unchecked(
        "0x07e32e97ad7d1809358418ec553d61d0f537fba13d5b8ac3aa479ec9c632ef95",
    );
    let calldata = vec![
        config.starknet_account_address, // owner
        state_root,
        Felt::from(0x00), // block_number
        block_hash,
    ];
    let (tx_hash, core_contract_address) = deploy::deploy_v1(
        &account,
        core_contract_class_hash,
        Felt::from_hex_unchecked("0x00"), // salt
        &calldata,
        false,
    )
    .await
    .unwrap();
    wait_for_confirmation(&provider, tx_hash).await.unwrap();
    println!(
        "Core contract deployed at address: 0x{:x}",
        core_contract_address
    );
}
