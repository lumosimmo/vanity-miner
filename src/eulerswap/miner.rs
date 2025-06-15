use crate::eulerswap::compute::creation_code_meta_proxy;
use crate::eulerswap::config::{EulerSwapConfig, EulerSwapResult};
use crate::univ4_hook::{V4HookConfig, V4HookPermissions, mine_v4_hook_salt};
use alloy_primitives::keccak256;
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::{from_value, to_value};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const EULERSWAP_HOOK_PERMISSIONS: V4HookPermissions = V4HookPermissions {
    before_initialize: true,
    before_swap: true,
    before_swap_return_delta: true,
    before_donate: true,
    before_add_liquidity: true,

    after_initialize: false,
    after_add_liquidity: false,
    before_remove_liquidity: false,
    after_remove_liquidity: false,
    after_swap: false,
    after_donate: false,
    after_swap_return_delta: false,
    after_add_liquidity_return_delta: false,
    after_remove_liquidity_return_delta: false,
};

/// Mines for a salt that produces a Uniswap v4 hook address satisfying the given permission flags.
///
/// ## Arguments
/// * `config` - A `EulerSwapConfig` struct defining the mining parameters.
///
/// ## Returns
/// A `EulerSwapResult` containing the found salts/addresses and total iterations.
pub fn mine_eulerswap_salt(config: &EulerSwapConfig) -> EulerSwapResult {
    let pool_params = config.pool_params.abi_encode();
    let creation_code = creation_code_meta_proxy(config.eulerswap_impl, &pool_params);
    let init_code_hash = keccak256(creation_code);

    let v4_hook_config = V4HookConfig {
        deployer: config.factory,
        init_code_hash,
        max_iterations: config.max_iterations,
        max_results: config.max_results,
        seed: config.seed,
        permissions: EULERSWAP_HOOK_PERMISSIONS,
    };

    mine_v4_hook_salt(&v4_hook_config)
}

// WASM wrapper functions
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_eulerswap_salt(config: JsValue) -> Result<JsValue, JsValue> {
    let config: EulerSwapConfig =
        from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_eulerswap_salt(&config);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}
