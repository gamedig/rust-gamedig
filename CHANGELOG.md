
Who knows what the future holds...

# 0.X.Y - DD/MM/2023
### Changes:
Protocols:
- Quake 2: Fixed a bug where the version tag wouldn't always be present.

### Breaking...
Protocols:
- Gamespy 1: Renamed the players `frags` field to `score` to be more inline with the other protocols.
- Quake: The `version` field has been changed from `String`to `Option<String>`.

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
