//! This crate provides a clean, ready to use client for [Falco](https://github.com/falcosecurity/falco), a
//! Cloud Native Runtime Security tool written in C++.
//!
//! With this crate you can easily connect to the Falco gRPC API, interact with it, receive the alerts, and ...
//!

#[macro_use]
mod macros;
#[macro_use]
pub mod api;
mod certs;
pub mod client;
pub mod config;
mod errors;
pub use crate::errors::Error;
pub use crate::errors::ErrorKind;
pub use crate::errors::Result;
pub use crate::errors::ResultExt; // reexport of failure::ResultExt
