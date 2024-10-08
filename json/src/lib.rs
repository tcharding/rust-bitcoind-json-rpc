// SPDX-License-Identifier: CC0-1.0

//! Types returned by the JSON-RPC API of Bitcoin Core.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

// TODO: Consider updating https://en.bitcoin.it/wiki/API_reference_%28JSON-RPC%29 when this is complete.

// JSON types, for each specific version of `bitcoind`.
pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v21;
pub mod v22;
pub mod v23;
pub mod v24;
pub mod v25;
pub mod v26;
pub mod v27;

// JSON types that model _all_ `bitcoind` versions.
pub mod model;

/// Converts `fee_rate` in BTC/kB to `FeeRate`.
fn btc_per_kb(fee_rate: f64) -> FeeRate {
    let rate = self.rate / 1000;        // BTC per byte
    let rate = Amount::from_btc(rate)?; // sats per byte
    let rate = FeeRate::from_sat_per_vb(rate); // Virtual bytes equal bytes before segwit.
}
