// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.17.1`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod control;
pub mod generating;
pub mod network;
pub mod raw_transactions;
pub mod wallet;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};
use serde::{Deserialize, Serialize};

use crate::client_sync::{handle_defaults, into_json};
use crate::json::v17::*;

crate::define_jsonrpc_minreq_client!("v17");
crate::impl_client_check_expected_server_version!({ [170100] });

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();
crate::impl_client_v17__getblockcount!();
crate::impl_client_v17__getblockhash!();
crate::impl_client_v17__getblockheader!();
crate::impl_client_v17__getblockstats!();
crate::impl_client_v17__getchaintips!();
crate::impl_client_v17__getchaintxstats!();
crate::impl_client_v17__getdifficulty!();
crate::impl_client_v17__getmempoolancestors!();

// == Control ==
crate::impl_client_v17__getmemoryinfo!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();
crate::impl_client_v17__generate!();

// == Network ==
crate::impl_client_v17__getaddednodeinfo!();
crate::impl_client_v17__getnettotals!();
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_v17__getpeerinfo!();

// == Rawtransactions ==
crate::impl_client_v17__sendrawtransaction!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__unloadwallet!();
crate::impl_client_v17__loadwallet!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__gettransaction!();

/// Argument to the `Client::get_new_address_with_type` function.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddressType {
    Legacy,
    P2shSegwit,
    Bech32,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddressType::*;

        let s = match *self {
            Legacy => "legacy",
            P2shSegwit => "p2sh-segwit",
            Bech32 => "bech32",
        };
        fmt::Display::fmt(s, f)
    }
}
