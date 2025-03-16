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

#[cfg(feature = "rust-interface")]
pub mod application;
pub mod c_interface;
#[cfg(feature = "rust-interface")]
pub mod results;
#[cfg(feature = "rust-interface")]
pub mod tnlp;

// Export everything into the root of the crate (seeing as there aren't many public symbols in this
// library). We *don't* do this for the C interface because the symbols are being automatically
// generated and ideally consumers will not use them directly anyway, save for creating their own
// higher-level interface.

#[cfg(feature = "rust-interface")]
pub use application::*;
#[cfg(feature = "rust-interface")]
pub use results::*;
#[cfg(feature = "rust-interface")]
pub use tnlp::*;
