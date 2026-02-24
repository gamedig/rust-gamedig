use {gamedig::converters::GenericPlayer, neon::prelude::*};

pub struct Player;

impl Player {
    pub fn array<'a>(
        cx: &mut impl Context<'a>,
        players: &[GenericPlayer],
    ) -> JsResult<'a, JsArray> {
        let arr = JsArray::new(cx, players.len());

        for (i, p) in players.iter().enumerate() {
            let obj = Self::to_js(cx, p)?;
            arr.set(cx, i as u32, obj)?;
        }

        Ok(arr)
    }

    fn to_js<'a>(cx: &mut impl Context<'a>, p: &GenericPlayer) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let id_v = cx.number(p.id as f64);
        obj.set(cx, "id", id_v)?;

        let name_v: Handle<JsValue> = match p.name.as_deref() {
            Some(s) => cx.string(s).upcast(),
            None => cx.null().upcast(),
        };
        obj.set(cx, "name", name_v)?;

        Ok(obj)
    }
}
