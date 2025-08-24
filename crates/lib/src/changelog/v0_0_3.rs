//! # v0.0.3 (22/10/2022) **Yanked**
//!
//! ## Summary
//! Improved Valve protocol with multi-packet support and added several Source engine games, while changing how some responses are handled.
//!
//! ## Fixes
//! - Valve protocol now properly supports multi-packet responses (compressed ones not tested).
//!
//! ## Features
//! - [Counter Strike: Source](https://store.steampowered.com/app/240/CounterStrike_Source/) support (if protocol is 7, queries with multi-packet responses will crash).
//! - [Day of Defeat: Source](https://store.steampowered.com/app/300/Day_of_Defeat_Source/) support.
//! - [Garry's Mod](https://store.steampowered.com/app/4000/Garrys_Mod/) support.
//! - [Half-Life 2 Deathmatch](https://store.steampowered.com/app/320/HalfLife_2_Deathmatch/) support.
//! - [Left 4 Dead](https://store.steampowered.com/app/500/Left_4_Dead/) support.
//! - [Left 4 Dead 2](https://store.steampowered.com/app/550/Left_4_Dead_2/) support.
//!
//! ## Breaking changes
//! - CSGO, TF2 and TS now have independent responses, if you want a generic one, query the protocol.
//!
//! ## Contributors
//! - [CosminPerRam](https://github.com/CosminPerRam)
