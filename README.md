# Ipopt Bindgen

[![crates.io](https://img.shields.io/crates/v/ipopt_bindgen.svg)](https://crates.io/crates/ipopt_bindgen)
[![docs.rs](https://docs.rs/bindgen/badge.svg)](https://docs.rs/ipopt_bindgen/)
[![License](https://img.shields.io/crates/l/ipopt_bindgen.svg)](https://opensource.org/licenses/MIT)

This crate provides raw Rust bindings to the C interface of the [Ipopt](https://github.com/coin-or/Ipopt "Ipopt GitHub repository") optimization library using [bindgen](https://github.com/rust-lang/rust-bindgen "bindgen GitHub repository").

## Quick Start

Install Ipopt (more details later in the prerequisites section).

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ipopt_bindgen = "0.1"
```

Use the Ipopt C interface in your project:

```rust
use ipopt_bindgen::*;

// Ipopt C interface symbols are now available.
println!("{0}.{1}.{2}", IPOPT_VERSION_MAJOR, IPOPT_VERSION_MINOR, IPOPT_VERSION_RELEASE);
```

Or, run the example in this repository:

```sh
cargo run --example hs071
```

## Support

This crate has currently been tested on:

- Debian based Linux, with Ipopt installed from source.
- Windows 11, with Ipopt installed from precompiled binaries, MSVC.

## Why another Ipopt binding?

Rust already has other crates that provide bindings to Ipopt:

- [ipopt-rs](https://crates.io/crates/ipopt) (with [ipopt-sys](https://crates.io/crates/ipopt-sys))
- [ipopt-src](https://github.com/Maroon502/ipopt-src)

So why create another one?

The purpose of this crate is to use the system Ipopt installation directly, rather than build it from within cargo.
This has both advantages and disadvantages, and might not be right for you.

Pros:

- ✅ Completely customize your Ipopt build without the constraints of `cargo` or existing crates.
- ✅ Potentially fewer build dependencies for your project.
- ✅ Not tied to a specific version of Ipopt (unless the C API has breaking changes).

Cons:

- ❌ Requires a system installation of Ipopt, which may be inconvenient for your project.
- ❌ Uses the C interface, rather than the more feature-ful C++ one.

If you just want to get started with Ipopt in Rust, you should probably use one of the aforementioned existing crates.
However, if you already have Ipopt installed on your system or need to use a build with specific features, this crate might be for you.

Ideally, an idiomatic Rust crate would be build on top of this one, or it could be integrated into an existing crate.

There is a [full, mirrored example of the HS071 problem](examples/hs071.rs "HS071 problem link") from the Ipopt documentation in the examples directory.
Again, the ideal usage of this crate is to be consumed by a higher-level crate that provides a more idiomatic interface, but it is possible to use it directly.
The example demonstrates the C API usage, which will probably aid in that endeavour!

## Prerequisites

### Ipopt

Ipopt must be installed on your system.
The binding header file will attempt to include `coin-or/IpStdCInterface.h`.

On Linux, you can either install Ipopt using your package manager or [build it from source](https://coin-or.github.io/Ipopt/INSTALL.html "Ipopt build documentation").
By default, these processes should make Ipopt immediately available for use with this crate.

On Windows, you can get [precompiled binaries](https://www.coin-or.org/download/binary/Ipopt/ "Ipopt GitHub Releases") from the Ipopt GitHub repository.
Then, add the downloaded release files to your environment variables as follows:

- Add the directory containing precompiled binaries to your `PATH`.
- Add the directory containing static libraries to `LIB`.
- Add the directory containing the Ipopt headers to `INCLUDE`.

On Windows, by default this crate will attempt to use the `cc` crate to automatically detect an MSVC installation.
This is used to more easily supply standard library headers for the Ipopt C interface header.
If this fails, you will need to set the `INCLUDE` environment variable to include your desired standard library headers.

### Bindgen

The [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html "bindgen requirements documentation") must also be available on your system.
This basically amounts to having `clang` installed.

On Windows, you can use `winget install LLVM.LLVM`.
On Linux, you can use your package manager to install `clang`.
LLVM also provide an [`apt` install script](https://apt.llvm.org/ "LLVM apt script site link").

## Changes

All meaningful changes to this project are documented in the [CHANGELOG](CHANGELOG.md "Changelog link").

## License

This repository is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](./LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

Ipopt itself is licensed under the Eclipse Public License (EPL) version 2.0.
