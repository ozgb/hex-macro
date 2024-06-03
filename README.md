# hex-macro

[![Crates.io](https://img.shields.io/crates/v/hex-macro.svg)](https://crates.io/crates/hex-macro)
[![Documentation](https://docs.rs/hex-macro/badge.svg)](https://docs.rs/hex-macro)
[![License](https://img.shields.io/crates/l/hex-macro.svg)](https://crates.io/crates/hex-macro)

`hex-macro` is a Rust crate that provides a `hex!` macro for compile-time hex decoding.

```rust
use hex_macro::hex;

pub const DATA: [u8; 11] = hex!("0x48656c6c6f20776f726c64"); // Also works without the '0x' prefix
assert_eq!(DATA, *b"Hello world");
```

## Features

- Decodes hex strings into byte arrays at compile time.
- Supports hex strings with and without the `0x` prefix.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hex-macro = "0.1.0"
```

## TODO

- [ ] Add support for other decoding backends using feature flags

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
