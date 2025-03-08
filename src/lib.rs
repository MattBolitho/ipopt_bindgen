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

pub mod c_interface;
#[cfg(feature = "rust-interface")]
pub mod tnlp;
#[cfg(feature = "rust-interface")]
pub mod application;

pub use c_interface::*;
#[cfg(feature = "rust-interface")]
pub use tnlp::*;
#[cfg(feature = "rust-interface")]
pub use application::*;
