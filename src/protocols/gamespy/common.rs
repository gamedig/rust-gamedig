use crate::{GDErrorKind, GDResult};
use std::collections::HashMap;

pub fn has_password(server_vars: &mut HashMap<String, String>) -> GDResult<bool> {
    let password_value = server_vars
        .remove("password")
        .ok_or(GDErrorKind::PacketBad.rich("Missing password (exists) field"))?
        .to_lowercase();

    if let Ok(has) = password_value.parse::<bool>() {
        return Ok(has);
    }

    let as_numeral: u8 = password_value
        .parse()
        .map_err(|e| GDErrorKind::TypeParse.rich(e))?;

    Ok(as_numeral != 0)
}
