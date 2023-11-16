
Who knows what the future holds...

# 0.X.Y - DD/MM/YYYY
### Changes:
Games:
- [Valheim](https://store.steampowered.com/app/892970/Valheim/) support.
- [The Front](https://store.steampowered.com/app/2285150/The_Front/) support.
- [Conan Exiles](https://store.steampowered.com/app/440900/Conan_Exiles/) support.
- Added a valve protocol query example.

Protocols:
- Added the unreal2 protocol and its associated games: Darkest Hour, Devastation, Killing Floor, Red Orchestra, Unreal Tournament 2003, Unreal Tournament 2004 (by @Douile).

CLI:
- Added a CLI (by @cainthebest).

### Breaking:
Game:
- Changed identifications of the following games as they weren't properly expecting the naming rules:
- - Left 4 Dead: `left4dead` -> `l4d`.
- - 7 Days to Die: `7d2d` in definitions and `sd2d` in game declaration -> `sdtd`.
- - Quake 3 Arena: `quake3arena` -> `q3a`.
- Minecraft Legacy 1.5 and 1.3 were renamed to 1.4 and beta 1.8 respectively to show the lowest version they support, this change includes Structs, Enum and game id renames, also removed the "v" from the game definition name.

Protocols:
- Valve: Removed `SteamApp` due to it not being really useful at all, replaced all instances with `Engine`.

# 0.4.1 - 13/10/2023
### Changes:
Game:
- Added [Barotrauma](https://store.steampowered.com/app/602960/Barotrauma/) support.

Crate:
- Added `Send` and `Sync` on `Error::source` to fix some async issues.

Protocols:
- Minecraft Java: Add derives to `RequestSettings` and add `new_just_hostname` that creates new settings just by specifying
the hostname, `protocol_version` defaults to -1.

Games:
- Organised game modules into protocols (when protocol used by other games),
  you can now access a game by its name or by its protocol name:
  - `use gamedig::games::teamfortress2;`
  - `use gamedig::games::valve::teamfortress2;`

Generics:
- Added standard derives to `ProprietaryProtocol`, `CommonResponseJson`, `CommonPlayerJson`, `TimeoutSettings` and
`ExtraRequestSettings`.

### Breaking...
None, yaay!

# 0.4.0 - 07/10/2023
### Changes:
Games:
- [Creativerse](https://store.steampowered.com/app/280790/Creativerse/) support.

Protocols:
- Quake 2: Fixed a bug where the version tag wouldn't always be present.
- The Ship: Removed instances of using `unwrap` without handling the panics.

Crate:
- Updated [byteorder](https://crates.io/crates/byteorder) dependency from 1.4 to 1.5.
- Rich errors, capturing backtrace is done on `RUST_BACKTRACE=1`. (by @Douile)
- Applied some nursery Clippy lints.
- The `retries` field was added to `TimeoutSettings` that specifies the number of times to retry a failed request (request being individual send, receive sequence, some protocols can include multiple requests in a single query). (by @Douile)
    - By default `retries` is set to `0`, meaning no retries will be attempted

Generics:
- Added `ExtraRequestSettings` containing all possible extra request settings. (by @Douile)
- Added `query_with_timeout_and_extra_settings()` to allow generic queries with extra settings. (by @Douile)

### Breaking...
Crate:
- The enum used for errors, `GDError` has been renamed to `GDErrorKind`.
- `GDError` is now a struct that holds its kind, the source and a backtrace.
- The `Socket::apply_timeout` method now borrows `TimeoutSettings` (`&Option<TimeoutSettings>`)
    - To make this easier to work with a new method was added to `TimeoutSettings`: `TimeoutSettings::get_read_and_write_or_defaults` this takes a borrowed optional `TimeoutSettings` and returns the contained read and write durations or the default read and write durations.

Generics:
- Renamed `CommonResponseJson`'s `game` field (and the function) to `game_mode`.
- Changed `players_maximum` and `players_online` (and their functions) types from `u64` to `u32`.
- Changed `score` type (and the function) of player from `u32` to `i32`.

Games:
- Rename some game definitions and implementations to follow a stable ID naming system.

Protocols:
- Valve:
1. Renamed `protocol` to `protocol_version`.
2. Renamed `version` to `game_version`.
3. Renamed `game` to `game_mode`.
4. Fixed `player`'s `score` field being `u32` when it needed to be `i32`, as specified in the protocol.
5. Added the field `check_app_id` to `GatherSettings` which controls if the app id specified to the request and 
reported by the server are the same, errors if not, enabled by default. (by @Douile)
6. Valve: Renamed SteamApp enum variants to match new definition names

- GameSpy (1, 2, 3):
1. Renamed `version` to `game_version`.
2. Changed `players_maximum` and `players_online` (and their functions) types from `usize` to `u32`.

- GameSpy 1:
1. Renamed the player's `frags` to `score` and type from `u32` to `i32`.
2. Made `Option` the following response fields `team`, `face`, `skin`, `mesh` and `secret` to fix missing fields issues. (by @Douile)

- Quake (1, 2):
1. Renamed `game_type` to `game_mode`.
2. Changed `version` type from `String`to `Option<String>`.

- Minecraft Java
1. Renamed `version_protocol` to `protocol_version`.
2. Renamed `version_name` to `game_version`.
3. Renamed `players_sample` to `players`.
4. Added an optional parameter, `RequestSettings`, which contains fields that are used when creating the handshake
packet (this solves some servers not responding to the query). (by @Douile)
5. Legacy versions naming has been changed to represent up to what version they can query, `LegacyBV1_8` (Beta 1.8 to 
1.3) -> `LegacyV1_3` and `LegacyV1_4` (1.4 to 1.5) -> `LegacyV1_5` (and their enums accordingly).

- Minecraft Bedrock
1. Renamed `version_protocol` to `protocol_version`.

- Minecraft:
1. Added an optional parameter, `request_settings` parameter to `query`.

- The Ship:
1. Renamed `protocol` to `protocol_version`.
2. Renamed `max_players` to `players_maximum` and changed its type from `u64` to `u32`.
3. Renamed `bots` to `players_bots`. and changed its type from `u64` to `u32`.
4. Renamed `players` to `players_online`.
5. Renamed `players_details` to `players`.
6. Renamed `game` to `game_mode`.
7. Added field `game_version`.
8. Changed `players_bots` type from `Option<u64>` to `Option<u32>`.
9. Changed `score` type of player from `u32` to `i32`.

- Frontlines: Fuel of War:
1. Renamed `game_mode` to `game`.
2. Renamed `version` to `game_version`.
3. Renamed `protocol` to `protocol_version`.
4. Renamed `game` to `game_mode`.
5. Changed `players_maximum` and `players_minimum` types from `usize` to `u32`.

- Just Cause 2: Multiplayer:
1. Renamed `version` to `game_version`.
2. Changed `players_maximum` and `players_minimum` types from `usize` to `u32`.

# 0.3.0 - 18/07/2023
### Changes:
Protocols:
- GameSpy 2 support.
- Quake 2: Added Optional address field to Player.

Generic query:
- Added generic queries (by [@Douile](https://github.com/Douile)) which come with a common struct for the response fields.
- The supported games list is available programmatically.

Games:
- [Halo: Combat Evolved](https://en.wikipedia.org/wiki/Halo:_Combat_Evolved) support.
- [Just Cause 2: Multiplayer](https://store.steampowered.com/app/259080/Just_Cause_2_Multiplayer_Mod/) support.
- [Warsow](https://warsow.net/) support.

Internal:
- Buffer reader rewrite, resulting in more data checks and better code quality (thanks [@cainthebest](https://github.com/cainthebest)).
- Better CI to never break accidentally MSRV again (thanks [@Douile](https://github.com/Douile)).

### Breaking...
Protocols:
- Quake 2: Renamed the players `frags` field to `score` to be more inline with the other protocols.

Crate:
- `no_games` and `no_services` have been changed to `games` and `services`, this better represents that they are present by default (by [@Douile](https://github.com/Douile)).
- Fixed crate's `rust-version`, it is now `1.60.0` (was `1.56.1`)

# 0.2.3 - 02/06/2023
### Changes:
Protocols:
- Valve:
1. Added standard and serde derives to `GatheringSettings`.
- Quake 1, 2 and 3 support.

Games:
- [Quake 2](https://store.steampowered.com/app/2320/Quake_II/) support.
- [Quake 1](https://store.steampowered.com/app/2310/Quake/) support.
- [Quake 3: Arena](https://store.steampowered.com/app/2200/Quake_III_Arena/) support.
- [Hell Let Loose](https://store.steampowered.com/app/686810/Hell_Let_Loose/) support.
- [Soldier of Fortune 2](https://www.gog.com/en/game/soldier_of_fortune_ii_double_helix_gold_edition) support.

### Breaking:
- Every function that used `&str` for the address has been changed to `&IpAddr` (thanks [@Douile](https://github.com/Douile) for the re-re-write).
- Protocols now use `&SocketAddr` instead of `address: &str, port: u16`.

Services:
- Valve Master Query:
1. Removed Filter and SearchFilters lifetimes and changed `&'a str` to `String` and `&'a [&'a str]` to `Vec<String>`

# 0.2.2 - 01/05/2023
### Changes:
Crate:
- General optimizations thanks to [cargo clippy](https://github.com/rust-lang/rust-clippy) and [@cainthebest](https://github.com/cainthebest).
- Added feature `serde` which enables json serialization/deserialization for all types (by [@cainthebest](https://github.com/cainthebest)).
- Documentation improvements.

Protocols:
- GameSpy 1: Add key `admin` as a possible variable for `admin_name`.
- GameSpy 3 support.

Games:
- [Serious Sam](https://www.gog.com/game/serious_sam_the_first_encounter) support.
- [Frontlines: Fuel of War](https://store.steampowered.com/app/9460/Frontlines_Fuel_of_War/) support.
- [Crysis Wars](https://steamcommunity.com/app/17340) support.

Services:
- [Valve Master Server Query](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol) support.
- Added feature `no_services` which disables the supported services.

### Breaking:
Protocols:
- Valve: Request type enums have been renamed from all caps to starting-only uppercase, ex: `INFO` to `Info`
- GameSpy 1: `players_minimum` is now an `Option<u8>` instead of an `u8`
- GameSpy 1: Is now under `protocols::gamespy::one` instead of `protocols::gamespy`

# 0.2.1 - 03/03/2023
### Changes:
Crate:
- Added feature `no_games` which disables the supported games (useful when only the 
protocols/services are needed, also saves storage space).

Games:
- [V Rising](https://store.steampowered.com/app/1604030/V_Rising/) support.
- [Unreal Tournament](https://en.wikipedia.org/wiki/Unreal_Tournament) support.
- [Battlefield 1942](https://www.ea.com/games/battlefield/battlefield-1942) support.

Protocols:
- Valve:
1. Reversed (from `0.1.0`) "Players with no name are no more added to the `players_details` field.", also added a note in the [protocols](PROTOCOLS.md) file regarding this.
2. Fixed querying while multiple challenge responses might happen.
- GameSpy 1 support.

### Breaking:
None.

# 0.2.0 - 18/02/2023
### Changes:
Games:
- [Don't Starve Together](https://store.steampowered.com/app/322330/Dont_Starve_Together/) support.
- [Colony Survival](https://store.steampowered.com/app/366090/Colony_Survival/) support.
- [Onset](https://store.steampowered.com/app/1105810/Onset/) support.
- [Codename CURE](https://store.steampowered.com/app/355180/Codename_CURE/) support.
- [Ballistic Overkill](https://store.steampowered.com/app/296300/Ballistic_Overkill/) support.
- [BrainBread 2](https://store.steampowered.com/app/346330/BrainBread_2/) support.
- [Avorion](https://store.steampowered.com/app/445220/Avorion/) support.
- [Operation: Harsh Doorstop](https://store.steampowered.com/app/736590/Operation_Harsh_Doorstop/) support.

Protocols:
- Valve:
1. `appid` is now a field in the `Response` struct.

### Breaking:
Protocols:
- Valve:
due to some games being able to host a server from within the game AND from a dedicated server, 
if you were to query one of them, the query would fail for the other one, as the `SteamID` enum
for that game could specify only one id.
1. `SteamID` is now `SteamApp`, was an u32 enum, and now it's a simple enum.  
2. `App` is now `Engine`, the `Source` enum's structure has been changed from `Option<u32>` to
`Option<u32, Option<u32>>`, where the first parameter is the game app id and the second is
the dedicated server app id (if there is one).

# 0.1.0 - 17/01/2023
### Changes:
Games: 
- [Risk of Rain 2](https://store.steampowered.com/app/632360/Risk_of_Rain_2/) support.
- [Battalion 1944](https://store.steampowered.com/app/489940/BATTALION_Legacy/) support. 
- [Black Mesa](https://store.steampowered.com/app/362890/Black_Mesa/) support.
- [Project Zomboid](https://store.steampowered.com/app/108600/Project_Zomboid/) support.
- [Age of Chivalry](https://store.steampowered.com/app/17510/Age_of_Chivalry/) support.

Protocols:
- Valve: Players with no name are no more added to the `players_details` field.
- Valve: Split packets are now appending in the correct order.

Crate: 
- `MSRV` is now `1.56.1` (was `1.58.1`)

### Breaking:
Protocols:
- Valve: The rules field is now a `HashMap<String, String>` instead of a `Vec<ServerRule>` (where the `ServerRule` structure had a name and a value fields).
- Valve: Structs that contained the `players`, `max_players` and `bots` fields have been renamed to `players_online`, `players_maximum` and `players_bots` respectively.
- Minecraft: Structs that contained the `online_players`, `max_players` and `sample_players` fields have been renamed to `players_online`, `players_maximum` and `players_sample` respectively.
- Minecraft: The Java query response struct named `Response` has been renamed to `JavaResponse`.

Errors: 
- Besides the `BadGame` error, now no other errors returns details about what happened (as it was quite pointless).  

Crate: 
- `package.metadata.msrv` has been replaced with `package.rust-version`

# 0.0.7 - 03/01/2023
### Changes:
[Minecraft](https://www.minecraft.com) bedrock edition support.  
Fix Minecraft legacy v1.6 max/online players count being reversed.  
Added `query_legacy_specific` method to the Minecraft protocol.

### Breaking:  
Removed `query_specific` from the mc protocol in favor of `query_java`, `query_legacy` and `query_legacy_specific`.  
Some public functions that are meant to be used only internally were made private.

# 0.0.6 - 28/11/2022
[Minecraft](https://www.minecraft.com) support (bedrock not supported yet).  
[7 Days To Die](https://store.steampowered.com/app/251570/7_Days_to_Die/) support.  
[ARK: Survival Evolved](https://store.steampowered.com/app/346110/ARK_Survival_Evolved/) support.  
[Unturned](https://store.steampowered.com/app/304930/Unturned/) support.  
[The Forest](https://store.steampowered.com/app/242760/The_Forest/) support.  
[Team Fortress Classic](https://store.steampowered.com/app/20/Team_Fortress_Classic/) support.  
[Sven Co-op](https://store.steampowered.com/app/225840/Sven_Coop/) support.  
[Rust](https://store.steampowered.com/app/252490/Rust/) support.  
[Counter-Strike](https://store.steampowered.com/app/10/CounterStrike/) support.  
[Arma 2: Operation Arrowhead](https://store.steampowered.com/app/33930/Arma_2_Operation_Arrowhead/) support.  
[Day of Infamy](https://store.steampowered.com/app/447820/Day_of_Infamy/) support.  
[Half-Life Deathmatch: Source](https://store.steampowered.com/app/360/HalfLife_Deathmatch_Source/) support.  
Successfully tested `Alien Swarm` and `Insurgency: Modern Infantry Combat`.  
Restored rules response for `Counter-Strike: Global Offensive` (note: for a full player list response, the cvar `host_players_show` must be set to `2`).  
Increased Valve Protocol `PACKET_SIZE` from 1400 to 6144 (because some games send larger packets than the specified protocol size).  
Removed DNS resolving as it was not needed.  
Valve Protocol minor optimizations.  

# 0.0.5 - 15/11/2022
Added `SocketBind` error, regarding failing to bind a socket.  
Socket custom timeout capability (with an error if provided durations are zero).  
Because of this, a parameter similar to GatherSettings has been added on the Valve Protocol Query.
Support for GoldSrc split packets and obsolete A2S_INFO response.  
Changed the Valve Protocol app parameter to represent the engine responses.
It is now an enum of:
- `Source(Option<u32>)` - A Source response with optionally, the id (if the id is present and the response id is not the same, the query fails), if it isn't provided, find it.
- `GoldSrc(bool)` - A GoldSrc response with the option to enforce the obsolete A2S_INFO response.

Fixed Source multi-packet response crash due to when a certain app with a certain protocol doesn't have the Size field.  
Reduced Valve Protocol `PACKET_SIZE` to be as specified from 2048 to 1400.  
[Counter-Strike: Condition Zero](https://store.steampowered.com/app/80/CounterStrike_Condition_Zero/) implementation.  
[Day of Defeat](https://store.steampowered.com/app/30/Day_of_Defeat/) implementation.  
Games besides CSGO and TS now have the same response structure.  

# 0.0.4 - 23/10/2022
Queries now support DNS resolve.  
Changed uses a bit, example: from `use gamedig::valve::ValveProtocol::query` to `use gamedig::protocols::valve::query`.  
Changed Valve Protocol Query parameters to (ip, port, app, gather_settings), changes include:
- the app is now optional, being None means to anonymously query the server.
- gather_settings is now also an optional, being None means all query settings.  

Valve Protocol now supports querying anonymous apps (see previous lines).  
Better bad game error.  
[Alien Swarm](https://store.steampowered.com/app/630/Alien_Swarm/) implementation (not tested).  
[Alien Swarm: Reactive Drop](https://store.steampowered.com/app/563560/Alien_Swarm_Reactive_Drop/) implementation.  
[Insurgency](https://store.steampowered.com/app/222880/Insurgency/) implementation.  
[Insurgency: Sandstorm](https://store.steampowered.com/app/581320/Insurgency_Sandstorm/) implementation.  
[Insurgency: Modern Infantry Combat](https://store.steampowered.com/app/17700/INSURGENCY_Modern_Infantry_Combat/) implementation (not tested).  

# 0.0.3 - 22/10/2022
Valve protocol now properly supports multi-packet responses (compressed ones not tested).  
CSGO, TF2 and TS now have independent Responses, if you want a generic one, query the protocol.  
[Counter Strike: Source](https://store.steampowered.com/app/240/CounterStrike_Source/) implementation (if protocol is 7, queries with multi-packet responses will crash).  
[Day of Defeat: Source](https://store.steampowered.com/app/300/Day_of_Defeat_Source/) implementation.  
[Garry's Mod](https://store.steampowered.com/app/4000/Garrys_Mod/) implementation.  
[Half-Life 2 Deathmatch](https://store.steampowered.com/app/320/HalfLife_2_Deathmatch/) implementation.  
[Left 4 Dead](https://store.steampowered.com/app/500/Left_4_Dead/) implementation.  
[Left 4 Dead 2](https://store.steampowered.com/app/550/Left_4_Dead_2/) implementation.  

# 0.0.2 - 20/10/2022
Further implementation of the Valve protocol (PLAYERS and RULES queries).  
[Counter Strike: Global Offensive](https://store.steampowered.com/app/730/CounterStrike_Global_Offensive/) implementation.  
[The Ship](https://developer.valvesoftware.com/wiki/The_Ship) implementation.  
The library now has error handling.

# 0.0.1 - 16/10/2022
The first usable version of the crate, yay!  
It brings:  
Initial implementation of the [Valve server query protocol](https://developer.valvesoftware.com/wiki/Server_queries).  
Initial [Team Fortress 2](https://en.wikipedia.org/wiki/Team_Fortress_2) support.

# 0.0.0 - 15/10/2022
The first *markdown*, the crate is unusable as it doesn't contain anything helpful.
