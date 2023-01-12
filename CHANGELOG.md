
Who knows what the future holds...

# X - DD/MM/YYYY
### Changes:
Games: [Risk of Rain 2](https://store.steampowered.com/app/632360/Risk_of_Rain_2/) support.  
Valve Protocol: Players with no name are no more added to the `players_details` field.

### Breaking:
Valve Protocol: The rules field is now a `HashMap<String, String>` instead of a `Vec<ServerRule>` (where the `ServerRule` structure had a name and a value fields).  
Errors: Besides the `BadGame` error, now no other errors returns details about what happened (as it was quite pointless).

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
