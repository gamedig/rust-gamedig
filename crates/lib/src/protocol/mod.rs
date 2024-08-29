/// Query protocol implementations.
///
/// This module contains the implementations for the various query protocols 
/// supported by `gamedig`. These protocols are used to retrieve information 
/// from game servers, such as the number of players, server name, map details, 
/// and other relevant data.
pub mod query;

/// RCON (Remote Console) protocol implementations.
///
/// This module provides the tools needed to interact with game servers via the 
/// RCON protocol. RCON allows for remote command execution on game servers, 
/// enabling server administrators to manage and control their servers 
/// programmatically. This module includes functionalities for connecting 
/// to servers, sending commands, and processing the responses.
pub mod rcon;
