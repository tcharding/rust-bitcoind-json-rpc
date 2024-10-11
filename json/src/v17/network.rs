// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
/// These types do not implement `into_model` because apart from fee rate there is no additional
/// `rust-bitcoin` types needed.
use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getaddednodeinfo`.
///
/// > getaddednodeinfo ( "node" )
/// >
/// > Returns information about the given added node, or all added nodes
/// > (note that onetry addnodes are not listed here)
/// >
/// > Arguments:
/// > 1. "node"   (string, optional) If provided, return information about this specific node, otherwise all nodes are returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetAddedNodeInfo(Vec<AddedNode>);

/// An item from the list returned by the JSON-RPC method `getaddednodeinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedNode {
    /// The node IP address or name (as provided to addnode).
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// If connected.
    pub connected: bool,
    /// Only when connected = true.
    pub addresses: Vec<AddedNodeAddress>,
}

/// An address returned as part of the JSON-RPC method `getaddednodeinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedNodeAddress {
    /// The bitcoin server IP and port we're connected to.
    pub address: String,
    /// Connection, inbound or outbound.
    pub connected: String,
}

/// Result of JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns the number of connections to other nodes.
pub struct GetConnectionCount(u64);

/// Result of JSON-RPC method `getnettotals`.
///
/// > getnettotals
/// >
/// > Returns information about network traffic, including bytes in, bytes out,
/// > and current time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetTotals {
    /// Total bytes received.
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recieved: u64,
    /// Total bytes sent.
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    /// Current UNIX time in milliseconds.
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
    /// Upload target totals.
    #[serde(rename = "uploadtarget")]
    pub upload_target: UploadTarget,
}

/// The `upload_target` field from the result of JSON-RPC method `getnettotals`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UploadTarget {
    /// Length of the measuring timeframe in seconds.
    pub timeframe: u64,
    /// Target in bytes.
    pub target: u64,
    /// True if target is reached.
    pub target_reached: bool,
    /// True if serving historical blocks.
    pub serve_historical_blocks: bool,
    /// Bytes left in current time cycle.
    pub bytes_left_in_cycle: u64,
    /// Seconds left in current time cycle.
    pub time_left_in_cycle: u64,
}

/// Result of the JSON-RPC method `getnetworkinfo`.
///
/// > getnetworkinfo
///
/// > Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNetworkInfo {
    /// The server version.
    pub version: usize,
    /// The server subversion string.
    pub subversion: String,
    /// The protocol version.
    #[serde(rename = "protocolversion")]
    pub protocol_version: usize,
    /// The services we offer to the network (hex string).
    #[serde(rename = "localservices")]
    pub local_services: String,
    /// `true` if transaction relay is requested from peers.
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// The time offset.
    #[serde(rename = "timeoffset")]
    pub time_offset: isize,
    /// The total number of connections.
    pub connections: usize,
    #[serde(rename = "networkactive")]
    /// Whether p2p networking is enabled.
    pub network_active: bool,
    /// Information per network.
    pub networks: Vec<GetNetworkInfoNetwork>,
    /// Minimum relay fee rate for transactions in BTC/kB.
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kB.
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    /// List of local addresses.
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoAddress>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Part of the result of the JSON-RPC method `getnetworkinfo` (information per network).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

impl GetNetworkInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNetworkInfo, ParseAmountError> {
        // Relay fee is measured in BTC/kB.
        let relay_fee = self.relay_fee / 1000;        // BTC per byte
        let relay_fee = Amount::from_btc(relay_fee)?; // sats per byte
        let relay_fee = FeeRate::from_sat_per_vb(relay_fee); // Virtual bytes equal bytes before segwit.
        // Same for incremental_fee (BTC/kB).
        let incremental_fee = FeeRate::from_sat_per_vb(Amount::from_btc(self.incremental_fee / 1000)?);

        model::GetNetworkInfo {
            version: self.version,
            subversion: self.subversion,
            protocol_version: self.protocol_version,
            local_services: self.local_services,
            local_services_names: self.local_services_names,
            local_relay: self.local_relay,
            time_offset: self.time_offset,
            connections: sellf.connections,
            connections_in: self.connections_in,
            connections_out: self.connections_out,
            network_active: self.network_active,
            networks: self.networks.iter().map(|n| n.into_model()),
            relay_fee,
            incremental_fee,
            local_addresses: self.local_addresses.iter().map(|a| a.into_model()),
            warnings: self.warnings,
        }
    }
}

impl GetNetworkInfoNetwork {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetNetworkInfoNetwork {
        model::GetNetworkInfoNetwork {
            name: self.name,
            limited: self.limited,
            reachable: self.reachable,
            proxy: self.proxy,
            proxy_randomize_credentials: self.proxy_randomize_credentials,
        }
    }
}

impl GetNetworkInfoAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetNetworkInfoAddress {
        model::GetNetworkInfoAddress {
            address: self.address,
            port: self.port,
            score: self.score,
        }
    }
}

/// Result of JSON-RPC method `getpeerinfo`.
///
/// > getpeerinfo
/// >
/// > Returns data about each connected network node as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetPeerInfo(pub Vec<PeerInfo>);

/// An item from the list returned by the JSON-RPC method `getpeerinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PeerInfo {
    /// Peer index.
    pub id: u32,
    /// The IP address and port of the peer ("host:port").
    #[serde(rename = "addr")]
    pub address: String,
    /// Bind address of the connection to the peer ("ip:port").
    #[serde(rename = "addrbind")]
    pub address_bind: String,
    /// Local address as reported by the peer.
    #[serde(rename = "addrlocal")]
    pub address_local: String,
    /// The services offered.
    pub services: String,
    /// Whether peer has asked us to relay transactions to it.
    #[serde(rename = "relaytxes")]
    pub relay_transactions: bool,
    /// The time in seconds since epoch (Jan 1 1970 GMT) of the last send.
    #[serde(rename = "lastsend")]
    pub last_send: u32,
    /// The time in seconds since epoch (Jan 1 1970 GMT) of the last receive.
    #[serde(rename = "lastrecv")]
    pub last_received: u32,
    /// The total bytes sent.
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    /// The total bytes received.
    #[serde(rename = "bytesrecv")]
    pub bytes_received: u64,
    /// The connection time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "conntime")]
    pub connection_time: u32,
    /// The time offset in seconds.
    #[serde(rename = "timeoffset")]
    pub time_offset: u32,
    /// Ping time (if available).
    #[serde(rename = "pingtime")]
    pub ping_time: u32,
    /// Minimum observed ping time (if any at all).
    #[serde(rename = "minping")]
    pub minimum_ping: u32,
    /// Ping wait (if non-zero).
    #[serde(rename = "pingwait")]
    pub ping_wait: u32,
    /// The peer version, such as 70001.
    pub version: u32,
    /// The string version (e.g. "/Satoshi:0.8.5/").
    #[serde(rename = "subver")]
    pub subversion: String,
    /// Inbound (true) or Outbound (false).
    pub inbound: bool,
    /// Whether connection was due to addnode/-connect or if it was an automatic/inbound connection.
    #[serde(rename = "addnode")]
    pub add_node: bool,
    /// The starting height (block) of the peer.
    #[serde(rename = "startingheight")]
    pub starting_height: u32,
    /// The ban score.
    #[serde(rename = "banscore")]
    pub ban_score: u64,
    /// The last header we have in common with this peer.
    pub synced_headers: u32,
    /// The last block we have in common with this peer.
    pub synced_blocks: u32,
    /// The heights of blocks we're currently asking from this peer.
    pub inflight: Vec<u32>,
    /// Whether the peer is whitelisted.
    pub whitelisted: bool,
    /// The total bytes sent aggregated by message type.
    #[serde(rename = "bytessent_per_msg")]
    pub bytes_sent_per_message: Vec<BytesPerMessage>,
    /// The total bytes received aggregated by message type.
    #[serde(rename = "bytesrecv_per_msg")]
    pub bytes_received_per_message: Vec<BytesPerMessage>,
}

/// An item from the list returned by the JSON-RPC method `getpeerinfo`.
 // FIXME: The docs show invalid JSON so I don't know exactly what this should be.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BytesPerMessage {
    #[serde(rename = "addr")]
    pub address: u32,
}

/// Result of JSON-RPC method `listbanned`.
///
/// > listbanned
///
/// > List all banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListBanned(pub Vec<Banned>);

/// An item from the list returned by the JSON-RPC method `listbanned`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Banned(String);      // FIXME: The docs are empty so I don't know what shape this is.
