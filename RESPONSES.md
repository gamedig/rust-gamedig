Every protocol has its own response type(s), below is a listing of the overlapping fields on these responses.

If a cell is blank it doesn't exist, otherwise it contains the type of that data in the current column's response type.
In the case that a field that performs the same function exists in the current column's response type that name is annotated in brackets.

# Response table

| Field                | Generic | GameSpy(1)         | GameSpy(2) | GameSpy(3)         | Minecraft(Java)       | Minecraft(Bedrock) | Valve               | Quake            | Proprietary: FFOW  | Proprietary: TheShip |
|----------------------|---------|--------------------|------------|--------------------|-----------------------|--------------------|---------------------|------------------|--------------------|----------------------|
| name                 | Option  | String             | String     | String             |                       | String             | String              | String           | String             | String               |
| description          | Option  |                    |            |                    | String                |                    |                     |                  | String             |                      |
| game                 | Option  | String (game_type) |            | String (game_type) |                       | Option (game_mode) | String              |                  | String (game_mode) | String               |
| game_version         | Option  | String             |            | String             | String (version_name) |                    | String (version)    | String (version) | String (version)   | String (version)     |
| map                  | Option  | String             | String     | String             |                       | Option             | String              | String           | String             | String               |
| players_maximum      | u64     | usize              | usize      | usize              | u32                   | u32                | u8                  | u8               | u8                 | u8 (max_players)     |
| players_online       | u64     | usize              | usize      | usize              | u32                   | u32                | u8                  | u8               | u8                 | u8 (players)         |
| players_bots         | Option  |                    |            |                    |                       |                    | u8                  |                  |                    | u8 (bots)            |
| has_password         | Option  | bool               | bool       | bool               |                       |                    | bool                |                  | bool               | bool                 |
| map_title            |         | Option             |            |                    |                       |                    |                     |                  |                    |                      |
| admin_contact        |         | Option             |            |                    |                       |                    |                     |                  |                    |                      |
| admin_name           |         | Option             |            |                    |                       |                    |                     |                  |                    |                      |
| players_minimum      |         | Option             | Option     | Option             |                       |                    |                     |                  |                    |                      |
| players              |         | Vec                | Vec        | Vec                |                       |                    | Option>             | Vec              |                    | Vec (player_details) |
| tournament           |         | bool               |            | bool               |                       |                    |                     |                  |                    |                      |
| unused_entries       |         | Hashmap            |            | HashMap            |                       |                    | Option (extra_data) | HashMap          |                    |                      |
| teams                |         |                    | Vec        | Vec                |                       |                    |                     |                  |                    |                      |
| version_protocol     |         |                    |            |                    | i32                   | String             | u8 (protocol)       |                  | u8 (protocol)      | u8 (protocol)        |
| players_sample       |         |                    |            |                    | Option>               |                    |                     |                  |                    |                      |
| favicon              |         |                    |            |                    | Option                |                    |                     |                  |                    |                      |
| previews_chat        |         |                    |            |                    | Option                |                    |                     |                  |                    |                      |
| enforces_secure_chat |         |                    |            |                    | Option                |                    |                     |                  |                    |                      |
| server_type          |         |                    |            |                    | Server                | Server             | Server              |                  |                    | Server               |
| edition              |         |                    |            |                    |                       | String             |                     |                  |                    |                      |
| id                   |         |                    |            |                    |                       | String             |                     |                  |                    |                      |
| rules                |         |                    |            |                    |                       |                    | Option>             |                  |                    | HashMap              |
| folder               |         |                    |            |                    |                       |                    | String              |                  |                    |                      |
| appid                |         |                    |            |                    |                       |                    | u32                 |                  |                    |                      |
| environment_type     |         |                    |            |                    |                       |                    | Environment         |                  | Environment        |                      |
| vac_secured          |         |                    |            |                    |                       |                    | bool                |                  | bool               | bool                 |
| the_ship             |         |                    |            |                    |                       |                    | Option              |                  |                    |                      |
| is_mod               |         |                    |            |                    |                       |                    | bool                |                  |                    |                      |
| mod_data             |         |                    |            |                    |                       |                    | Option              |                  |                    |                      |
| active_mod           |         |                    |            |                    |                       |                    |                     |                  | String             |                      |
| round                |         |                    |            |                    |                       |                    |                     |                  | u8                 |                      |
| rounds_maximum       |         |                    |            |                    |                       |                    |                     |                  | u8                 |                      |
| time_left            |         |                    |            |                    |                       |                    |                     |                  | u16                |                      |
| port                 |         |                    |            |                    |                       |                    |                     |                  |                    | Option               |
| steam_id             |         |                    |            |                    |                       |                    |                     |                  |                    | Option               |
| tv_port              |         |                    |            |                    |                       |                    |                     |                  |                    | Option               |
| tv_name              |         |                    |            |                    |                       |                    |                     |                  |                    | Option               |
| keywords             |         |                    |            |                    |                       |                    |                     |                  |                    | Option               |
| mode                 |         |                    |            |                    |                       |                    |                     |                  |                    | u8                   |
| witnesses            |         |                    |            |                    |                       |                    |                     |                  |                    | u8                   |
| duration             |         |                    |            |                    |                       |                    |                     |                  |                    | u8                   |
