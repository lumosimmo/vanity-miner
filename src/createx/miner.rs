use crate::createx::compute;
use crate::createx::config::{
    Create2Config, Create2Match, Create2Result, Create3Config, Create3Match, Create3Result,
};
use alloy_primitives::{Address, B256, keccak256};
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::{from_value, to_value};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_SEED: u128 = 1337;

/// Mines for a salt that produces a CREATE2 address satisfying the given predicate.
///
/// ## Arguments
/// * `config` - A `Create2Config` struct defining the mining parameters.
/// * `predicate` - A closure that returns true if the computed address is the desired one.
///
/// ## Returns
/// A `Create2Result` containing the found salts/addresses and total iterations.
pub fn mine_create2_salt(
    config: &Create2Config,
    predicate: &dyn Fn(Address) -> bool,
) -> Create2Result {
    let seed = config.seed.unwrap_or(DEFAULT_SEED).to_be_bytes();
    let mut salt = B256::from(keccak256(seed).0);
    let mut results = Vec::new();

    let mut total_iterations = 0;

    for i in 0..config.max_iterations {
        total_iterations = (i + 1) as usize;
        increment_b256_in_place(&mut salt);

        let computed_address =
            compute::create2_address(config.deployer, salt, config.init_code_hash);

        if predicate(computed_address) {
            results.push(Create2Match {
                salt,
                computed_address,
            });

            if results.len() >= config.max_results as usize {
                break;
            }
        }
    }

    Create2Result {
        results,
        total_iterations,
    }
}

/// Mines for a salt that produces a CREATE2 address with a specific prefix.
pub fn mine_create2_salt_with_prefix(config: &Create2Config, prefix: &[u8]) -> Create2Result {
    if prefix.len() > 20 {
        panic!("Prefix cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| addr.starts_with(prefix);
    mine_create2_salt(config, &predicate)
}

/// Mines for a salt that produces a CREATE2 address with a specific suffix.
pub fn mine_create2_salt_with_suffix(config: &Create2Config, suffix: &[u8]) -> Create2Result {
    if suffix.len() > 20 {
        panic!("Suffix cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| addr.ends_with(suffix);
    mine_create2_salt(config, &predicate)
}

/// Mines for a salt that produces a CREATE2 address containing a specific byte sequence.
pub fn mine_create2_salt_with_contains(config: &Create2Config, contains: &[u8]) -> Create2Result {
    if contains.len() > 20 {
        panic!("Contained sequence cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| {
        addr.windows(contains.len())
            .any(|window| window == contains)
    };
    mine_create2_salt(config, &predicate)
}

/// Mines for a salt that produces a CREATE3 address satisfying the given predicate.
///
/// ## Arguments
/// * `config` - A `Create3Config` struct defining the mining parameters.
/// * `predicate` - A closure that returns true if the computed address is the desired one.
///
/// ## Returns
/// A `Create3Result` containing the found salts/addresses and total iterations.
pub fn mine_create3_salt(
    config: &Create3Config,
    predicate: &dyn Fn(Address) -> bool,
) -> Create3Result {
    let mut salt_bytes = [0u8; 32];
    let mut results = Vec::new();

    // Permissioned protection
    if let Some(caller) = config.caller {
        salt_bytes[0..20].copy_from_slice(caller.as_slice());
    }

    // Cross-chain protection
    if config.chain_id.is_some() {
        salt_bytes[20] = 0x01;
    }

    let seed = config.seed.unwrap_or(DEFAULT_SEED);
    let mut total_iterations = 0;

    for i in 0..config.max_iterations {
        total_iterations = (i + 1) as usize;

        let current_random_numeric = seed.wrapping_add(i as u128);
        let random_part_bytes = current_random_numeric.to_be_bytes();

        salt_bytes[21..32].copy_from_slice(&random_part_bytes[5..]);

        let salt = B256::from(salt_bytes);
        let guarded_salt = compute::guarded_salt(salt, config.caller, config.chain_id);
        let computed_address = compute::create3_address(config.deployer, guarded_salt);

        if predicate(computed_address) {
            results.push(Create3Match {
                salt,
                guarded_salt,
                computed_address,
            });

            if results.len() >= config.max_results as usize {
                break;
            }
        }
    }

    Create3Result {
        results,
        total_iterations,
    }
}

/// Mines for a salt that produces a CREATE3 address with a specific prefix.
pub fn mine_create3_salt_with_prefix(config: &Create3Config, prefix: &[u8]) -> Create3Result {
    if prefix.len() > 20 {
        panic!("Prefix cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| addr.starts_with(prefix);
    mine_create3_salt(config, &predicate)
}

/// Mines for a salt that produces a CREATE3 address with a specific suffix.
pub fn mine_create3_salt_with_suffix(config: &Create3Config, suffix: &[u8]) -> Create3Result {
    if suffix.len() > 20 {
        panic!("Suffix cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| addr.ends_with(suffix);
    mine_create3_salt(config, &predicate)
}

/// Mines for a salt that produces a CREATE3 address containing a specific byte sequence.
pub fn mine_create3_salt_with_contains(config: &Create3Config, contains: &[u8]) -> Create3Result {
    if contains.len() > 20 {
        panic!("Contained sequence cannot be longer than 20 bytes");
    }
    let predicate = |addr: Address| {
        addr.windows(contains.len())
            .any(|window| window == contains)
    };
    mine_create3_salt(config, &predicate)
}

fn increment_b256_in_place(bytes: &mut B256) {
    for byte in bytes.iter_mut().rev() {
        let (result, carry) = byte.overflowing_add(1);
        *byte = result;
        if !carry {
            return;
        }
    }
}

// WASM wrapper functions
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create3_salt_with_prefix(
    config: JsValue,
    prefix: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create3Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create3_salt_with_prefix(&config, prefix);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create3_salt_with_suffix(
    config: JsValue,
    suffix: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create3Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create3_salt_with_suffix(&config, suffix);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create3_salt_with_contains(
    config: JsValue,
    contains: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create3Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create3_salt_with_contains(&config, contains);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create2_salt_with_prefix(
    config: JsValue,
    prefix: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create2Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create2_salt_with_prefix(&config, prefix);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create2_salt_with_suffix(
    config: JsValue,
    suffix: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create2Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create2_salt_with_suffix(&config, suffix);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_create2_salt_with_contains(
    config: JsValue,
    contains: &[u8],
) -> Result<JsValue, JsValue> {
    let config: Create2Config =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_create2_salt_with_contains(&config, contains);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::createx::compute;
    use crate::createx::config::Create3Config;
    use alloy_primitives::address;

    const DEPLOYER: Address = address!("ba5ed099633d3b313e4d5f7bdc1305d3c28ba5ed");
    const CALLER: Address = address!("DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF");

    #[test]
    fn test_mine_create3_for_leading_zeros() {
        let chain_id = 130u64;

        let config = Create3Config {
            deployer: DEPLOYER,
            caller: Some(CALLER),
            chain_id: Some(chain_id),
            max_iterations: 1_000_000,
            max_results: 1,
            seed: Some(1234),
        };

        let mining_result = mine_create3_salt(&config, &|addr| addr[0] == 0x00);
        assert!(
            !mining_result.results.is_empty(),
            "Failed to find address with leading zero byte within {} iterations",
            config.max_iterations
        );

        let result = &mining_result.results[0];

        assert_eq!(
            result.computed_address[0], 0x00,
            "Computed address should start with 0x00"
        );
        assert_eq!(
            &result.salt[0..20],
            CALLER.as_slice(),
            "Salt should contain caller address in bytes 0-19"
        );
        assert_eq!(
            result.salt[20], 0x01,
            "Salt byte 20 should be 0x01 for chain protection"
        );

        let expected_guarded_salt =
            compute::guarded_salt(result.salt, Some(CALLER), Some(chain_id));
        assert_eq!(
            result.guarded_salt, expected_guarded_salt,
            "Guarded salt computation mismatch"
        );

        let expected_address = compute::create3_address(DEPLOYER, result.guarded_salt);
        assert_eq!(
            result.computed_address, expected_address,
            "Address computation mismatch"
        );
    }

    #[test]
    fn test_mine_create3_for_specific_suffix() {
        let config = Create3Config {
            deployer: DEPLOYER,
            caller: None,
            chain_id: None,
            max_iterations: 1_000_000,
            max_results: 1,
            seed: Some(1234),
        };

        let suffix = &[0xba, 0xbe];
        let mining_result = mine_create3_salt_with_suffix(&config, suffix);
        assert!(
            !mining_result.results.is_empty(),
            "Failed to find address ending with 0xbabe within {} iterations",
            config.max_iterations
        );

        let result = &mining_result.results[0];

        assert!(
            result.computed_address.ends_with(suffix),
            "Address should end with suffix 0xbabe"
        );
        assert_eq!(
            &result.salt[0..20],
            Address::ZERO.as_slice(),
            "Salt bytes 0-19 should be zero (no caller protection)"
        );
        assert_eq!(
            result.salt[20], 0x00,
            "Salt byte 20 should be 0x00 (no chain protection)"
        );
    }

    #[test]
    fn test_mine_create3_for_specific_prefix_and_chain_id() {
        let chain_id: u64 = 130;

        let config = Create3Config {
            deployer: DEPLOYER,
            caller: Some(CALLER),
            chain_id: Some(chain_id),
            max_iterations: 1_000_000,
            max_results: 1,
            seed: Some(1234),
        };

        let prefix = &[0x27, 0x18];
        let mining_result = mine_create3_salt_with_prefix(&config, prefix);
        assert!(
            !mining_result.results.is_empty(),
            "Failed to find address starting with 0x2718 within {} iterations",
            config.max_iterations
        );

        let result = &mining_result.results[0];

        assert!(
            result.computed_address.starts_with(prefix),
            "Address should start with prefix 0x2718"
        );
        assert_eq!(
            &result.salt[0..20],
            CALLER.as_slice(),
            "Salt should contain caller address for permission protection"
        );
        assert_eq!(
            result.salt[20], 0x01,
            "Salt byte 20 should be 0x01 for cross-chain protection"
        );

        let expected_guarded_salt =
            compute::guarded_salt(result.salt, Some(CALLER), Some(chain_id));
        assert_eq!(
            result.guarded_salt, expected_guarded_salt,
            "Guarded salt should match expected computation"
        );

        let expected_address = compute::create3_address(DEPLOYER, result.guarded_salt);
        assert_eq!(
            result.computed_address, expected_address,
            "Final address should match expected computation"
        );
    }

    #[test]
    fn test_mine_create3_for_contains() {
        let config = Create3Config {
            deployer: DEPLOYER,
            caller: None,
            chain_id: None,
            max_iterations: 1_000_000,
            max_results: 1,
            seed: Some(1234),
        };

        let contains = &[0xab, 0xcd];
        let mining_result = mine_create3_salt_with_contains(&config, contains);
        assert!(
            !mining_result.results.is_empty(),
            "Failed to find address containing 0xabcd within {} iterations",
            config.max_iterations
        );

        let result = &mining_result.results[0];

        assert!(
            result
                .computed_address
                .windows(contains.len())
                .any(|window| window == contains),
            "Address should contain the sequence 0xabcd"
        );
        assert_eq!(
            &result.salt[0..20],
            Address::ZERO.as_slice(),
            "Salt should have no caller protection"
        );
        assert_eq!(
            result.salt[20], 0x00,
            "Salt should have no chain protection"
        );
    }

    #[test]
    fn test_mine_create3_multiple_results() {
        let config = Create3Config {
            deployer: DEPLOYER,
            caller: None,
            chain_id: None,
            max_iterations: 10_000_000,
            max_results: 3,
            seed: Some(1234),
        };

        let mining_result = mine_create3_salt(&config, &|addr| addr[0] < 0x10);

        assert!(
            !mining_result.results.is_empty(),
            "Should find at least one address with first byte < 0x10"
        );

        for (i, result) in mining_result.results.iter().enumerate() {
            assert!(
                result.computed_address[0] < 0x10,
                "Result {} address first byte should be < 0x10",
                i
            );
        }

        let mut unique_addresses = std::collections::HashSet::new();
        for (i, result) in mining_result.results.iter().enumerate() {
            assert!(
                unique_addresses.insert(result.computed_address),
                "Result {} has duplicate address: {}",
                i,
                result.computed_address
            );
        }
    }
}
