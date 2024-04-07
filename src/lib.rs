//! # Ipopt Bindgen
//!
//! This crate provides access to the C API of the
//! [Ipopt](https://github.com/coin-or/Ipopt) optimizer.
//! As the name implies, it uses [bindgen](https://github.com/rust-lang/rust-bindgen)
//! to generate the bindings.
//!
//! This crate does **not** build Ipopt itself.
//! The `build.rs` script will generate the bindings based on the system's installed
//! version of Ipopt.
//!
//! This crate only provides access to the raw C bindings of Ipopt.
//! It is unlikely that this will be nice to use directly.
//! For more information, please refer to the
//! [Ipopt C API documentation](https://coin-or.github.io/Ipopt/INTERFACES.html#INTERFACE_C).
//! For a full usage example, please refer to the `examples` directory of the project's
//! GitHub repository.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
