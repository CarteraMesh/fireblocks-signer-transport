# fireblocks-signer-transport

[![Crates.io](https://img.shields.io/crates/v/fireblocks-signer-transport.svg)](https://crates.io/crates/fireblocks-signer-transport)
[![Docs.rs](https://docs.rs/fireblocks-signer-transport/badge.svg)](https://docs.rs/fireblocks-signer-transport)
[![CI](https://github.com/CarteraMesh/fireblocks-signer-transport/workflows/CI/badge.svg)](https://github.com/CarteraMesh/fireblocks-signer-transport/actions)
[![Cov](https://codecov.io/github/CarteraMesh/fireblocks-signer-transport/graph/badge.svg?token=dILa1k9tlW)](https://codecov.io/github/CarteraMesh/fireblocks-signer-transport)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install fireblocks-signer-transport`

## Development

### Prerequisites

- **Rust Nightly**: Required for code formatting with advanced features
  ```bash
  rustup install nightly
  ```

### Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/CarteraMesh/fireblocks-signer-transport.git
   cd fireblocks-signer-transport
   ```

2. **Set up environment**
   ```bash
   # Copy and configure environment variables
   cp env-sample .env

   # Install Rust nightly for formatting
   rustup install nightly
   ```

3. **Build and test**
   ```bash
   # Build the project
   cargo build

   # Run tests (requires valid Fireblocks credentials in .env)
   cargo test

   # Format code (requires nightly)
   cargo +nightly fmt --all
   ```

### Code Formatting

This project uses advanced Rust formatting features that require nightly:

```bash
# Format all code
cargo +nightly fmt --all

# Check formatting
cargo +nightly fmt --all -- --check
```

## License

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
