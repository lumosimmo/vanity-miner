pub mod compute;
pub mod config;
pub mod miner;

pub use compute::{create2_address, create3_address, guarded_salt};
pub use config::{
    Create2Config, Create2Match, Create2Result, Create3Config, Create3Match, Create3Result,
};
pub use miner::{
    mine_create2_salt, mine_create2_salt_with_contains, mine_create2_salt_with_prefix,
    mine_create2_salt_with_suffix, mine_create3_salt, mine_create3_salt_with_contains,
    mine_create3_salt_with_prefix, mine_create3_salt_with_suffix,
};
