pub mod config;
pub mod miner;

pub use config::{V4HookConfig, V4HookMatch, V4HookPermissions, V4HookResult};
pub use miner::mine_v4_hook_salt;
