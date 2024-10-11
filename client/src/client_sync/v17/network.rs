// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `getaddednodeinfo`
#[macro_export]
macro_rules! impl_client_v17__getaddednodeinfo {
    () => {
        impl Client {
            pub fn get_added_node_info(&self) -> Result<GetAddedNodeInfo> { self.call("getaddednodeinfo", &[]) }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getnettotals`
#[macro_export]
macro_rules! impl_client_v17__getnettotals {
    () => {
        impl Client {
            pub fn get_net_totals(&self) -> Result<GetNetTotals> { self.call("getnettotals", &[]) }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getnetworkinfo`
#[macro_export]
macro_rules! impl_client_v17__getnetworkinfo {
    () => {
        impl Client {
            /// Returns the server version field of `GetNetworkInfo`.
            pub fn server_version(&self) -> Result<usize> {
                let info = self.get_network_info()?;
                Ok(info.version)
            }

            pub fn get_network_info(&self) -> Result<GetNetworkInfo> {
                self.call("getnetworkinfo", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getpeerinfo`
#[macro_export]
macro_rules! impl_client_v17__getpeerinfo {
    () => {
        impl Client {
            pub fn get_peer_info(&self) -> Result<GetPeerInfo> { self.call("getpeerinfo", &[]) }
        }
    };
}
