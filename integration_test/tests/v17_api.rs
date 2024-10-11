//! Test the JSON-RPC API against `bitcoind v0.17.1`.

#![cfg(feature = "v17")]

use integration_test::*;

// == Blockchain ==
mod blockchain {
    use super::*;

    impl_test_v17__getbestblockhash!();
    impl_test_v17__getblock_verbosity_0!();
    impl_test_v17__getblock_verbosity_1!();
    impl_test_v17__getblockchaininfo!();
    impl_test_v17__getblockcount!();
    impl_test_v17__getblockhash!();
    impl_test_v17__getblockheader!();
    impl_test_v17__getblockstats!();
    impl_test_v17__getchaintips!();
    impl_test_v17__getchaintxstats!();
    impl_test_v17__getdifficulty!();
}

// == Control ==
mod control {
    use super::*;

    impl_test_v17__getmemoryinfo!();
    impl_test_v17__logging!();
    impl_test_v17__stop!();
    impl_test_v17__uptime!();
}

// == Generating ==
mod generating {
    use super::*;

    impl_test_v17__generatetoaddress!();
    impl_test_v17__generate!();
}

// == Network ==
mod network {
    use super::*;

    impl_test_v17__getaddednodeinfo!();
    impl_test_v17__getnettotals!();
    impl_test_v17__getnetworkinfo!();
    impl_test_v17__getpeerinfo!();
}

// == Rawtransactions ==
mod raw_transactions {
    use super::*;

    impl_test_v17__sendrawtransaction!();
}

// == Wallet ==
mod wallet {
    use super::*;

    impl_test_v17__createwallet!();
    impl_test_v17__loadwallet!();
    // impl_test_v17__unloadwallet!();

    impl_test_v17__getnewaddress!();
    impl_test_v17__getbalance!();
    impl_test_v17__sendtoaddress!();
    impl_test_v17__gettransaction!();
}

