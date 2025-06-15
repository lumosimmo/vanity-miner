use crate::createx::config::{Create2Match, Create2Result};
use alloy_primitives::{Address, B256};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::wasm_serde;

/// The bitmask for all 14 hook flags (0x3FFF).
pub const ALL_HOOK_MASK: u16 = (1 << 14) - 1;

// Individual hook permission flags
pub const BEFORE_INITIALIZE_FLAG: u16 = 1 << 13;
pub const AFTER_INITIALIZE_FLAG: u16 = 1 << 12;
pub const BEFORE_ADD_LIQUIDITY_FLAG: u16 = 1 << 11;
pub const AFTER_ADD_LIQUIDITY_FLAG: u16 = 1 << 10;
pub const BEFORE_REMOVE_LIQUIDITY_FLAG: u16 = 1 << 9;
pub const AFTER_REMOVE_LIQUIDITY_FLAG: u16 = 1 << 8;
pub const BEFORE_SWAP_FLAG: u16 = 1 << 7;
pub const AFTER_SWAP_FLAG: u16 = 1 << 6;
pub const BEFORE_DONATE_FLAG: u16 = 1 << 5;
pub const AFTER_DONATE_FLAG: u16 = 1 << 4;
pub const BEFORE_SWAP_RETURNS_DELTA_FLAG: u16 = 1 << 3;
pub const AFTER_SWAP_RETURNS_DELTA_FLAG: u16 = 1 << 2;
pub const AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG: u16 = 1 << 1;
pub const AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG: u16 = 1 << 0;

/// The boolean flags for which hooks to enable. Will be converted into a `u16` bitmask.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct V4HookPermissions {
    pub before_initialize: bool,
    pub after_initialize: bool,
    pub before_add_liquidity: bool,
    pub after_add_liquidity: bool,
    pub before_remove_liquidity: bool,
    pub after_remove_liquidity: bool,
    pub before_swap: bool,
    pub after_swap: bool,
    pub before_donate: bool,
    pub after_donate: bool,
    pub before_swap_return_delta: bool,
    pub after_swap_return_delta: bool,
    pub after_add_liquidity_return_delta: bool,
    pub after_remove_liquidity_return_delta: bool,
}

impl V4HookPermissions {
    /// Converts the boolean permissions into a `u16` bitmask.
    pub fn to_flags(&self) -> u16 {
        let mut flags = 0u16;
        if self.before_initialize {
            flags |= BEFORE_INITIALIZE_FLAG;
        }
        if self.after_initialize {
            flags |= AFTER_INITIALIZE_FLAG;
        }
        if self.before_add_liquidity {
            flags |= BEFORE_ADD_LIQUIDITY_FLAG;
        }
        if self.after_add_liquidity {
            flags |= AFTER_ADD_LIQUIDITY_FLAG;
        }
        if self.before_remove_liquidity {
            flags |= BEFORE_REMOVE_LIQUIDITY_FLAG;
        }
        if self.after_remove_liquidity {
            flags |= AFTER_REMOVE_LIQUIDITY_FLAG;
        }
        if self.before_swap {
            flags |= BEFORE_SWAP_FLAG;
        }
        if self.after_swap {
            flags |= AFTER_SWAP_FLAG;
        }
        if self.before_donate {
            flags |= BEFORE_DONATE_FLAG;
        }
        if self.after_donate {
            flags |= AFTER_DONATE_FLAG;
        }
        if self.before_swap_return_delta {
            flags |= BEFORE_SWAP_RETURNS_DELTA_FLAG;
        }
        if self.after_swap_return_delta {
            flags |= AFTER_SWAP_RETURNS_DELTA_FLAG;
        }
        if self.after_add_liquidity_return_delta {
            flags |= AFTER_ADD_LIQUIDITY_RETURNS_DELTA_FLAG;
        }
        if self.after_remove_liquidity_return_delta {
            flags |= AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA_FLAG;
        }
        flags
    }

    /// Converts the boolean permissions into a 2-byte suffix.
    pub fn to_suffix(&self) -> [u8; 2] {
        let flags = self.to_flags();
        let mut suffix = [0u8; 2];
        suffix[0] = (flags >> 8) as u8;
        suffix[1] = flags as u8;
        suffix
    }
}

/// Configuration for the Uniswap v4 Hook mining process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V4HookConfig {
    /// The address that will deploy the hook contract.
    pub deployer: Address,
    /// The desired hook permissions for the resulting address.
    pub permissions: V4HookPermissions,
    /// The init code hash to use for the CREATE2 address.
    pub init_code_hash: B256,
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

/// A single successful match from a v4 Hook mining operation.
pub type V4HookMatch = Create2Match;

/// Result structure that includes matches and total iterations.
pub type V4HookResult = Create2Result;
