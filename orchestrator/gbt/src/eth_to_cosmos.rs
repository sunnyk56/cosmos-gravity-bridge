use crate::utils::TIMEOUT;
use clap::ArgMatches;
use clarity::Address as EthAddress;
use clarity::PrivateKey as EthPrivateKey;
use clarity::Uint256;
use cosmos_gravity::send::{send_request_batch, send_to_eth};
use deep_space::address::Address as CosmosAddress;
use deep_space::{coin::Coin, private_key::PrivateKey as CosmosPrivateKey};
use env_logger::Env;
use ethereum_gravity::deploy_erc20::deploy_erc20;
use ethereum_gravity::send_to_cosmos::send_to_cosmos;
use gravity_proto::gravity::QueryDenomToErc20Request;
use gravity_utils::connection_prep::{check_for_eth, check_for_fee_denom, create_rpc_connections};
use std::process::exit;
use std::time::Instant;
use std::{time::Duration, u128};
use tokio::time::sleep as delay_for;

use crate::utils::fraction_to_exponent;

pub async fn eth_to_cosmos(input: &&ArgMatches) {
    let web3 = connections.web3.unwrap();
    let cosmos_dest: CosmosAddress = args.flag_cosmos_destination.parse().unwrap();
    let ethereum_public_key = ethereum_key.to_public_key().unwrap();
    check_for_eth(ethereum_public_key, &web3).await;

    let res = web3
        .get_erc20_decimals(erc20_address, ethereum_public_key)
        .await
        .expect("Failed to query ERC20 contract");
    let decimals: u8 = res.to_string().parse().unwrap();
    let amount = fraction_to_exponent(args.flag_amount.unwrap(), decimals);

    let erc20_balance = web3
        .get_erc20_balance(erc20_address, ethereum_public_key)
        .await
        .expect("Failed to get balance, check ERC20 contract address");

    if erc20_balance == 0u8.into() {
        panic!(
            "You have zero {} tokens, please double check your sender and erc20 addresses!",
            gravity_address
        );
    } else if amount.clone() > erc20_balance {
        panic!("Insufficient balance {} > {}", amount, erc20_balance);
    }

    info!(
        "Sending {} to Cosmos from {} to {}",
        amount, erc20_address, ethereum_public_key, cosmos_dest
    );
    // we send some erc20 tokens to the gravity contract to register a deposit
    let res = send_to_cosmos(
        erc20_address,
        gravity_address,
        amount.clone(),
        cosmos_dest,
        ethereum_key,
        Some(TIMEOUT),
        &web3,
        vec![],
    )
    .await;
    match res {
        Ok(tx_id) => println!("Send to Cosmos txid: {:#066x}", tx_id),
        Err(e) => println!("Failed to send tokens! {:?}", e),
    }
}
