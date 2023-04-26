#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Filter {
    IsSecured(bool),
    Map(String),
    CanHavePassword(bool),
    CanBeEmpty(bool),
    CanBeFull(bool),
    AppId(u32),
}

fn bool_as_char_u8(b: bool) -> u8 {
    match b {
        true => b'1',
        false => b'0',
    }
}

impl Filter {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8>;

        match self {
            Filter::IsSecured(secured) => {
                bytes = "\\secure\\".as_bytes().to_vec();
                bytes.extend([bool_as_char_u8(*secured)]);
            }
            Filter::Map(map) => {
                bytes = "\\map\\".as_bytes().to_vec();
                bytes.extend(map.as_bytes());
            }
            Filter::CanHavePassword(password) => {
                bytes = "\\password\\".as_bytes().to_vec();
                bytes.extend([bool_as_char_u8(*password)]);
            }
            Filter::CanBeEmpty(empty) => {
                bytes = "\\empty\\".as_bytes().to_vec();
                bytes.extend([bool_as_char_u8(*empty)]);
            }
            Filter::CanBeFull(full) => {
                bytes = "\\full\\".as_bytes().to_vec();
                bytes.extend([bool_as_char_u8(*full)]);
            }
            Filter::AppId(id) => {
                bytes = "\\appid\\".as_bytes().to_vec();
                bytes.extend(id.to_string().as_bytes());
            }
        }

        bytes.extend([0x00]);
        bytes
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SearchFilters {
    filters: Vec<Filter>,
}

impl SearchFilters {
    pub(crate) fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub(crate) fn add(self, filter: Filter) -> Self {
        let mut last_filters = self.filters;

        let found_same_filter = last_filters.iter_mut().find_map(|f| {
            if std::mem::discriminant(f) == std::mem::discriminant(&filter) {
                Some(f)
            } else {
                None
            }
        });

        match found_same_filter {
            None => last_filters.push(filter),
            Some(f) => *f = filter,
        }

        Self {
            filters: last_filters,
        }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for filter in &self.filters {
            bytes.extend(filter.to_bytes())
        }

        bytes
    }
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
