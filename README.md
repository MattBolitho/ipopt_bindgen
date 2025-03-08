# Ipopt Bindgen

[![crates.io](https://img.shields.io/crates/v/ipopt_bindgen.svg)](https://crates.io/crates/ipopt_bindgen "ipopt_bindgen crates.io link")
[![docs.rs](https://docs.rs/bindgen/badge.svg)](https://docs.rs/ipopt_bindgen/ "ipopt_bindgen docs.rs link")
[![dependency status](https://deps.rs/repo/github/MattBolitho/ipopt_bindgen/status.svg)](https://deps.rs/repo/github/MattBolitho/ipopt_bindgen "ipopt_bindgen deps.rs link")
[![License](https://img.shields.io/crates/l/ipopt_bindgen.svg)](https://spdx.org/licenses/Apache-2.0.html "Apache 2.0 license link")

This crate provides raw Rust bindings to the C interface of the [Ipopt](https://github.com/coin-or/Ipopt "Ipopt GitHub repository") optimization library using [bindgen](https://github.com/rust-lang/rust-bindgen "bindgen GitHub repository").

## Quick Start

Install Ipopt (more details later in the prerequisites section).

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ipopt_bindgen = "0.3"
```

Use the Ipopt C interface in your project:

```rust
use ipopt_bindgen::*;

// Ipopt C interface symbols are now available.
println!("{0}.{1}.{2}", IPOPT_VERSION_MAJOR, IPOPT_VERSION_MINOR, IPOPT_VERSION_RELEASE);
```

Or, run the examples in this repository:

```sh
cargo run --example hs071-c-interface
```

## Why another Ipopt binding?

Rust already has other crates that provide bindings to Ipopt:

- [ipopt-rs](https://crates.io/crates/ipopt "ipopt-rs crates.io link") (with [ipopt-sys](https://crates.io/crates/ipopt-sys "ipopt-sys crates.io link"))
- [ipopt-src](https://github.com/Maroon502/ipopt-src "ipopt-src crates.io link")

So why create another one?

The purpose of this crate is to use the system Ipopt installation directly, rather than build it from within `cargo`.
This has both advantages and disadvantages, and might not be right for you.

Pros:

- ✅ Completely customize your Ipopt build without the constraints of `cargo` or existing crates.
- ✅ Potentially fewer build dependencies for your project.
- ✅ Not tied to a specific version of Ipopt (unless the C API has breaking changes).

Cons:

- ❌ Requires a system installation of Ipopt, which may be inconvenient for your project.
- ❌ Uses the C interface, rather than the more feature-ful C++ one.

There is a [full, mirrored example of the HS071 problem](examples/hs071.rs "HS071 problem link") from the Ipopt documentation in the examples directory.

## Prerequisites

### Ipopt

Ipopt must be installed on your system.
The binding header file will attempt to include the Ipopt C interface header.
By default, this will attempt to `#include <coin-or/IpStdCInterface.h>` (the typical include structure for source builds of Ipopt).
This behaviour can be controlled by the `IPOPT_BINDGEN_INCLUDE_PREFIX` environment variable, which will change this include statement to match the pattern `#include <${IPOPT_BINDGEN_INCLUDE_PREFIX}IpStdCInterface.h>`.
If the value of this environment variable does not end with a `/`, then it will be automatically added for you.

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

## Changes

All meaningful changes to this project are documented in the [CHANGELOG](CHANGELOG.md "Changelog link").

The project is versioned using [0ver](https://0ver.org/ "0ver specification link").

## License

This repository is licensed under the Apache-2.0 with LLVM exception.
Please refer to the [`LICENSE` file](./LICENSE "License file link") for more information.

Ipopt itself is licensed under the [Eclipse Public License (EPL) version 2.0](https://spdx.org/licenses/EPL-2.0.html "Eclipse Public License Version 2.0 link").
