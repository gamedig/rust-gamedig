use {crate::player::Player, gamedig::converters::GenericServer, neon::prelude::*};

pub struct Server;

impl Server {
    pub fn to_js<'a>(cx: &mut impl Context<'a>, s: &GenericServer) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let name_v = cx.string(&s.name);
        obj.set(cx, "name", name_v)?;

        Self::set_opt_str(cx, &obj, "description", s.description.as_deref())?;
        Self::set_opt_str(cx, &obj, "map", s.map.as_deref())?;
        Self::set_opt_str(cx, &obj, "mode", s.mode.as_deref())?;
        Self::set_opt_str(cx, &obj, "version", s.version.as_deref())?;

        Self::set_opt_bool(cx, &obj, "antiCheat", s.anti_cheat)?;
        Self::set_opt_bool(cx, &obj, "hasPassword", s.has_password)?;

        let max_v = cx.number(s.max_players as f64);
        obj.set(cx, "maxPlayers", max_v)?;

        let cur_v = cx.number(s.current_players as f64);
        obj.set(cx, "currentPlayers", cur_v)?;

        match s.players.as_deref() {
            Some(players) => {
                let arr = Player::array(cx, players)?;
                obj.set(cx, "players", arr)?;
            }
            None => {
                let null_v = cx.null();
                obj.set(cx, "players", null_v)?;
            }
        }

        Ok(obj)
    }

    fn set_opt_str<'a>(
        cx: &mut impl Context<'a>,
        obj: &Handle<'a, JsObject>,
        key: &str,
        val: Option<&str>,
    ) -> NeonResult<()> {
        let v: Handle<JsValue> = match val {
            Some(s) => cx.string(s).upcast(),
            None => cx.null().upcast(),
        };
        obj.set(cx, key, v)?;
        Ok(())
    }

    fn set_opt_bool<'a>(
        cx: &mut impl Context<'a>,
        obj: &Handle<'a, JsObject>,
        key: &str,
        val: Option<bool>,
    ) -> NeonResult<()> {
        let v: Handle<JsValue> = match val {
            Some(b) => cx.boolean(b).upcast(),
            None => cx.null().upcast(),
        };
        obj.set(cx, key, v)?;
        Ok(())
    }
}
