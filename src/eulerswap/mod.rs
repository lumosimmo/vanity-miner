pub mod compute;
pub mod config;
pub mod miner;

pub use config::{EulerSwapConfig, EulerSwapResult};
pub use miner::mine_eulerswap_salt;
