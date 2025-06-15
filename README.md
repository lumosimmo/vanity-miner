# Vanity Miner

A blazing-fast, minimalistic vanity address miner for common smart contract deployment patterns. Written in Rust and optimized for portability, perfect for just-in-time (JIT) address mining directly on the client-side in environments like WebAssembly (WASM).

## Motivation

This project was born out of the need for a client-side JIT vanity address miner for contract deployments from a web app.

Instead of pre-mining salts and securing them behind APIs and captchas, **Vanity Miner** offers a simpler, more decentralized alternative: a miner optimized for the client-side which can run directly in the user's browser. One bonus point to this approach is that the mining process itself serves as a lightweight Proof-of-Work (POW), effectively acting as its own captcha while providing a seamless user experience.

## Supported Use-cases

The following use-cases are supported:

- Create3 via [CreateX](https://github.com/pcaversaccio/createx/)

## Performance

Finding a 4-character hex prefix typically takes less than a second, even in a browser.

## Tradeoffs

Vanity Miner prioritizes **broad portability** and **ease of integration**, especially in WebAssembly environments. For this reason, it is CPU-based and written in pure Rust.

If your only goal is to find the absolute most complex vanity address with maximum efficiency, a GPU-accelerated tool like [**createXcrunch**](https://github.com/HrikB/createXcrunch/) may be a better fit.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

The project currently uses the `keccak256` implementation from [Alloy](https://crates.io/crates/alloy-primitives). A significant performance improvement could be achieved by implementing a custom version that supports partial hash computation for more efficient prefix matching.
