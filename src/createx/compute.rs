use alloy_primitives::{Address, B256, ChainId, U256, keccak256};

/// The proxy initcode used in CreateX's CREATE3 pattern.
///
/// See [`CreateX#deployCreate3`].
///
/// [`CreateX#deployCreate3`]: https://github.com/pcaversaccio/createx/blob/23d8a42b8e922f134ef2fa9e9470dfc6cced5d2e/src/CreateX.sol#L632
const PROXY_INIT_CODE: &[u8] = &[
    0x67, 0x36, 0x3d, 0x3d, 0x37, 0x36, 0x3d, 0x34, 0xf0, 0x3d, 0x52, 0x60, 0x08, 0x60, 0x18, 0xf3,
];

/// Implements different safeguarding mechanisms depending on the encoded values in the salt.
///
/// See [`CreateX#_guard`].
///
/// [`CreateX#_guard`]: https://github.com/pcaversaccio/createx/blob/23d8a42b8e922f134ef2fa9e9470dfc6cced5d2e/src/CreateX.sol#L886
pub fn guarded_salt(salt: B256, caller: Option<Address>, chain_id: Option<ChainId>) -> B256 {
    match (caller, chain_id) {
        // keccak256(abi.encode(msg.sender, block.chainid, salt))
        (Some(c), Some(id)) => {
            let mut preimage = [0u8; 96]; // 32 (address) + 32 (chain_id) + 32 (salt)
            preimage[12..32].copy_from_slice(c.as_slice());
            preimage[32..64].copy_from_slice(&U256::from(id).to_be_bytes::<32>());
            preimage[64..96].copy_from_slice(salt.as_slice());
            keccak256(preimage)
        }
        // _efficientHash({a: bytes32(uint(uint160(msg.sender))), b: salt})
        (Some(c), None) => {
            let mut preimage = [0u8; 64]; // 32 (address) + 32 (salt)
            preimage[12..32].copy_from_slice(c.as_slice());
            preimage[32..64].copy_from_slice(salt.as_slice());
            keccak256(preimage)
        }
        // _efficientHash({a: bytes32(block.chainid), b: salt})
        (None, Some(id)) => {
            let mut preimage = [0u8; 64]; // 32 (chain_id) + 32 (salt)
            preimage[0..32].copy_from_slice(&U256::from(id).to_be_bytes::<32>());
            preimage[32..64].copy_from_slice(salt.as_slice());
            keccak256(preimage)
        }
        // keccak256(abi.encode(salt))
        (None, None) => keccak256(salt),
    }
}

/// Computes a CREATE3 address from a guarded salt and deployer address.
pub fn create3_address(deployer: Address, guarded_salt: B256) -> Address {
    let proxy_init_code_hash = keccak256(PROXY_INIT_CODE);

    // 1 (0xff) + 20 (deployer) + 32 (salt) + 32 (hash)
    let mut proxy_preimage = [0u8; 85];
    proxy_preimage[0] = 0xff;
    proxy_preimage[1..21].copy_from_slice(deployer.as_slice());
    proxy_preimage[21..53].copy_from_slice(guarded_salt.as_slice());
    proxy_preimage[53..85].copy_from_slice(proxy_init_code_hash.as_slice());
    let proxy_address_hash = keccak256(&proxy_preimage);
    let proxy_address = Address::from_slice(&proxy_address_hash[12..]);

    // 1 (0xd6) + 1 (0x94) + 20 (proxy_addr) + 1 (0x01)
    let mut final_preimage = [0u8; 23];
    final_preimage[0] = 0xd6;
    final_preimage[1] = 0x94;
    final_preimage[2..22].copy_from_slice(proxy_address.as_slice());
    final_preimage[22] = 0x01;
    let final_address_hash = keccak256(&final_preimage);
    Address::from_slice(&final_address_hash[12..])
}

/// Computes a CREATE2 address from a deployer address, salt, and init code hash.
pub fn create2_address(deployer: Address, salt: B256, init_code_hash: B256) -> Address {
    // 1 (0xff) + 20 (deployer) + 32 (salt) + 32 (init_code_hash) = 85 bytes
    let mut preimage = [0u8; 85];
    preimage[0] = 0xff;
    preimage[1..21].copy_from_slice(deployer.as_slice());
    preimage[21..53].copy_from_slice(salt.as_slice());
    preimage[53..85].copy_from_slice(init_code_hash.as_slice());

    let hash = keccak256(&preimage);

    Address::from_slice(&hash[12..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};

    #[test]
    fn test_create3_address_no_protection() {
        let deployer = address!("ba5ed099633d3b313e4d5f7bdc1305d3c28ba5ed");
        let salt = b256!("0000000000000000000000000000000000000000000000000000000000000001");

        let guarded = guarded_salt(salt, None, None);
        assert_eq!(
            guarded,
            keccak256(salt),
            "Guarded salt without protection should equal keccak256 of raw salt"
        );

        let computed_address = create3_address(deployer, guarded);
        let expected_address = address!("cbc02963eef555f755e8af71fd66d3366631fe74");
        assert_eq!(
            computed_address, expected_address,
            "CREATE3 address computation mismatch for deployer {} with salt {}",
            deployer, salt
        );
    }
}
