use std::net::SocketAddr;

pub struct Dict;

impl Dict {
    pub fn get(&self, id: &str, addr: SocketAddr, port: u16) -> () {
        hashify::fnc_map_ignore_case!(id.as_bytes(),
            #[cfg(feature = "gamespy_1")]
            "gamespy_1" => {
                todo!();
            },

            #[cfg(feature = "gamespy_2")]
            "gamespy_2" => {
                todo!();
            },

            #[cfg(feature = "gamespy_3")]
            "gamespy_3" => {
                todo!();
            },

            #[cfg(feature = "mumble")]
            "mumble" => {
                todo!();
            },

            #[cfg(feature = "quake_1")]
            "quake_1" => {
                todo!();
            },

            #[cfg(feature = "quake_2")]
            "quake_2" => {
                todo!();
            },

            #[cfg(feature = "quake_3")]
            "quake_3" => {
                todo!();
            },

            #[cfg(feature = "teamspeak_2")]
            "teamspeak_2" => {
                todo!();
            },

            #[cfg(feature = "teamspeak_3")]
            "teamspeak_3" => {
                todo!();
            },

            #[cfg(feature = "unreal_2")]
            "unreal_2" => {
                todo!();
            },

            #[cfg(feature = "valve")]
            "valve" => {
                todo!();
            },

            _ => {
                todo!();

                // some handling for unknown IDs like but in better way
                // eprintln!("Unknown ID: {id}");

            }
        );
    }
}
