// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Network ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::FeeRate;
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `getnetworkinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    pub local_services: String,
    /// The services we offer to the network, in human-readable form.
    pub local_services_names: Vec<String>,
    /// `true` if transaction relay is requested from peers.
    pub local_relay: bool,
    /// The time offset.
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    /// The number of inbound connections.
    pub connections_in: usize,
    /// The number of outbound connections.
    pub connections_out: usize,
    /// Whether p2p networking is enabled.
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions.
    pub relay_fee: FeeRate,
    /// Minimum fee rate increment for mempool limiting or replacement.
    pub incremental_fee: FeeRate,
    /// List of local addresses.
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (information per network).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfoNetwork {
    /// Network (ipv4, ipv6, onion, i2p, cjdns).
    pub name: String,
    /// Is the network limited using -onlynet?.
    pub limited: bool,
    /// Is the network reachable?
    pub reachable: bool,
    /// ("host:port"): The proxy that is used for this network, or empty if none.
    pub proxy: String,
    /// Whether randomized credentials are used.
    pub proxy_randomize_credentials: bool,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (local address info).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetNetworkInfoAddress {
    /// Network address.
    pub address: String,
    /// Network port.
    pub port: u16,
    /// Relative score.
    pub score: u32,
}
