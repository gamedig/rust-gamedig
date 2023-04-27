/// A query filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Filter {
    IsSecured(bool),
    Map(String),
    CanHavePassword(bool),
    CanBeEmpty(bool),
    CanBeFull(bool),
    AppId(u32),
    HasTags(Vec<String>),
}

fn bool_as_char_u8(b: bool) -> u8 {
    match b {
        true => b'1',
        false => b'0',
    }
}

impl Filter {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

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
            Filter::HasTags(tags) => {
                if !tags.is_empty() {
                    bytes = "\\gametype\\".as_bytes().to_vec();
                    for tag in tags {
                        bytes.extend(tag.as_bytes());
                        bytes.extend([b',']);
                    }

                    bytes.pop();
                }
            }
        }

        bytes
    }
}

/// Query search filters.
/// An example of constructing one:
/// ```rust
/// use gamedig::valve_master_server::{Filter, SearchFilters};
///
/// let search_filters = SearchFilters::new()
///             .insert(Filter::AppId(440))
///             .insert(Filter::CanHavePassword(true));
/// ```
/// This would query the servers that are (by App ID) 440 and that can contain
/// passwords.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SearchFilters {
    filters: Vec<Filter>,
}

impl Default for SearchFilters {
    fn default() -> Self { SearchFilters::new() }
}

impl SearchFilters {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn insert(self, filter: Filter) -> Self {
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

        bytes.extend([0x00]);
        bytes
    }
}

/// The region that you want to query server for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
