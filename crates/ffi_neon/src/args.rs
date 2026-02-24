use {
    gamedig::converters::{HttpTimeout, TcpTimeout, TimeoutConfig, UdpTimeout},
    neon::prelude::*,
    std::time::Duration,
};

#[derive(Clone)]
pub struct Query {
    pub game_id: String,
    pub addr: String,
    pub timeout: Option<TimeoutConfig>,
}

impl Query {
    pub fn parse(cx: &mut FunctionContext) -> NeonResult<Self> {
        let game_id = cx.argument::<JsString>(0)?.value(cx);
        let addr = cx.argument::<JsString>(1)?.value(cx);
        let timeout = Timeout::parse(cx, 2)?.map(Timeout::into_config);

        Ok(Self {
            game_id,
            addr,
            timeout,
        })
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

        let root: Handle<JsObject> = if v.is_a::<JsNull, _>(cx) || v.is_a::<JsUndefined, _>(cx) {
            return Ok(None);
        } else if v.is_a::<JsObject, _>(cx) {
            v.downcast_or_throw::<JsObject, _>(cx)?
        } else {
            return cx.throw_type_error("timeout must be an object, null, or undefined");
        };

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
        let obj: Handle<JsObject> = if v.is_a::<JsNull, _>(cx) || v.is_a::<JsUndefined, _>(cx) {
            return Ok(TcpTimeout::default());
        } else if v.is_a::<JsObject, _>(cx) {
            v.downcast_or_throw::<JsObject, _>(cx)?
        } else {
            return cx.throw_type_error("timeout.tcp must be an object, null, or undefined");
        };

        let connect_v = obj.get(cx, "connect")?;
        let read_v = obj.get(cx, "read")?;
        let write_v = obj.get(cx, "write")?;

        Ok(TcpTimeout {
            connect: Self::ms(cx, connect_v, "timeout.tcp.connect")?,
            read: Self::ms(cx, read_v, "timeout.tcp.read")?,
            write: Self::ms(cx, write_v, "timeout.tcp.write")?,
        })
    }

    fn udp<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> NeonResult<UdpTimeout> {
        let obj: Handle<JsObject> = if v.is_a::<JsNull, _>(cx) || v.is_a::<JsUndefined, _>(cx) {
            return Ok(UdpTimeout::default());
        } else if v.is_a::<JsObject, _>(cx) {
            v.downcast_or_throw::<JsObject, _>(cx)?
        } else {
            return cx.throw_type_error("timeout.udp must be an object, null, or undefined");
        };

        let read_v = obj.get(cx, "read")?;
        let write_v = obj.get(cx, "write")?;

        Ok(UdpTimeout {
            read: Self::ms(cx, read_v, "timeout.udp.read")?,
            write: Self::ms(cx, write_v, "timeout.udp.write")?,
        })
    }

    fn http<'a>(cx: &mut FunctionContext<'a>, v: Handle<'a, JsValue>) -> NeonResult<HttpTimeout> {
        let obj: Handle<JsObject> = if v.is_a::<JsNull, _>(cx) || v.is_a::<JsUndefined, _>(cx) {
            return Ok(HttpTimeout::default());
        } else if v.is_a::<JsObject, _>(cx) {
            v.downcast_or_throw::<JsObject, _>(cx)?
        } else {
            return cx.throw_type_error("timeout.http must be an object, null, or undefined");
        };

        let global_v = obj.get(cx, "global")?;

        Ok(HttpTimeout {
            global: Self::ms(cx, global_v, "timeout.http.global")?,
        })
    }

    fn ms<'a>(
        cx: &mut FunctionContext<'a>,
        v: Handle<'a, JsValue>,
        name: &str,
    ) -> NeonResult<Option<Duration>> {
        if v.is_a::<JsNull, _>(cx) || v.is_a::<JsUndefined, _>(cx) {
            return Ok(None);
        }

        let n = v.downcast_or_throw::<JsNumber, _>(cx)?.value(cx);
        if !n.is_finite() || n < 0.0 {
            return cx.throw_range_error(format!("{name} must be non negative milliseconds"));
        }

        Ok(Some(Duration::from_millis(n as u64)))
    }
}
