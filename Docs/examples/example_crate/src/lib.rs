#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Project-agnostic service-oriented template.
//! Crate root.

pub mod config;
pub mod container;
pub mod error;
pub mod processors;
pub mod adapters;
pub mod providers;
pub mod services;
pub mod helpers;
pub mod models;
