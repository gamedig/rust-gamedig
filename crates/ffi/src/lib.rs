// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *    ____                      ____  _         _____ _____ ___    *
// *   / ___| __ _ _ __ ___   ___|  _ \(_) __ _  |  ___|  ___|_ _|   *
// *  | |  _ / _` | '_ ` _ \ / _ \ | | | |/ _` | | |_  | |_   | |    *
// *  | |_| | (_| | | | | | |  __/ |_| | | (_| | |  _| |  _|  | |    *
// *   \____|\__,_|_| |_| |_|\___|____/|_|\__, | |_|   |_|   |___|   *
// *                                      |___/                      *
// *                 Copyright (c) 2022 - 2026                       *
// *            GameDig Organization & Contributors                  *
// *                                                                 *
// *               Licensed under the MIT License                    *
// *  See the LICENSE file in the project root for more information  *
// *                                                                 *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use {
    gamedig::{
        converters::{
            GenericPlayer,
            GenericServer,
            HttpTimeout,
            TcpTimeout,
            TimeoutConfig,
            UdpTimeout,
        },
        dict::Dict,
    },
    std::{
        ffi::{CStr, CString, c_char},
        ptr,
        time::Duration,
    },
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamedig_tcp_timeout_t {
    pub connect_ms: u32,
    pub read_ms: u32,
    pub write_ms: u32,
}

impl From<gamedig_tcp_timeout_t> for TcpTimeout {
    fn from(v: gamedig_tcp_timeout_t) -> Self {
        Self {
            connect: (v.connect_ms != 0).then(|| Duration::from_millis(v.connect_ms as u64)),
            read: (v.read_ms != 0).then(|| Duration::from_millis(v.read_ms as u64)),
            write: (v.write_ms != 0).then(|| Duration::from_millis(v.write_ms as u64)),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamedig_udp_timeout_t {
    pub read_ms: u32,
    pub write_ms: u32,
}

impl From<gamedig_udp_timeout_t> for UdpTimeout {
    fn from(v: gamedig_udp_timeout_t) -> Self {
        Self {
            read: (v.read_ms != 0).then(|| Duration::from_millis(v.read_ms as u64)),
            write: (v.write_ms != 0).then(|| Duration::from_millis(v.write_ms as u64)),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gamedig_http_timeout_t {
    pub global_ms: u32,
}

impl From<gamedig_http_timeout_t> for HttpTimeout {
    fn from(v: gamedig_http_timeout_t) -> Self {
        Self {
            global: (v.global_ms != 0).then(|| Duration::from_millis(v.global_ms as u64)),
        }
    }
}

#[repr(C)]
pub struct gamedig_timeout_config_t {
    pub tcp: gamedig_tcp_timeout_t,
    pub udp: gamedig_udp_timeout_t,
    pub http: gamedig_http_timeout_t,
}

impl From<&gamedig_timeout_config_t> for TimeoutConfig {
    fn from(v: &gamedig_timeout_config_t) -> Self {
        Self {
            tcp: v.tcp.into(),
            udp: v.udp.into(),
            http: v.http.into(),
        }
    }
}

impl gamedig_timeout_config_t {
    pub unsafe fn to_timeout_config(ptr: *const Self) -> TimeoutConfig {
        if ptr.is_null() {
            return TimeoutConfig::default();
        }

        if (ptr as usize) % std::mem::align_of::<Self>() != 0 {
            return TimeoutConfig::default();
        }

        TimeoutConfig::from(unsafe { &*ptr })
    }
}

#[repr(C)]
pub enum gamedig_opt_bool_t {
    GAMEDIG_BOOL_FALSE = 0,
    GAMEDIG_BOOL_TRUE = 1,
    GAMEDIG_BOOL_UNKNOWN = 2,
}

impl From<Option<bool>> for gamedig_opt_bool_t {
    fn from(v: Option<bool>) -> Self {
        match v {
            Some(true) => Self::GAMEDIG_BOOL_TRUE,
            Some(false) => Self::GAMEDIG_BOOL_FALSE,
            None => Self::GAMEDIG_BOOL_UNKNOWN,
        }
    }
}

#[repr(C)]
pub struct gamedig_player_t {
    pub id: u16,
    pub name: *const c_char,
}

impl From<&GenericPlayer> for gamedig_player_t {
    fn from(p: &GenericPlayer) -> Self {
        Self {
            id: p.id,
            name: p
                .name
                .as_deref()
                .and_then(|s| CString::new(s).ok())
                .map_or(ptr::null(), |c| c.into_raw()),
        }
    }
}

#[repr(C)]
pub struct gamedig_player_list_t {
    pub data: *const gamedig_player_t,
    pub len: usize,
}

impl From<Option<&[GenericPlayer]>> for gamedig_player_list_t {
    fn from(players: Option<&[GenericPlayer]>) -> Self {
        match players {
            None => {
                Self {
                    data: ptr::null(),
                    len: 0,
                }
            }
            Some(slice) if slice.is_empty() => {
                Self {
                    data: ptr::null(),
                    len: 0,
                }
            }
            Some(slice) => {
                let mut out = Vec::with_capacity(slice.len());
                for p in slice {
                    out.push(gamedig_player_t::from(p));
                }

                let len = out.len();
                let boxed = out.into_boxed_slice();
                let data = Box::into_raw(boxed) as *const gamedig_player_t;

                Self { data, len }
            }
        }
    }
}

#[repr(C)]
pub struct gamedig_server_t {
    pub name: *const c_char,
    pub description: *const c_char,
    pub map: *const c_char,
    pub mode: *const c_char,
    pub version: *const c_char,
    pub anti_cheat: gamedig_opt_bool_t,
    pub has_password: gamedig_opt_bool_t,
    pub max_players: u16,
    pub current_players: u16,
    pub players: *const gamedig_player_list_t,
}

impl From<&GenericServer> for gamedig_server_t {
    fn from(s: &GenericServer) -> Self {
        let players_ptr: *const gamedig_player_list_t = match s.players.as_deref() {
            None => ptr::null(),
            Some(slice) => {
                let list: gamedig_player_list_t = Some(slice).into();
                Box::into_raw(Box::new(list)) as *const gamedig_player_list_t
            }
        };

        Self {
            name: CString::new(s.name.as_str())
                .unwrap_or_else(|_| CString::new("").unwrap())
                .into_raw(),

            description: s
                .description
                .as_deref()
                .and_then(|x| CString::new(x).ok())
                .map_or(ptr::null(), |c| c.into_raw()),

            map: s
                .map
                .as_deref()
                .and_then(|x| CString::new(x).ok())
                .map_or(ptr::null(), |c| c.into_raw()),

            mode: s
                .mode
                .as_deref()
                .and_then(|x| CString::new(x).ok())
                .map_or(ptr::null(), |c| c.into_raw()),

            version: s
                .version
                .as_deref()
                .and_then(|x| CString::new(x).ok())
                .map_or(ptr::null(), |c| c.into_raw()),

            anti_cheat: s.anti_cheat.into(),
            has_password: s.has_password.into(),

            max_players: s.max_players,
            current_players: s.current_players,

            players: players_ptr,
        }
    }
}

pub type gamedig_error_t = *mut c_char;

#[unsafe(no_mangle)]
pub extern "C" fn gamedig_query(
    game_id: *const c_char,
    addr: *const c_char,
    timeout: *const gamedig_timeout_config_t,
    out_srv: *mut gamedig_server_t,
    out_error: *mut gamedig_error_t,
) -> i32 {
    if out_srv.is_null() {
        return -1;
    }

    let (game_id, addr) = unsafe {
        if game_id.is_null() || addr.is_null() {
            return -1;
        }

        let game_id = match CStr::from_ptr(game_id).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        let addr = match CStr::from_ptr(addr).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        (game_id, addr)
    };

    let timeout_opt = unsafe {
        if timeout.is_null() {
            None
        } else {
            Some(gamedig_timeout_config_t::to_timeout_config(timeout))
        }
    };

    let res = Dict::query_by_game_id(game_id, addr, timeout_opt);

    match res {
        Ok(server) => unsafe {
            *out_srv = gamedig_server_t::from(&server);

            if !out_error.is_null() {
                *out_error = ptr::null_mut();
            }

            0
        },

        Err(err) => {
            if !out_error.is_null() {
                let msg = err.to_string();
                let cstr = CString::new(msg).unwrap_or_else(|_| CString::new("error").unwrap());
                unsafe {
                    *out_error = cstr.into_raw();
                }
            }

            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gamedig_ffi_free_server(server: *mut gamedig_server_t) {
    unsafe {
        if server.is_null() {
            return;
        }

        let srv = &mut *server;

        unsafe fn free_cstr(p: &mut *const c_char) {
            if !p.is_null() {
                drop(unsafe { CString::from_raw(*p as *mut c_char) });
                *p = ptr::null();
            }
        }

        free_cstr(&mut srv.name);
        free_cstr(&mut srv.description);
        free_cstr(&mut srv.map);
        free_cstr(&mut srv.mode);
        free_cstr(&mut srv.version);

        if !srv.players.is_null() {
            let list_ptr = srv.players as *mut gamedig_player_list_t;
            let list = &mut *list_ptr;

            if !list.data.is_null() {
                let slice_ptr =
                    ptr::slice_from_raw_parts_mut(list.data as *mut gamedig_player_t, list.len);

                let boxed_players: Box<[gamedig_player_t]> = Box::from_raw(slice_ptr);

                for p in boxed_players.iter() {
                    if !p.name.is_null() {
                        drop(CString::from_raw(p.name as *mut c_char));
                    }
                }
            }

            drop(Box::from_raw(list_ptr));
            srv.players = ptr::null();
        }

        srv.anti_cheat = gamedig_opt_bool_t::GAMEDIG_BOOL_UNKNOWN;
        srv.has_password = gamedig_opt_bool_t::GAMEDIG_BOOL_UNKNOWN;
        srv.max_players = 0;
        srv.current_players = 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gamedig_ffi_free_error(error: gamedig_error_t) {
    unsafe {
        if !error.is_null() {
            drop(CString::from_raw(error));
        }
    }
}
