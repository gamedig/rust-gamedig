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
use {
    gamedig::{
        converters::{
            GenericDataMap,
            GenericDataValue,
            GenericPlayer,
            GenericServer,
            HttpTimeout,
            TcpTimeout,
            TimeoutConfig,
            UdpTimeout,
        },
        dict::Dict,
    },
    neon::prelude::*,
    std::{net::SocketAddr, str::FromStr, thread, time::Duration},
};

struct QueryArgs {
    game_id: String,
    addr: SocketAddr,
    timeout: Option<TimeoutConfig>,
}

impl QueryArgs {
    fn parse(cx: &mut FunctionContext) -> NeonResult<Self> {
        let game_id = cx.argument::<JsString>(0usize)?.value(cx);

        let addr_s = cx.argument::<JsString>(1usize)?.value(cx);
        let addr = match SocketAddr::from_str(&addr_s) {
            Ok(a) => a,
            Err(_) => return cx.throw_error("addr must be like \"127.0.0.1:27015\""),
        };

        let timeout = Timeout::parse(cx, 2usize)?.map(Timeout::into_config);

        Ok(Self {
            game_id,
            addr,
            timeout,
        })
    }

    fn run(self) -> Result<GenericServer, String> {
        Dict::query_by_game_id(&self.game_id, &self.addr, self.timeout).map_err(|r| r.to_string())
    }

    fn schedule(self, mut cx: FunctionContext) -> JsResult<JsPromise> {
        let channel = cx.channel();
        let (deferred, promise) = cx.promise();

        thread::spawn(move || {
            let result = self.run();

            channel.send(move |mut cx| {
                match result {
                    Ok(server) => {
                        let js = JsServer::from_rust(&mut cx, &server)?;
                        deferred.resolve(&mut cx, js);
                    }
                    Err(msg) => {
                        let err = cx.error(msg)?;
                        deferred.reject(&mut cx, err);
                    }
                }
                Ok(())
            });
        });

        Ok(promise)
    }
}

struct Timeout {
    tcp: TcpTimeout,
    udp: UdpTimeout,
    http: HttpTimeout,
}

impl Timeout {
    fn parse(cx: &mut FunctionContext, idx: usize) -> NeonResult<Option<Self>> {
        let Some(v) = cx.argument_opt(idx) else {
            return Ok(None);
        };
        if Self::is_nullish(cx, v) {
            return Ok(None);
        }

        let root = v.downcast_or_throw::<JsObject, _>(cx)?;

        let tcp_v = root.get(cx, "tcp")?;
        let udp_v = root.get(cx, "udp")?;
        let http_v = root.get(cx, "http")?;

        Ok(Some(Self {
            tcp: Self::tcp(cx, tcp_v)?,
            udp: Self::udp(cx, udp_v)?,
            http: Self::http(cx, http_v)?,
        }))
    }

    fn into_config(self) -> TimeoutConfig {
        TimeoutConfig {
            tcp: self.tcp,
            udp: self.udp,
            http: self.http,
        }
    }

    fn tcp<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> NeonResult<TcpTimeout> {
        let Some(obj) = Self::obj_opt(cx, v, "timeout.tcp")? else {
            return Ok(TcpTimeout::default());
        };

        let connect_v = obj.get(cx, "connect")?;
        let read_v = obj.get(cx, "read")?;
        let write_v = obj.get(cx, "write")?;

        Ok(TcpTimeout {
            connect: Self::ms(cx, connect_v)?,
            read: Self::ms(cx, read_v)?,
            write: Self::ms(cx, write_v)?,
        })
    }

    fn udp<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> NeonResult<UdpTimeout> {
        let Some(obj) = Self::obj_opt(cx, v, "timeout.udp")? else {
            return Ok(UdpTimeout::default());
        };

        let read_v = obj.get(cx, "read")?;
        let write_v = obj.get(cx, "write")?;

        Ok(UdpTimeout {
            read: Self::ms(cx, read_v)?,
            write: Self::ms(cx, write_v)?,
        })
    }

    fn http<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> NeonResult<HttpTimeout> {
        let Some(obj) = Self::obj_opt(cx, v, "timeout.http")? else {
            return Ok(HttpTimeout::default());
        };

        let global_v = obj.get(cx, "global")?;
        Ok(HttpTimeout {
            global: Self::ms(cx, global_v)?,
        })
    }

    fn is_nullish<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> bool {
        v.is_a::<JsUndefined, _>(cx) || v.is_a::<JsNull, _>(cx)
    }

    fn obj_opt<'a>(
        cx: &mut FunctionContext<'a>,
        v: Handle<'a, JsValue>,
        name: &str,
    ) -> NeonResult<Option<Handle<'a, JsObject>>> {
        if Self::is_nullish(cx, v) {
            return Ok(None);
        }
        if v.is_a::<JsObject, _>(cx) {
            Ok(Some(v.downcast_or_throw::<JsObject, _>(cx)?))
        } else {
            cx.throw_type_error(format!("{name} must be an object, null, or undefined"))
        }
    }

    fn ms<'a>(
        cx: &mut FunctionContext<'a>,
        v: Handle<'a, JsValue>,
    ) -> NeonResult<Option<Duration>> {
        if Self::is_nullish(cx, v) {
            return Ok(None);
        }
        let n = v.downcast_or_throw::<JsNumber, _>(cx)?.value(cx);
        if !n.is_finite() || n < 0.0 {
            return cx.throw_range_error("timeout values must be non-negative milliseconds");
        }
        Ok(Some(Duration::from_millis(n as u64)))
    }
}

struct JsServer;

impl JsServer {
    fn from_rust<'a>(cx: &mut impl Context<'a>, s: &GenericServer) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let name = cx.string(&s.name);
        obj.set(cx, "name", name)?;

        let description = Self::opt_str(cx, s.description.as_deref());
        obj.set(cx, "description", description)?;

        let map = Self::opt_str(cx, s.map.as_deref());
        obj.set(cx, "map", map)?;

        let mode = Self::opt_str(cx, s.mode.as_deref());
        obj.set(cx, "mode", mode)?;

        let version = Self::opt_str(cx, s.version.as_deref());
        obj.set(cx, "version", version)?;

        let anti_cheat = Self::opt_str(cx, s.anti_cheat.as_deref());
        obj.set(cx, "antiCheat", anti_cheat)?;

        let has_password = Self::opt_bool(cx, s.has_password);
        obj.set(cx, "hasPassword", has_password)?;

        let max_players = cx.number(s.max_players as f64);
        obj.set(cx, "maxPlayers", max_players)?;

        let current_players = cx.number(s.current_players as f64);
        obj.set(cx, "currentPlayers", current_players)?;

        let players_val: Handle<JsValue> = match s.players.as_ref() {
            Some(players) => {
                let arr = Self::players(cx, players)?;
                arr.upcast()
            }
            None => cx.null().upcast(),
        };
        obj.set(cx, "players", players_val)?;

        let add_data_val: Handle<JsValue> = match s.additional_data.as_ref() {
            Some(m) => {
                let o = Self::data_map(cx, m)?;
                o.upcast()
            }
            None => cx.null().upcast(),
        };
        obj.set(cx, "additionalData", add_data_val)?;

        Ok(obj)
    }

    fn players<'a>(cx: &mut impl Context<'a>, players: &[GenericPlayer]) -> JsResult<'a, JsArray> {
        let arr = JsArray::new(cx, players.len());

        for (i, p) in players.iter().enumerate() {
            let p_obj = Self::player(cx, p)?;
            arr.set(cx, i as u32, p_obj)?;
        }

        Ok(arr)
    }

    fn player<'a>(cx: &mut impl Context<'a>, p: &GenericPlayer) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let name = cx.string(&p.name);
        obj.set(cx, "name", name)?;

        let add_data_val: Handle<JsValue> = match p.additional_data.as_ref() {
            Some(m) => {
                let o = Self::data_map(cx, m)?;
                o.upcast()
            }
            None => cx.null().upcast(),
        };
        obj.set(cx, "additionalData", add_data_val)?;

        Ok(obj)
    }

    fn data_map<'a>(cx: &mut impl Context<'a>, map: &GenericDataMap) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        for (k, v) in map.iter() {
            let js_v = Self::data_value(cx, v)?;
            obj.set(cx, k.as_str(), js_v)?;
        }

        Ok(obj)
    }

    fn data_value<'a>(cx: &mut impl Context<'a>, v: &GenericDataValue) -> JsResult<'a, JsValue> {
        match v {
            GenericDataValue::U8(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::U16(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::U32(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::U64(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::Usize(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::U128(x) => Ok(cx.string(x.to_string()).upcast()),

            GenericDataValue::I8(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::I16(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::I32(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::I64(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::Isize(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::I128(x) => Ok(cx.string(x.to_string()).upcast()),

            GenericDataValue::F32(x) => Ok(cx.number(*x as f64).upcast()),
            GenericDataValue::F64(x) => Ok(cx.number(*x).upcast()),

            GenericDataValue::Bool(b) => Ok(cx.boolean(*b).upcast()),

            GenericDataValue::String(s) => Ok(cx.string(s.as_str()).upcast()),
            GenericDataValue::StringList(list) => {
                let arr = JsArray::new(cx, list.len());
                for (i, s) in list.iter().enumerate() {
                    let js_s = cx.string(s);
                    arr.set(cx, i as u32, js_s)?;
                }
                Ok(arr.upcast())
            }

            GenericDataValue::Duration(d) => Ok(cx.number(d.as_millis() as f64).upcast()),
            GenericDataValue::IpAddr(ip) => Ok(cx.string(ip.to_string()).upcast()),
            GenericDataValue::SocketAddr(a) => Ok(cx.string(a.to_string()).upcast()),
        }
    }

    fn opt_str<'a>(cx: &mut impl Context<'a>, v: Option<&str>) -> Handle<'a, JsValue> {
        match v {
            Some(s) => cx.string(s).upcast(),
            None => cx.null().upcast(),
        }
    }

    fn opt_bool<'a>(cx: &mut impl Context<'a>, v: Option<bool>) -> Handle<'a, JsValue> {
        match v {
            Some(b) => cx.boolean(b).upcast(),
            None => cx.null().upcast(),
        }
    }
}

fn query(mut cx: FunctionContext) -> JsResult<JsPromise> { QueryArgs::parse(&mut cx)?.schedule(cx) }

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("query", query)?;
    Ok(())
}
