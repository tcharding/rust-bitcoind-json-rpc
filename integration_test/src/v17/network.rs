// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_network_info`.
#[macro_export]
macro_rules! impl_test_v17__getaddednodeinfo {
    () => {
        #[test]
        fn get_added_node_info() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let _ = bitcoind.client.get_added_node_info().expect("getaddednodeinfo");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_network_info`.
#[macro_export]
macro_rules! impl_test_v17__getnettotals {
    () => {
        #[test]
        fn get_net_totals() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let _ = bitcoind.client.get_net_totals().expect("getnettotals");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_network_info` and
/// `check_expected_server_version`.
#[macro_export]
macro_rules! impl_test_v17__getnetworkinfo {
    () => {
        #[test]
        fn get_network_info() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let json = bitcoind.client.get_network_info().expect("getnetworkinfo");
            assert!(json.into_model().is_ok());

            // Server version is returned as part of the getnetworkinfo method.
            bitcoind.client.check_expected_server_version().expect("unexpected version");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_peer_info`.
#[macro_export]
macro_rules! impl_test_v17__getpeerinfo {
    () => {
        #[test]
        fn get_peer_info() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let _ = bitcoind.client.get_peer_info().expect("getpeerinfo");
        }
    };
}
