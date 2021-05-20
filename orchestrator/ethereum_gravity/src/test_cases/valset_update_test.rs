//! This entire file is an enormous encoding test for validator set updates
//! and signature encoding specifically

#[cfg(test)]
mod tests {
    use std::fs::{self, read_to_string};

    use clarity::utils::hex_str_to_bytes;
    use clarity::PrivateKey;
    use clarity::{abi::encode_tokens, utils::bytes_to_hex_str};
    use gravity_utils::{
        message_signatures::encode_valset_confirm,
        types::{Valset, ValsetConfirmResponse, ValsetMember},
    };

    use crate::{utils::encode_valset_struct, valset_update::encode_valset_update_payload};

    #[test]
    fn decode_large_valset() {
        let some_cosmos_address = "althea1hv0dcr9l2l090jxtuxu8nsa2jm6h307xcmf4sw"
            .parse()
            .unwrap();
        let gravity_id = "foo";
        let str = read_to_string("test_files/valset_update_rlp").unwrap();
        let known_good_bytes = hex_str_to_bytes(&str).unwrap();
        let private_keys = read_to_string("test_files/ethers_test_privkeys").unwrap();
        let powers_str = read_to_string("test_files/ethers_test_powers").unwrap();
        let mut keys: Vec<PrivateKey> = Vec::new();
        for line in private_keys.lines() {
            keys.push(line.trim().parse().unwrap());
        }
        let mut powers: Vec<u64> = Vec::new();
        for line in powers_str.lines() {
            powers.push(line.trim().parse().unwrap());
        }

        let mut members0 = Vec::new();
        for (key, power) in keys.iter().zip(powers.iter()) {
            members0.push(ValsetMember {
                power: *power,
                eth_address: Some(key.to_public_key().unwrap()),
            });
            println!("{}", key.to_public_key().unwrap());
        }
        let valset0 = Valset {
            nonce: 0,
            members: members0,
            reward_amount: 0u8.into(),
            reward_token: None,
        };
        powers[0] -= 3;
        powers[1] += 3;
        let mut members1 = Vec::new();
        for (key, power) in keys.iter().zip(powers) {
            members1.push(ValsetMember {
                power,
                eth_address: Some(key.to_public_key().unwrap()),
            });
        }
        let valset1 = Valset {
            nonce: 1,
            members: members1,
            reward_amount: 0u8.into(),
            reward_token: None,
        };

        let mut confirms = Vec::new();
        for key in keys {
            let message = encode_valset_confirm(gravity_id.to_string(), valset1.clone());
            let eth_signature = key.sign_ethereum_msg(&message);
            confirms.push(ValsetConfirmResponse {
                orchestrator: some_cosmos_address,
                eth_address: key.to_public_key().unwrap(),
                nonce: 1u8.into(),
                eth_signature,
            })
        }

        let encoded_update_bytes =
            encode_valset_update_payload(valset1, valset0, &confirms, gravity_id.to_string())
                .unwrap();

        let bad = debug_print_data(&encoded_update_bytes);
        fs::write("/tmp/bad-encoding", bad).expect("Unable to write file");
        let good = debug_print_data(&known_good_bytes);
        fs::write("/tmp/good-encoding", good).expect("Unable to write file");

        // assert_eq!(
        //     bytes_to_hex_str(&encoded_update_bytes),
        //     bytes_to_hex_str(&known_good_bytes)
        // )
    }

    /// Function used for debug printing hex dumps
    /// of ethereum events with each uint256 on a new
    /// line
    fn debug_print_data(input: &[u8]) -> String {
        let mut out = String::new();
        let count = input.len() / 32;
        out += "data hex dump\n";
        for i in 0..count {
            out += &format!(
                "0x{}\n",
                bytes_to_hex_str(&input[(i * 32)..((i * 32) + 32)])
            )
        }
        out += "end hex dump\n";
        out
    }
}
