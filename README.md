# Vanity Miner

A blazing-fast, minimalistic vanity address miner for common smart contract deployment patterns. Written in Rust and optimized for portability, perfect for just-in-time (JIT) address mining directly on the client-side in environments like WebAssembly (WASM).

## Motivation

This project was born out of the need for a client-side JIT vanity address miner for contract deployments from a web app.

Instead of pre-mining salts and securing them behind APIs and captchas, **Vanity Miner** offers a simpler, more decentralized alternative: a miner optimized for the client-side which can run directly in the user's browser. One bonus point to this approach is that the mining process itself serves as a lightweight Proof-of-Work (POW), effectively acting as its own captcha while providing a seamless user experience.

## Supported Use-cases

The following use-cases are supported:

- **CREATE2**: Direct CREATE2 address with prefix/suffix/contains patterns
- **CREATE3**: CREATE3 address deployed via [CreateX](https://github.com/pcaversaccio/createx/)
- **Uniswap V4 Hooks**: Uniswap V4 hook address with permission flags
- **EulerSwap**: EulerSwap address with pool parameters

## Performance

On an iPhone 16 base model, the miner achieves approximately 1.06 million iterations per second (in-browser WASM). Expected mining times:

| Pattern Match          | Probability     | Expected Iterations | Estimated Time |
| ---------------------- | --------------- | ------------------- | -------------- |
| 2 characters (1 byte)  | 1 in 256        | 256                 | < 1 ms         |
| 4 characters (2 bytes) | 1 in 65,536     | 65,536              | ~62 ms         |
| 6 characters (3 bytes) | 1 in 16,777,216 | 16,777,216          | ~16 seconds    |

_Note: These are statistical averages based on my testing. Actual mining time may vary due to the probabilistic nature of the process._

## Tradeoffs

Vanity Miner prioritizes **broad portability** and **ease of integration**, especially in WebAssembly environments. For this reason, it is CPU-based and written in pure Rust.

If your only goal is to find the absolute most complex vanity address with maximum efficiency, a GPU-accelerated tool like [**createXcrunch**](https://github.com/HrikB/createXcrunch/) may be a better fit.

## Installation

### As a Rust library

```toml
[dependencies]
vanity-miner = "0.1.0"
```

### For WebAssembly

```bash
# Install wasm-pack if you haven't already
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the WASM package
wasm-pack build --target web

# The generated `pkg/` directory contains the WASM module and JS bindings
```

## API Reference

### Mining Functions

All CreateX mining functions support three pattern matching modes:

- `mine_*_salt_with_prefix(config, prefix)` - Match addresses starting with the given prefix in a `u8` array
- `mine_*_salt_with_suffix(config, suffix)` - Match addresses ending with the given suffix in a `u8` array
- `mine_*_salt_with_contains(config, contains)` - Match addresses containing the given sequence in a `u8` array

Available for:

- CREATE2: `mine_create2_salt_*`
- CREATE3: `mine_create3_salt_*`

The Uniswap V4 hook mining function is a wrapper around the `mine_create2_salt_with_suffix` function, with the suffix defined by the permission flags.

The EulerSwap mining function is a wrapper around the Uniswap V4 hook mining function.

All public functions are also available in WASM as `wasm_mine_*`.

### Configuration Structures

- `Create2Config` - Configuration for CREATE2 mining
- `Create3Config` - Configuration for CREATE3 mining (via CreateX)
- `V4HookConfig` - Configuration for Uniswap V4 hook address mining
- `EulerSwapConfig` - Configuration for EulerSwap address mining

All configurations support:

- `max_iterations` - Maximum mining iterations before stopping
- `max_results` - Maximum number of matching results to find
- `seed` - Optional seed for deterministic mining

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

The project currently uses the `keccak256` implementation from [Alloy](https://crates.io/crates/alloy-primitives). A significant performance improvement could be achieved by implementing a custom version that supports partial hash computation for more efficient prefix matching.
