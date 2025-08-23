//! # v0.0.4 (23/10/2022)
//!
//! ## Summary
//! Added DNS resolve, anonymous app queries, clearer errors, and support for Alien Swarm and Insurgency titles, with updated Valve protocol usage.
//!
//! ## Fixes
//! - Better bad game error.
//!
//! ## Features
//! - Queries now support DNS resolve.
//! - Valve Protocol now supports querying anonymous apps (see breaking changes).
//! - [Alien Swarm](https://store.steampowered.com/app/630/Alien_Swarm/) support.
//! - [Alien Swarm: Reactive Drop](https://store.steampowered.com/app/563560/Alien_Swarm_Reactive_Drop/) support.
//! - [Insurgency](https://store.steampowered.com/app/222880/Insurgency/) support.
//! - [Insurgency: Sandstorm](https://store.steampowered.com/app/581320/Insurgency_Sandstorm/) support.
//! - [Insurgency: Modern Infantry Combat](https://store.steampowered.com/app/17700/INSURGENCY_Modern_Infantry_Combat/) support.
//!
//! ## Breaking changes
//! - Changed uses a bit, example: from `use gamedig::valve::ValveProtocol::query` to `use gamedig::protocols::valve::query`.
//! - Changed Valve Protocol Query parameters to (`ip`, `port`, `app`, `gather_settings`).
//!  - `app` is now optional, being `None` means to anonymously query the server.
//!  - `gather_settings` is now optional, being `None` means all query settings will be used.
//!
//! ## Contributors
//! - [CosminPerRam](https://github.com/CosminPerRam)
