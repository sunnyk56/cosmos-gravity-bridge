//! This is the testing module for relay market functionality.
//! This is where relayers utilize web30 to interact with a testnet to obtain coin swap values
//! and determine whether relays should happen or not
use std::str::FromStr;

use crate::utils::{ValidatorKeys, send_one_eth, start_orchestrators};
use clarity::{Address as EthAddress, Uint256};
use deep_space::Contact;
use gravity_proto::gravity::query_client::QueryClient as GravityQueryClient;
use tonic::transport::Channel;
use web30::client::Web3;
use web30::amm::{DAI_CONTRACT_ADDRESS, WETH_CONTRACT_ADDRESS, UNISWAP_QUOTER_ADDRESS};

pub async fn relay_market_test(
    _web30: &Web3,
    _grpc_client: GravityQueryClient<Channel>,
    _contact: &Contact,
    keys: Vec<ValidatorKeys>,
    gravity_address: EthAddress,
) {
    // Start Relayer
    // Get price from uniswap
    // Test valset relay happy path
    let address_with_funds = EthAddress::parse_and_validate("0x1D4dAEEffd706043bc5C6b8616d5d78A33Eaed84").unwrap();
    send_one_eth(address_with_funds, _web30).await;
    let fee = 500u32;
    info!("fee: {}", fee);
    let amount = Uint256::from_str("1000000000000000000").unwrap();
    let weth_in_dai = _web30.get_uniswap_price(address_with_funds.clone(), *WETH_CONTRACT_ADDRESS,
        *DAI_CONTRACT_ADDRESS, fee.clone().into(), amount.clone(),
        0u32.into(), None).await;
    if weth_in_dai.is_err() {
        info!("Error getting swap price from uniswap: {:?}", weth_in_dai.as_ref().err());
    }
    let weth_in_dai = weth_in_dai.unwrap();
    print!("{:?} weth is worth {} dai less fees", amount, weth_in_dai);
    let fee = 3000u32;
    let weth_in_dai2 = _web30.get_uniswap_price(address_with_funds.clone(), *WETH_CONTRACT_ADDRESS,
        *DAI_CONTRACT_ADDRESS, fee.clone().into(), amount.clone(),
        0u32.into(), None).await;
    if weth_in_dai2.is_err() {
        info!("Error getting swap price from uniswap: {:?}", weth_in_dai2.as_ref().err());
    }
    let weth_in_dai2 = weth_in_dai2.unwrap();
    assert!(weth_in_dai2 > 1u32.into());
    print!("{:?} weth is worth {} dai less fees", amount, weth_in_dai2);
}

#[test]
pub fn relay_market_local() {
    use actix::System;
    use env_logger::{Builder, Env};
    use std::time::Duration;
    use crate::{one_eth, send_one_eth};
    Builder::from_env(Env::default().default_filter_or("debug")).init(); // Change to debug for logs
    let runner = System::new();
    let web3 = Web3::new("http://localhost:8545", crate::OPERATION_TIMEOUT);
    let caller_address =
        EthAddress::parse_and_validate("0x5A0b54D5dc17e0AadC383d2db43B0a0D3E029c4c").unwrap();
    let amount = one_eth();
    let fee = Uint256::from(500u16);
    let sqrt_price_limit_x96_uint160 = Uint256::from(0u16);

    runner.block_on(async move {
        debug!("Test");
        send_one_eth(caller_address, &web3).await;
        let price = web3
            .get_uniswap_price(
                caller_address,
                *WETH_CONTRACT_ADDRESS,
                *DAI_CONTRACT_ADDRESS,
                fee.clone(),
                amount.clone(),
                sqrt_price_limit_x96_uint160.clone(),
                None,
            )
            .await;
        if price.is_err() {
            panic!("Error getting price: {:?}", price.err());
        }
        let weth2dai = price.unwrap();
        debug!("weth->dai price is {}", weth2dai);
        assert!(weth2dai > 0u32.into());
        let price = web3
            .get_uniswap_price(
                caller_address,
                *DAI_CONTRACT_ADDRESS,
                *WETH_CONTRACT_ADDRESS,
                fee.clone(),
                weth2dai,
                sqrt_price_limit_x96_uint160,
                None,
            )
            .await;
        let dai2weth = price.unwrap();
        debug!("dai->weth price is {}", &dai2weth);
        let amount_float: f64 = (amount.to_string()).parse().unwrap();
        let dai2weth_float: f64 = (dai2weth.to_string()).parse().unwrap();
        // If we were to swap, we should get within 5% back what we originally put in to account for slippage and fees
        assert!((0.95 * amount_float) < dai2weth_float && dai2weth_float < (1.05 * amount_float));
    });
}