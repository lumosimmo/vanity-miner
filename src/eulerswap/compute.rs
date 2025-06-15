use crate::createx::create2_address;
use crate::eulerswap::config::EulerSwapParams;
use alloy_primitives::{Address, B256, keccak256};

const BYTECODE_HEAD: &[u8] = &[
    0x60, 0x0b, 0x38, 0x03, 0x80, 0x60, 0x0b, 0x3d, 0x39, 0x3d, 0xf3, 0x36, 0x3d, 0x3d, 0x37, 0x3d,
    0x3d, 0x3d, 0x3d, 0x60, 0x36, 0x80, 0x38, 0x03, 0x80, 0x91, 0x36, 0x39, 0x36, 0x01, 0x3d, 0x73,
];
const BYTECODE_TAIL: &[u8] = &[
    0x5a, 0xf4, 0x3d, 0x3d, 0x93, 0x80, 0x3e, 0x60, 0x34, 0x57, 0xfd, 0x5b, 0xf3,
];

/// Computes the address of the EulerSwap pool.
pub fn eulerswap_address(
    factory: Address,
    eulerswap_impl: Address,
    pool_params: EulerSwapParams,
    salt: B256,
) -> Address {
    let creation_code = creation_code_meta_proxy(eulerswap_impl, &pool_params.abi_encode());
    let init_code_hash = keccak256(creation_code);

    create2_address(factory, salt, init_code_hash)
}

/// Computes the creation code for the EulerSwap pool.
pub fn creation_code_meta_proxy(target_contract: Address, metadata: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(BYTECODE_HEAD);
    bytes.extend_from_slice(target_contract.as_slice());
    bytes.extend_from_slice(BYTECODE_TAIL);
    bytes.extend_from_slice(metadata);

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{U256, address, aliases::U112, b256};

    #[test]
    fn test_eulerswap_address() {
        let factory = address!("0xFb9FE66472917F0F8966506A3bf831Ac0c10caD4");
        let eulerswap_impl = address!("0xF5d35536482f62c9031b4d6bD34724671BCE33d1");
        let pool_params = EulerSwapParams {
            vault0: address!("0x797DD80692c3b2dAdabCe8e30C07fDE5307D48a9"),
            vault1: address!("0x313603FA690301b0CaeEf8069c065862f9162162"),
            euler_account: address!("0x0AFbF798467f9b3b97F90D05Bf7Df592d89A6CF0"),
            equilibrium_reserve0: U112::from(68925668118_u128),
            equilibrium_reserve1: U112::from(73751769958_u128),
            price_x: U256::from(1000000),
            price_y: U256::from(1000000),
            concentration_x: U256::from(999000000000000100_u128),
            concentration_y: U256::from(999000000000000100_u128),
            fee: U256::from(10000000000000_u128),
            protocol_fee: U256::from(0),
            protocol_fee_recipient: address!("0000000000000000000000000000000000000000"),
        };
        let salt = b256!("0xad9a4b2ded54a895eb0ed679849a84bbe50609a3b39654ad9616cd0c24b4107b");

        let computed_address =
            eulerswap_address(factory, eulerswap_impl, pool_params.clone(), salt);
        let expected_address = address!("0x8863dC83c8EeE1B67e61523ffcb928f40580E8A8");
        assert_eq!(
            computed_address, expected_address,
            "EulerSwap address computation mismatch for factory {} with eulerswap_impl {} with pool_params {:?} with salt {}",
            factory, eulerswap_impl, pool_params, salt
        );
    }
}
