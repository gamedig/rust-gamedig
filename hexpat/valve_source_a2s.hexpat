// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: GameDig Organization & Contributors

// Editor: https://github.com/WerWolv/ImHex
// Documentation: https://docs.werwolv.net/documentation

#pragma author GameDig Organization & Contributors
#pragma description Valve Source Engine Query Protocol Packet

#pragma endian little

import std.mem;
import std.string;
import type.magic;

struct A2S_INFO {
    type::Magic<"Source Engine Query\0"> payload;

    if (std::mem::remaining() == 4) {
        u32 challenge;
    }
};

enum ServerType: u8 {
    Dedicated = 'd',
    Listen    = 'l',
    Proxy     = 'p'
};

enum Environment: u8 {
    Linux   = 'l',
    Windows = 'w',
    Mac     = 'm'
};

enum Visibility: u8 {
    Public  = 0,
    Private = 1
};

enum TheShipMode: u8 {
    Hunt            = 0,
    Elimination     = 1,
    Duel            = 2,
    Deathmatch      = 3,
    VIPTeam         = 4,
    TeamElimination = 5

};

bitfield ExtraDataFlag {
    bool game_id   : 1;  
    padding        : 3;  
    bool steam_id  : 1;  
    bool keywords  : 1;  
    bool source_tv : 1; 
    bool port      : 1; 
};

struct S2A_INFO {
    u8 protocol;

    std::string::NullString name;
    std::string::NullString map;
    std::string::NullString folder;
    std::string::NullString game;

    u16 id;
    u8 players;
    u8 max_players;
    u8 bots;

    u8 server_type_raw;
    ServerType server_type = server_type_raw | 0x20 [[export]];

    u8 environment_raw;
    u8 environment_norm = environment_raw | 0x20;
    Environment environment = (environment_norm == 'o') ? 'm' : environment_norm [[export]];

    Visibility visibility;
    bool vac;

    if (id == 2400) {
        TheShipMode mode;
        u8 witnesses;
        u8 duration;
    }

    std::string::NullString version;

    if (!std::mem::eof()) {
        ExtraDataFlag edf;

        if (edf.port) {
            u16 port;
        }

        if (edf.steam_id) {
            u64 steam_id;
        }

        if (edf.source_tv) {
            u16 source_tv_port;
            std::string::NullString source_tv_name;
        }

        if (edf.keywords) {
            std::string::NullString keywords;
        }

        if (edf.game_id) {
            u64 game_id;
        }
    }
};

struct A2S_PLAYER {
    if (std::mem::remaining() == 4) {
        u32 challenge;
    }
};

struct Player {
    u8 index;
    std::string::NullString name;
    s32 score;
    float duration;
};

struct TheShipPlayer: Player {
    u32 deaths;
    u32 money;
};

struct S2A_PLAYER {
    u8 num_players;
    
    try {
        TheShipPlayer players[num_players];
    } catch {
        Player players[num_players];
    }
};

struct A2S_RULES {
    if (std::mem::remaining() == 4) {
        u32 challenge;
    }
};

struct Rule {
    std::string::NullString name;
    std::string::NullString value;
};

struct S2A_RULES {
    u16 num_rules;
    Rule rules[num_rules];
};

struct S2A_CHALLENGE {
    u32 challenge;
};

enum PacketStructure: u32 {
    Complete   = 0xFFFFFFFF,
    Fragmented = 0xFFFFFFFE
};

enum PacketKind: u8 {
    A2S_INFO      = 0x54,
    S2A_INFO      = 0x49,

    A2S_PLAYER    = 0x55,
    S2A_PLAYER    = 0x44,

    A2S_RULES     = 0x56,
    S2A_RULES     = 0x45,

    S2A_CHALLENGE = 0x41
};

struct ValveSourceQueryPacket {
    PacketStructure header;

    if (header == PacketStructure::Complete) {
        PacketKind kind;

        match (kind) {
            (PacketKind::A2S_INFO):      A2S_INFO      body;
            (PacketKind::S2A_INFO):      S2A_INFO      body;
            (PacketKind::A2S_PLAYER):    A2S_PLAYER    body;
            (PacketKind::S2A_PLAYER):    S2A_PLAYER    body;
            (PacketKind::A2S_RULES):     A2S_RULES     body;
            (PacketKind::S2A_RULES):     S2A_RULES     body;
            (PacketKind::S2A_CHALLENGE): S2A_CHALLENGE body;
        }

    }
    else if (header == PacketStructure::Fragmented) {
        u32 request_id;
        bool is_compressed = (request_id >> 31) & 1 [[export]];
        u16 total_fragments;
        u16 fragment_number;
        u16 fragment_size;

        if (request_id == 0 && is_compressed) {
            u32 uncompressed_size;
            u32 crc32;
        }
    }
};

ValveSourceQueryPacket packet @ 0x00;