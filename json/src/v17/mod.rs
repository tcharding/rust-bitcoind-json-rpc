// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC types for `bitcoind v0.17.1`.
//!
//! These structs model the JSON data returned by the JSON-RPC API. They use stdlib types (or custom
//! types) and where necessary implement an `into_model` function to convert the type to a
//! [`crate::model`] type of the same name. The types in this module are version specific, the types
//! in the `model` module are version non-specific and are strongly typed using `rust-bitcoin`.
//!
//! Key:
//! - `[ ]` means not yet done.
//! - `[x]` marks means implemented _and_ tested.
//! - `[-]` means intentionally not done, typically because method does not return anything.
//!
//! **== Blockchain ==**
//! - [x] `getbestblockhash`
//! - [x] `getblock "blockhash" ( verbosity ) `
//! - [x] `getblockchaininfo`
//! - [x] `getblockcount`
//! - [x] `getblockhash height`
//! - [x] `getblockheader "hash" ( verbose )`
//! - [x] `getblockstats hash_or_height ( stats )`
//! - [x] `getchaintips`
//! - [x] `getchaintxstats ( nblocks blockhash )`
//! - [x] `getdifficulty`
//! - [x] `getmempoolancestors txid (verbose)`
//! - [x] `getmempooldescendants txid (verbose)`
//! - [x] `getmempoolentry txid`
//! - [x] `getmempoolinfo`
//! - [x] `getrawmempool ( verbose )`
//! - [x] `gettxout "txid" n ( include_mempool )`
//! - [x] `gettxoutproof ["txid",...] ( blockhash )`
//! - [x] `gettxoutsetinfo`
//! - [x] `preciousblock "blockhash"`
//! - [-] `pruneblockchain`
//! - [-] `savemempool`
//! - [-] `scantxoutset <action> ( <scanobjects> )`
//! - [x] `verifychain ( checklevel nblocks )`
//! - [-] `verifytxoutproof "proof"`
//!
//! **== Control ==**
//! - [x] `getmemoryinfo ("mode")`
//! - [-] `help ( "command" )`
//! - [x] `logging ( <include> <exclude> )`
//! - [x] `stop`
//! - [x] `uptime`
//!
//! **== Generating ==**
//! - [x] `generate nblocks ( maxtries )`
//! - [x] `generatetoaddress nblocks address (maxtries)`
//!
//! **== Mining ==**
//! - [ ] `getblocktemplate ( TemplateRequest )`
//! - [ ] `getmininginfo`
//! - [ ] `getnetworkhashps ( nblocks height )`
//! - [ ] `prioritisetransaction <txid> <dummy value> <fee delta>`
//! - [ ] `submitblock "hexdata"  ( "dummy" )`
//!
//! **== Network ==**
//! - [-] `addnode "node" "add|remove|onetry"`
//! - [-] `clearbanned`
//! - [-] `disconnectnode "[address]" [nodeid]`
//! - [x] `getaddednodeinfo ( "node" )`
//! - [-] `getconnectioncount`
//! - [x] `getnettotals`
//! - [x] `getnetworkinfo`
//! - [x] `getpeerinfo`
//! - [-] `listbanned`
//! - [-] `ping`
//! - [-] `setban "subnet" "add|remove" (bantime) (absolute)`
//! - [-] `setnetworkactive true|false`
//!
//! **== Rawtransactions ==**
//! - [ ] `combinepsbt ["psbt",...]`
//! - [ ] `combinerawtransaction ["hexstring",...]`
//! - [ ] `converttopsbt "hexstring" ( permitsigdata iswitness )`
//! - [ ] `createpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
//! - [ ] `createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
//! - [ ] `decodepsbt "psbt"`
//! - [ ] `decoderawtransaction "hexstring" ( iswitness )`
//! - [ ] `decodescript "hexstring"`
//! - [ ] `finalizepsbt "psbt" ( extract )`
//! - [ ] `fundrawtransaction "hexstring" ( options iswitness )`
//! - [ ] `getrawtransaction "txid" ( verbose "blockhash" )`
//! - [ ] `sendrawtransaction "hexstring" ( allowhighfees )`
//! - [ ] `signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )`
//! - [ ] `signrawtransactionwithkey "hexstring" ["privatekey1",...] ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
//! - [ ] `testmempoolaccept ["rawtxs"] ( allowhighfees )`
//!
//! **== Util ==**
//! - [ ] `createmultisig nrequired ["key",...] ( "address_type" )`
//! - [ ] `estimatesmartfee conf_target ("estimate_mode")`
//! - [ ] `signmessagewithprivkey "privkey" "message"`
//! - [ ] `validateaddress "address"`
//! - [ ] `verifymessage "address" "signature" "message"`
//!
//! **== Wallet ==**
//! - [-] `abandontransaction "txid"`
//! - [-] `abortrescan`
//! - [x] `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
//! - [-] `backupwallet "destination"`
//! - [x] `bumpfee "txid" ( options ) `
//! - [x] `createwallet "wallet_name" ( disable_private_keys )`
//! - [x] `dumpprivkey "address"`
//! - [x] `dumpwallet "filename"`
//! - [-] `encryptwallet "passphrase"`
//! - [-] `getaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [-] `getaccountaddress (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [-] `getaddressbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [x] `getaddressesbylabel "label"`
//! - [x] `getaddressinfo "address"`
//! - [x] `getbalance ( "(dummy)" minconf include_watchonly )`
//! - [x] `getnewaddress ( "label" "address_type" )`
//! - [ ] `getrawchangeaddress ( "address_type" )`
//! - [-] `getreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [ ] `getreceivedbyaddress "address" ( minconf )`
//! - [x] `gettransaction "txid" ( include_watchonly )`
//! - [ ] `getunconfirmedbalance`
//! - [ ] `getwalletinfo`
//! - [-] `importaddress "address" ( "label" rescan p2sh )`
//! - [-] `importmulti "requests" ( "options" )`
//! - [-] `importprivkey "privkey" ( "label" ) ( rescan )`
//! - [-] `importprunedfunds`
//! - [-] `importpubkey "pubkey" ( "label" rescan )`
//! - [-] `importwallet "filename"`
//! - [-] `keypoolrefill ( newsize )`
//! - [-] `listaccounts (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [ ] `listaddressgroupings`
//! - [ ] `listlabels ( "purpose" )`
//! - [ ] `listlockunspent`
//! - [-] `listreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [ ] `listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )`

//! - [ ] `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )`
//! - [ ] `listtransactions (label count skip include_watchonly)`
//! - [ ] `listunspent ( minconf maxconf  ["addresses",...] [include_unsafe] [query_options])`
//! - [ ] `listwallets`
//! - [x] `loadwallet "filename"`

//! - [ ] `lockunspent unlock ([{"txid":"txid","vout":n},...])`
//! - [ ] `move (Deprecated, will be removed in V0.18. To use this command, start bitcboind with -deprecatedrpc=accounts)`
//! - [ ] `removeprunedfunds "txid"`
//! - [ ] `rescanblockchain ("start_height") ("stop_height")`
//! - [ ] `sendfrom (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [ ] `sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode")`
//! - [x] `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")`
//! - [ ] `setaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - [ ] `sethdseed ( "newkeypool" "seed" )`
//! - [ ] `settxfee amount`
//! - [ ] `signmessage "address" "message"`
//! - [ ] `signrawtransactionwithwallet "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
//! - [ ] `unloadwallet ( "wallet_name" )`
//! - [ ] `walletcreatefundedpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable ) ( options bip32derivs )`
//! - [ ] `walletlock`
//! - [ ] `walletpassphrase "passphrase" timeout`
//! - [ ] `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - [ ] `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
//!
//! **== Zmq ==**
//! - [ ] `getzmqnotifications`

/// JSON-RPC types by API section.
mod blockchain;
mod control;
mod generating;
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

#[doc(inline)]
pub use self::{
    blockchain::{
        Bip9Softfork, Bip9SoftforkStatus, ChainTips, ChainTipsStatus, GetBestBlockHash,
        GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats,
        GetBlockVerbosityOne, GetBlockVerbosityZero, GetBlockchainInfo, GetChainTips,
        GetChainTxStats, GetDifficulty, GetMempoolAncestors, GetMempoolAncestorsVerbose, GetTxOut,
        ScriptPubkey, Softfork, SoftforkReject,
    },
    control::{GetMemoryInfoStats, Locked, Logging, Uptime},
    generating::{Generate, GenerateToAddress},
    network::{
        AddedNodeAddress, BytesPerMessage, GetAddedNodeInfo, GetNetTotals, GetNetworkInfo,
        GetNetworkInfoAddress, GetNetworkInfoNetwork, GetPeerInfo, PeerInfo, UploadTarget,
    },
    raw_transactions::SendRawTransaction,
    wallet::{
        CreateWallet, GetBalance, GetNewAddress, GetTransaction, GetTransactionDetail,
        GetTransactionDetailCategory, LoadWallet, SendToAddress,
    },
};
