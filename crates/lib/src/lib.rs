#![doc = include_str!("../README.md")]

pub(crate) mod core;

pub mod error;
pub mod prelude;
pub mod settings;

pub mod game;
pub mod protocol;
pub mod service;

#[cfg(feature = "attribute_adapters")]
pub mod adapters;
