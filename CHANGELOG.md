
Who knows what the future holds...

# 0.0.5 - ??/??/2022
Support for GoldSrc split packets and obsolete A2S_INFO response.  
Changed the Valve Protocol app parameter to represent the engine responses.
It is now an enum of:
- `Source(Option<u32>)` - A Source response with optionally, the id (if the id is present and the response id is not the same, the query fails)
- `GoldSrc(bool)` - A GoldSrc response with the option to enforce the obsolete A2S_INFO response.

[Counter-Strike: Condition Zero](https://store.steampowered.com/app/80/CounterStrike_Condition_Zero/) implementation.  
[Day of Defeat](https://store.steampowered.com/app/30/Day_of_Defeat/) implementation.  
Games besides CSGO and TS now have the same response structure.  
Fixed Source multipacket response crash due to when a certain app with a certain protocol doesnt have the Size field.

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
