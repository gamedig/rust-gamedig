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

// Forbid unsafe code within the library.
#![forbid(unsafe_code)]
// Adds the README file at the beginning of the documentation.
#![doc = include_str!("../README.md")]
// Adds the logo to the documentation.
#![doc(
    html_logo_url = "https://github.com/user-attachments/assets/179d72f8-0c1f-4034-9852-b725254ece53"
)]

// We use macros from log module so if the feature
// is not enabled, we still need to expose to the crate so
// that the macros are no ops.
//
// Another note with log macros is that we need to keep the log
// module above consuming modules, otherwise modules that are above log
// will not be able to use the macros as they must be in scope before used.
// https://rustc-dev-guide.rust-lang.org/macro-expansion.html#name-resolution
#[macro_use]
#[cfg(not(feature = "attribute_log"))]
pub(crate) mod log;

/// Logging utilities.
///
/// This feature gated module provides logging targets that can be used
/// with the `log` crate for logging events within the library.
#[macro_use]
#[cfg(feature = "attribute_log")]
pub mod log;

/// Core functionalities essential for the library.
///
/// This module contains the core logic and utilities that are foundational
/// to the library's operation. It is not meant for direct end user interaction
/// but serves as the backbone for other public modules.
#[allow(dead_code)]
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
/// game specific functionalities.
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

/// Convertors for transforming structures into common formats.
///
/// This feature gated module defines generic traits that facilitate
/// conversion of custom structures into widely used formats.
#[cfg(feature = "attribute_converters")]
pub mod converters;

/// Dictionary.
///
/// This feature gated module provides a dictionary that can be used to get
/// a generic client from a identifier.
#[cfg(feature = "attribute_dict")]
pub mod dict;
