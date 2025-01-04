/// The implementation.
/// Reference: [Server List Ping](https://wiki.vg/Server_List_Ping)
pub mod protocol;
/// All types used by the implementation.
pub mod types;

#[allow(unused_imports)]
pub use protocol::*;
pub use types::*;

use crate::{GDErrorKind, GDResult};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::net::{IpAddr, SocketAddr};

#[pymodule]
pub fn minecraft(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_query, m)?)?;
    Ok(())
}

/// Query with all the protocol variants one by one (Java -> Bedrock -> Legacy
/// (1.6 -> 1.4 -> Beta 1.8)).
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    if let Ok(response) = query_java(address, port, None) {
        return Ok(response);
    }

    if let Ok(response) = query_bedrock(address, port) {
        return Ok(JavaResponse::from_bedrock_response(response));
    }

    if let Ok(response) = query_legacy(address, port) {
        return Ok(response);
    }

    Err(GDErrorKind::AutoQuery.into())
}

#[pyfunction]
pub fn py_query(address: &str, port: u16) -> PyResult<Py<PyDict>> {
    let response = query(&address.parse().unwrap(), Some(port));
    // None is the default port (which is 27015), could also be Some(27015)

    match response { // Result type, must check what it is...
        Err(error) => {
            println!("Couldn't query, error: {}", error);
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Couldn't query, error: {}", error)))
        },
        Ok(r) => {
            Python::with_gil(|py| {
                let dict = PyDict::new(py);

                dict.set_item("game_version", r.game_version)?;
                dict.set_item("protocol_version", r.protocol_version)?;
                dict.set_item("players_maximum", r.players_maximum)?;
                dict.set_item("players_online", r.players_online)?;
                dict.set_item("description", r.description)?;
                dict.set_item("favicon", r.favicon)?;
                dict.set_item("previews_chat", r.previews_chat)?;
                dict.set_item("enforces_secure_chat", r.enforces_secure_chat)?;
                dict.set_item("server_type", format!("{:?}", r.server_type))?;

                if let Some(players) = r.players {
                    let players_list = PyList::new(py, players.iter().map(|p| {
                        let player_dict = PyDict::new(py);
                        player_dict.set_item("name", &p.name).unwrap();
                        player_dict.set_item("id", &p.id).unwrap();
                        player_dict
                    }).collect::<Vec<_>>());
                    dict.set_item("players", players_list)?;
                } else {
                    dict.set_item("players", py.None())?;
                }

                Ok(dict.into_py(py))
            })
        }
    }
}


/// Query a Java Server.
pub fn query_java(
    address: &IpAddr,
    port: Option<u16>,
    request_settings: Option<RequestSettings>,
) -> GDResult<JavaResponse> {
    protocol::query_java(
        &SocketAddr::new(*address, port_or_java_default(port)),
        None,
        request_settings,
    )
}

/// Query a (Java) Legacy Server (1.6 -> 1.4 -> Beta 1.8).
pub fn query_legacy(address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    protocol::query_legacy(&SocketAddr::new(*address, port_or_java_default(port)), None)
}

/// Query a specific (Java) Legacy Server.
pub fn query_legacy_specific(group: LegacyGroup, address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    protocol::query_legacy_specific(
        group,
        &SocketAddr::new(*address, port_or_java_default(port)),
        None,
    )
}

/// Query a Bedrock Server.
pub fn query_bedrock(address: &IpAddr, port: Option<u16>) -> GDResult<BedrockResponse> {
    protocol::query_bedrock(
        &SocketAddr::new(*address, port_or_bedrock_default(port)),
        None,
    )
}

fn port_or_java_default(port: Option<u16>) -> u16 { port.unwrap_or(25565) }

fn port_or_bedrock_default(port: Option<u16>) -> u16 { port.unwrap_or(19132) }
