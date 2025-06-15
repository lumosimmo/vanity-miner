use crate::univ4_hook::{V4HookMatch, V4HookResult};
use alloy_primitives::{Address, U256, aliases::U112};
use alloy_sol_types::{SolValue, sol};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::wasm_serde;

/// Parameters for the EulerSwap pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EulerSwapParams {
    pub vault0: Address,
    pub vault1: Address,
    pub euler_account: Address,
    pub equilibrium_reserve0: U112,
    pub equilibrium_reserve1: U112,
    pub price_x: U256,
    pub price_y: U256,
    pub concentration_x: U256,
    pub concentration_y: U256,
    pub fee: U256,
    pub protocol_fee: U256,
    pub protocol_fee_recipient: Address,
}

impl EulerSwapParams {
    pub fn abi_encode(&self) -> Vec<u8> {
        sol! {
            struct Params {
                address vault0;
                address vault1;
                address eulerAccount;
                uint112 equilibriumReserve0;
                uint112 equilibriumReserve1;
                uint256 priceX;
                uint256 priceY;
                uint256 concentrationX;
                uint256 concentrationY;
                uint256 fee;
                uint256 protocolFee;
                address protocolFeeRecipient;
            }
        }
        let params = Params {
            vault0: self.vault0,
            vault1: self.vault1,
            eulerAccount: self.euler_account,
            equilibriumReserve0: self.equilibrium_reserve0,
            equilibriumReserve1: self.equilibrium_reserve1,
            priceX: self.price_x,
            priceY: self.price_y,
            concentrationX: self.concentration_x,
            concentrationY: self.concentration_y,
            fee: self.fee,
            protocolFee: self.protocol_fee,
            protocolFeeRecipient: self.protocol_fee_recipient,
        };
        Params::abi_encode(&params)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EulerSwapConfig {
    /// The EulerSwapFactory address.
    pub factory: Address,
    /// The EulerSwap implementation address.
    pub eulerswap_impl: Address,
    /// The IEulerSwap.Params struct pool parameters.
    pub pool_params: EulerSwapParams,
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

/// A single successful match from a EulerSwap mining operation.
pub type EulerSwapMatch = V4HookMatch;

/// Result structure that includes EulerSwap matches and total iterations.
pub type EulerSwapResult = V4HookResult;
