# rebound-rs

Rust wrapper for the [REBOUND](https://github.com/hannorein/rebound) N-body simulation library.

`rebound-rs` provides a higher-level Rust API on top of the low-level `rebound-bind` FFI crate.

> [!WARNING]
> `rebound-rs` is currently a work in progress. Not all REBOUND APIs are wrapped yet, and the wrapper API may change before stabilization.

## Requirements

`rebound-rs` depends on `rebound-bind`, which builds and links the REBOUND C library through Rust FFI. Make sure your system has a working C toolchain and `libclang` available for bindgen.

On macOS, installing the Xcode Command Line Tools is usually enough:

```sh
xcode-select --install
```

On Linux, install Clang and libclang through your distribution package manager. For example, on Debian/Ubuntu:

```sh
sudo apt-get install clang libclang-dev
```

On Windows, install LLVM and make sure `libclang.dll` is available. If you use winget:

```powershell
winget install LLVM.LLVM
```

You may also need to set `LIBCLANG_PATH` to LLVM's `bin` directory, for example `C:\Program Files\LLVM\bin`.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
rebound-rs = "4.6.0-alpha.1"
```

Then import the crate in your Rust code:

```rust
use rebound_rs::*;
```

See the `examples/` directory for current usage examples.

You can run an example with:

```sh
cargo run --example simplest
```

or, if you use `just`:

```sh
just example simplest
```

## Development

Common development commands are available through the `Justfile`:

```sh
just check
just test
just clippy
just ci
```

You can list all available recipes with:

```sh
just --list
```

## License

This project is licensed under GPL-3.0.
