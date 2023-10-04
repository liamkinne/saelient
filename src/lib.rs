//! # Salient: J1939 in Rust
//!
//! This crate provides an interface built on top of
//! [`embedded_can`](https://docs.rs/embedded-can/latest/embedded_can/) traits.
//!
//! Currently this library only supports some basics like identifiers and basic
//! signal types. In time it will supports parsing and extracting signals from
//! message bodies based on identifier metadata.

pub mod identifier;
pub mod signal;
