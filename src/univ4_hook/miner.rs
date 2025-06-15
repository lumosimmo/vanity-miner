use crate::{
    createx::{Create2Config, mine_create2_salt_with_suffix},
    univ4_hook::config::{V4HookConfig, V4HookResult},
};
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::{from_value, to_value};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Mines for a salt that produces a Uniswap v4 hook address satisfying the given permission flags.
///
/// ## Arguments
/// * `config` - A `V4HookConfig` struct defining the mining parameters.
///
/// ## Returns
/// A `V4HookResult` containing the found salts/addresses and total iterations.
pub fn mine_v4_hook_salt(config: &V4HookConfig) -> V4HookResult {
    let create2_config = Create2Config {
        deployer: config.deployer,
        init_code_hash: config.init_code_hash,
        max_iterations: config.max_iterations,
        max_results: config.max_results,
        seed: config.seed,
    };
    let suffix = config.permissions.to_suffix();

    mine_create2_salt_with_suffix(&create2_config, &suffix)
}

// WASM wrapper functions
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_mine_v4_hook_salt(config: JsValue) -> Result<JsValue, JsValue> {
    let config: V4HookConfig = from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mining_result = mine_v4_hook_salt(&config);
    to_value(&mining_result).map_err(|e| JsValue::from_str(&e.to_string()))
}
