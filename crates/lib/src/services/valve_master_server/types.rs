use std::collections::HashMap;
use std::mem::Discriminant;

/// A query filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Filter {
    IsSecured(bool),
    RunsMap(String),
    CanHavePassword(bool),
    CanBeEmpty(bool),
    IsEmpty(bool),
    CanBeFull(bool),
    RunsAppID(u32),
    NotAppID(u32),
    HasTags(Vec<String>),
    MatchName(String),
    MatchVersion(String),
    /// Restrict to only a server if an IP hosts (on different ports) multiple
    /// servers.
    RestrictUniqueIP(bool),
    /// Query for servers on a specific address.
    OnAddress(String),
    Whitelisted(bool),
    SpectatorProxy(bool),
    IsDedicated(bool),
    RunsLinux(bool),
    HasGameDir(String),
}

const fn bool_as_char_u8(b: &bool) -> u8 {
    match b {
        true => b'1',
        false => b'0',
    }
}

impl Filter {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Self::IsSecured(secured) => {
                bytes = b"\\secure\\".to_vec();
                bytes.extend([bool_as_char_u8(secured)]);
            }
            Self::RunsMap(map) => {
                bytes = b"\\map\\".to_vec();
                bytes.extend(map.as_bytes());
            }
            Self::CanHavePassword(password) => {
                bytes = b"\\password\\".to_vec();
                bytes.extend([bool_as_char_u8(password)]);
            }
            Self::CanBeEmpty(empty) => {
                bytes = b"\\empty\\".to_vec();
                bytes.extend([bool_as_char_u8(empty)]);
            }
            Self::CanBeFull(full) => {
                bytes = b"\\full\\".to_vec();
                bytes.extend([bool_as_char_u8(full)]);
            }
            Self::RunsAppID(id) => {
                bytes = b"\\appid\\".to_vec();
                bytes.extend(id.to_string().as_bytes());
            }
            Self::HasTags(tags) => {
                if !tags.is_empty() {
                    bytes = b"\\gametype\\".to_vec();
                    for tag in tags.iter() {
                        bytes.extend(tag.as_bytes());
                        bytes.extend([b',']);
                    }

                    bytes.pop();
                }
            }
            Self::NotAppID(id) => {
                bytes = b"\\napp\\".to_vec();
                bytes.extend(id.to_string().as_bytes());
            }
            Self::IsEmpty(empty) => {
                bytes = b"\\noplayers\\".to_vec();
                bytes.extend([bool_as_char_u8(empty)]);
            }
            Self::MatchName(name) => {
                bytes = b"\\name_match\\".to_vec();
                bytes.extend(name.as_bytes());
            }
            Self::MatchVersion(version) => {
                bytes = b"\\version_match\\".to_vec();
                bytes.extend(version.as_bytes());
            }
            Self::RestrictUniqueIP(unique) => {
                bytes = b"\\collapse_addr_hash\\".to_vec();
                bytes.extend([bool_as_char_u8(unique)]);
            }
            Self::OnAddress(address) => {
                bytes = b"\\gameaddr\\".to_vec();
                bytes.extend(address.as_bytes());
            }
            Self::Whitelisted(whitelisted) => {
                bytes = b"\\white\\".to_vec();
                bytes.extend([bool_as_char_u8(whitelisted)]);
            }
            Self::SpectatorProxy(condition) => {
                bytes = b"\\proxy\\".to_vec();
                bytes.extend([bool_as_char_u8(condition)]);
            }
            Self::IsDedicated(dedicated) => {
                bytes = b"\\dedicated\\".to_vec();
                bytes.extend([bool_as_char_u8(dedicated)]);
            }
            Self::RunsLinux(linux) => {
                bytes = b"\\linux\\".to_vec();
                bytes.extend([bool_as_char_u8(linux)]);
            }
            Self::HasGameDir(game_dir) => {
                bytes = b"\\gamedir\\".to_vec();
                bytes.extend(game_dir.as_bytes());
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
///             .insert(Filter::RunsAppID(440))
///             .insert(Filter::IsEmpty(false))
///             .insert(Filter::CanHavePassword(false));
/// ```
/// This will construct filters that search for servers that can't have a
/// password, are not empty and run App ID 440.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchFilters {
    filters: HashMap<Discriminant<Filter>, Filter>,
    nor_filters: HashMap<Discriminant<Filter>, Filter>,
    nand_filters: HashMap<Discriminant<Filter>, Filter>,
}

impl Default for SearchFilters {
    fn default() -> Self { Self::new() }
}

impl SearchFilters {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            nor_filters: HashMap::new(),
            nand_filters: HashMap::new(),
        }
    }

    pub fn insert(self, filter: Filter) -> Self {
        let mut updated_fitler = self.filters;
        updated_fitler.insert(std::mem::discriminant(&filter), filter);

        Self {
            filters: updated_fitler,
            nand_filters: self.nand_filters,
            nor_filters: self.nor_filters,
        }
    }

    pub fn insert_nand(self, filter: Filter) -> Self {
        let mut updated_fitler = self.nor_filters;
        updated_fitler.insert(std::mem::discriminant(&filter), filter);

        Self {
            filters: self.filters,
            nand_filters: self.nand_filters,
            nor_filters: updated_fitler,
        }
    }

    pub fn insert_nor(self, filter: Filter) -> Self {
        let mut updated_fitler = self.nand_filters;
        updated_fitler.insert(std::mem::discriminant(&filter), filter);

        Self {
            filters: self.filters,
            nand_filters: updated_fitler,
            nor_filters: self.nor_filters,
        }
    }

    fn special_filter_to_bytes(name: &str, filters: &HashMap<Discriminant<Filter>, Filter>) -> Vec<u8> {
        let mut bytes = Vec::new();

        if !filters.is_empty() {
            bytes.extend(name.as_bytes());
            bytes.extend(filters.len().to_string().as_bytes());
            for filter in filters.values() {
                bytes.extend(filter.to_bytes());
            }
        }

        bytes
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for filter in self.filters.values() {
            bytes.extend(filter.to_bytes())
        }

        bytes.extend(Self::special_filter_to_bytes("nand", &self.nand_filters));
        bytes.extend(Self::special_filter_to_bytes("nor", &self.nor_filters));

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
