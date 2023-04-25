#[derive(Eq, PartialEq, Clone)]
pub struct Filters {
    pub secure: bool,
    pub game_dir: Option<String>,
    pub map: Option<String>,
    pub has_password: bool,
    pub not_empty: bool,
    pub not_full: bool,
}

impl Filters {
    pub(crate) fn to_bytes<'a>(self) -> &'a [u8] { &[0x00] }
}

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Region {
    UsEast = 0x00,
    UsWest = 0x01,
    AmericaSouth = 0x02,
    Europe = 0x03,
    Asia = 0x04,
    Australia = 0x05,
    MiddleEast = 0x06,
    Africa = 0x07,
    Others = 0xFF,
}
