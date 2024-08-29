#![doc = include_str!("../README.md")]

/// Core functionalities essential for the library.
///
/// This module contains the core logic and utilities that are foundational
/// to the library's operation. It is not meant for direct interaction but
/// serves as the backbone for other modules.
pub(crate) mod core;

/// Error handling utilities.
///
/// This module provides a comprehensive set of enums for managing and handling
/// errors that may occur during the library's operation.
pub mod error;

/// Common imports for easier usage of the library.
///
/// The prelude module is designed to include frequently used types and
/// functions, making it easier to use the library without importing
/// multiple items manually.
pub mod prelude;

/// Common settings for the library.
///
/// This module manages the configuration and settings that govern
/// the behavior of the library.
pub mod settings;

/// Implementations for specific games.
///
/// The game module contains the logic and structures related to
/// game-specific functionalities.
pub mod game;

/// Protocol implementations.
///
/// This module provides the implementations for various communication
/// protocols used within the library.
pub mod protocol;

/// Service implementations.
///
/// The service module handles the logic and structures related to
/// external services and APIs.
pub mod service;

/// Adapters for converting structures to common formats.
///
/// This feature-gated module includes adapters that facilitate the
/// conversion of custom structures into more widely-used formats.
#[cfg(feature = "attribute_adapters")]
pub mod adapters;
