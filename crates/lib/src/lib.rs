// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *           _____                      _____  _                   *
// *          / ____|                    |  __ \(_)                  *
// *         | |  __  __ _ _ __ ___   ___| |  | |_  __ _             *
// *         | | |_ |/ _` | '_ ` _ \ / _ \ |  | | |/ _` |            *
// *         | |__| | (_| | | | | | |  __/ |__| | | (_| |            *
// *          \_____|\__,_|_| |_| |_|\___|_____/|_|\__, |            *
// *                                                __/ |            *
// *                                               |___/             *
// *                 Copyright (c) 2022 - 2025                       *
// *            GameDig Organization & Contributors                  *
// *                                                                 *
// *               Licensed under the MIT License                    *
// *  See the LICENSE file in the project root for more information  *
// *                                                                 *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

// Adds the README file at the beginning of the documentation.
#![doc = include_str!("../README.md")]

/// Core functionalities essential for the library.
///
/// This module contains the core logic and utilities that are foundational
/// to the library's operation. It is not meant for direct end user interaction
/// but serves as the backbone for other public modules.
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

/// Logging utilities.
///
/// This feature-gated module provides logging targets that can be used
/// with the `log` crate for logging events within the library.
#[cfg(feature = "attribute_log")]
pub mod log;

pub mod dict;
