use alloy_primitives::{Address, B256, ChainId};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::wasm_serde;

/// Configuration for the Create3 mining process via CreateX.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Create3Config {
    /// The address of the CreateX factory contract.
    pub deployer: Address,
    /// Permissioned deploy protection for that address.
    pub caller: Option<Address>,
    /// Cross-chain deployment protection.
    pub chain_id: Option<ChainId>,
    /// The maximum number of attempts before giving up.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_u64")
    )]
    pub max_iterations: u64,
    /// The maximum number of results to find.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_u64")
    )]
    pub max_results: u64,
    /// Seed for the random number generator.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_option_u128")
    )]
    pub seed: Option<u128>,
}

/// A single successful match from a Create3 mining operation.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Create3Match {
    /// The raw salt that satisfies the config and predicate.
    pub salt: B256,
    /// The guarded salt after the `CreateX#_guard` logic is applied.
    pub guarded_salt: B256,
    /// The final, computed contract address.
    pub computed_address: Address,
}

/// Result structure that includes matches and total iterations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Create3Result {
    /// The found matches.
    pub results: Vec<Create3Match>,
    /// Total number of iterations performed.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(serialize_with = "crate::wasm_serde::serialize_usize")
    )]
    pub total_iterations: usize,
}

/// Configuration for the Create2 mining process
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Create2Config {
    /// The address of the CreateX factory contract.
    pub deployer: Address,
    /// The init code hash to use for the CREATE2 address.
    pub init_code_hash: B256,
    /// The maximum number of attempts before giving up.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_u64")
    )]
    pub max_iterations: u64,
    /// The maximum number of results to find.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_u64")
    )]
    pub max_results: u64,
    /// Seed for the random number generator.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(deserialize_with = "wasm_serde::deserialize_option_u128")
    )]
    pub seed: Option<u128>,
}

/// A single successful match from a Create2 mining operation.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Create2Match {
    /// The raw salt that satisfies the config and predicate.
    pub salt: B256,
    /// The final, computed contract address.
    pub computed_address: Address,
}

/// Result structure that includes matches and total iterations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Create2Result {
    /// The found matches.
    pub results: Vec<Create2Match>,
    /// Total number of iterations performed.
    #[cfg_attr(
        target_arch = "wasm32",
        serde(serialize_with = "crate::wasm_serde::serialize_usize")
    )]
    pub total_iterations: usize,
}
