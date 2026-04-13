// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *           _____                      _____  _                   *
// *          / ____|                    |  __ \(_)                  *
// *         | |  __  __ _ _ __ ___   ___| |  | |_  __ _             *
// *         | | |_ |/ _` | '_ ` _ \ / _ \ |  | | |/ _` |            *
// *         | |__| | (_| | | | | | |  __/ |__| | | (_| |            *
// *          \_____|\__,_|_| |_| |_|\___|_____/|_|\__, |            *
// *                                                __/ |            *
// *                                               |___/             *
// *                 Copyright (c) 2022 - 2026                       *
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
#![doc(html_logo_url = "https://github.com/user-attachments/assets/179d72f8-0c1f-4034-9852-b725254ece53")]

/// Core functionalities essential for the library.
///
/// This module contains the core logic and utilities that are foundational
/// to the library's operation. It is not meant for direct end user interaction
/// but serves as the backbone for other public modules.
#[allow(dead_code)]
pub(crate) mod core;

/// Convertors for transforming structures into common formats.
///
/// This feature gated module defines generic traits that facilitate
/// conversion of custom structures into widely used formats.
#[cfg(feature = "ext_converters")]
pub mod converters;

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

/// Dictionary for querying game servers by identifier.
///
/// This module provides a way to query game servers using a string identifier.
#[cfg(feature = "ext_dict")]
pub mod dict;

/// Common imports for easier usage of the library.
///
/// The prelude module is designed to include frequently used types and
/// functions, making it easier to use the library without importing
/// multiple items manually.
pub mod prelude;
