// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::hex::FromHex;
use bitcoin::{
    address, amount, block, hex, network, Address, Amount, Block, BlockHash, CompactTarget,
    FeeRate, Network, ScriptBuf, TxMerkleNode, TxOut, Txid, Weight, Work,
};
use internals::write_err;
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBestBlockHash(pub String);

impl GetBestBlockHash {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBestBlockHash, hex::HexToArrayError> {
        let hash = self.0.parse::<BlockHash>()?;
        Ok(model::GetBestBlockHash(hash))
    }

    /// Converts json straight to a `bitcoin::BlockHash`.
    pub fn block_hash(self) -> Result<BlockHash, hex::HexToArrayError> { Ok(self.into_model()?.0) }
}

/// Result of JSON-RPC method `getblock` with verbosity set to 0.
///
/// A string that is serialized, hex-encoded data for block 'hash'.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetBlockVerbosityZero(pub String);

impl GetBlockVerbosityZero {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityZero, encode::FromHexError> {
        let block = encode::deserialize_hex(&self.0)?;
        Ok(model::GetBlockVerbosityZero(block))
    }

    /// Converts json straight to a `bitcoin::Block`.
    pub fn block(self) -> Result<Block, encode::FromHexError> { Ok(self.into_model()?.0) }
}

/// Result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetBlockVerbosityOne {
    /// The block hash (same as provided) in RPC call.
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i32,
    /// The block size.
    pub size: usize,
    /// The block size excluding witness data.
    #[serde(rename = "strippedsize")]
    pub stripped_size: Option<usize>,
    /// The block weight as defined in BIP-141.
    pub weight: u64,
    /// The block height or index.
    pub height: usize,
    /// The block version.
    pub version: i32,
    /// The block version formatted in hexadecimal.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block time expressed in UNIX epoch time.
    pub time: usize,
    /// The median block time expressed in UNIX epoch time.
    #[serde(rename = "mediantime")]
    pub median_time: Option<usize>,
    /// The nonce
    pub nonce: u32,
    /// The bits.
    pub bits: String,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block.
    #[serde(rename = "nTx")]
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available).
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

impl GetBlockVerbosityOne {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityOne, GetBlockVerbosityOneError> {
        use GetBlockVerbosityOneError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let weight = Weight::from_wu(self.weight); // TODO: Confirm this uses weight units.
        let version = block::Version::from_consensus(self.version);

        let tx = self
            .tx
            .iter()
            .map(|t| encode::deserialize_hex::<Txid>(t).map_err(E::Tx))
            .collect::<Result<Vec<_>, _>>()?;

        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;

        let previous_block_hash = match self.previous_block_hash {
            Some(hash) => Some(hash.parse::<BlockHash>().map_err(E::PreviousBlockHash)?),
            None => None,
        };
        let next_block_hash = match self.next_block_hash {
            Some(hash) => Some(hash.parse::<BlockHash>().map_err(E::NextBlockHash)?),
            None => None,
        };

        Ok(model::GetBlockVerbosityOne {
            hash,
            confirmations: self.confirmations,
            size: self.size,
            stripped_size: self.stripped_size,
            weight,
            height: self.height,
            version,
            version_hex: self.version_hex,
            merkle_root: self.merkle_root, // TODO: Use hash, which one depends on segwit or not
            tx,
            time: self.time, // TODO: Use stronger type.
            median_time: self.median_time,
            nonce: self.nonce,
            bits,
            difficulty: self.difficulty,
            chain_work,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }
}

/// Error when converting a `GetBlockVerbosityOne` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerbosityOneError {
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerbosityOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_ork` field failed"; e),
            PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
        }
    }
}

impl std::error::Error for GetBlockVerbosityOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => Some(e),
            Tx(ref e) => Some(e),
            Bits(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
}

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// Method call: `getblockchaininfo`
///
/// > Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: String,
    /// The current number of blocks processed in the server.
    pub blocks: u64,
    /// The current number of headers we have validated.
    pub headers: u64,
    /// The hash of the currently best block.
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// Median time for the current best block.
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// Estimate of verification progress (between 0 and 1).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// Estimate of whether this node is in Initial Block Download (IBD) mode.
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// Total amount of work in active chain, in hexadecimal.
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The estimated size of the block and undo files on disk.
    pub size_on_disk: u64,
    /// If the blocks are subject to pruning.
    pub pruned: bool,
    /// Lowest-height complete block stored (only present if pruning is enabled).
    #[serde(rename = "pruneheight")]
    pub prune_height: Option<u64>,
    /// Whether automatic pruning is enabled (only present if pruning is enabled).
    pub automatic_pruning: Option<bool>,
    /// The target size used by pruning (only present if automatic pruning is enabled).
    pub prune_target_size: Option<u64>,
    /// Status of softforks in progress.
    pub softforks: Vec<Softfork>,
    /// Status of BIP-9 softforks in progress, maps softfork name -> [`Softfork`].
    pub bip9_softforks: BTreeMap<String, Bip9Softfork>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// Name of softfork.
    id: String,
    /// Block version.
    version: usize,
    /// Progress toward rejecting pre-softfork blocks.
    reject: SoftforkReject,
}

/// Progress toward rejecting pre-softfork blocks.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SoftforkReject {
    /// `true` if threshold reached.
    status: bool,
}

/// Status of BIP-9 softforksin progress.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9Softfork {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: u64,
    /// Height of the first block to which the status applies.
    pub since: u32,
}

/// BIP-9 softfork status: one of "defined", "started", "locked_in", "active", "failed".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bip9SoftforkStatus {
    /// BIP-9 softfork status "defined".
    Defined,
    /// BIP-9 softfork status "started".
    Started,
    /// BIP-9 softfork status "locked_in".
    LockedIn,
    /// BIP-9 softfork status "active".
    Active,
    /// BIP-9 softfork status "failed".
    Failed,
}

impl GetBlockchainInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;

        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: self.blocks,
            headers: self.headers,
            best_block_hash,
            difficulty: self.difficulty,
            median_time: self.median_time,
            verification_progress: self.verification_progress,
            initial_block_download: self.initial_block_download,
            chain_work,
            size_on_disk: self.size_on_disk,
            pruned: self.pruned,
            prune_height: self.prune_height,
            automatic_pruning: self.automatic_pruning,
            prune_target_size: self.prune_target_size,
            softforks,
            warnings: self.warnings,
        })
    }
}

impl Bip9SoftforkStatus {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::Bip9SoftforkStatus {
        use model::Bip9SoftforkStatus::*;

        match self {
            Self::Defined => Defined,
            Self::Started => Started,
            Self::LockedIn => LockedIn,
            Self::Active => Active,
            Self::Failed => Failed,
        }
    }
}

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    Chain(network::ParseNetworkError),
    BestBlockHash(hex::HexToArrayError),
    ChainWork(UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockchainInfoError::*;

        match *self {
            Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
        }
    }
}

impl std::error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockchainInfoError::*;

        match *self {
            Chain(ref e) => Some(e),
            BestBlockHash(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
        }
    }
}

/// Result of JSON-RPC method `getblockcount`.
///
/// > getblockcount
///
/// > Returns the number of blocks in the longest blockchain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockCount(pub u64);

impl GetBlockCount {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetBlockCount { model::GetBlockCount(self.0) }
}

/// Result of JSON-RPC method `getblockhash`.
///
/// > Returns hash of block in best-block-chain at height provided.
///
/// > Arguments:
/// > 1. height         (numeric, required) The height index
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHash(pub String);

impl GetBlockHash {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHash, hex::HexToArrayError> {
        let hash = self.0.parse::<BlockHash>()?;
        Ok(model::GetBlockHash(hash))
    }

    /// Converts json straight to a `bitcoin::BlockHash`.
    pub fn block_hash(self) -> Result<BlockHash, hex::HexToArrayError> { Ok(self.into_model()?.0) }
}

/// Result of JSON-RPC method `getblockheader` with verbosity set to `false`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader 'hash'.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
/// > 2. verbose         (boolean, optional, default=true) true for a json object, false for the hex encoded data
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeader(pub String);

impl GetBlockHeader {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeader, GetBlockHeaderError> {
        use GetBlockHeaderError as E;

        let v = Vec::from_hex(&self.0).map_err(E::Hex)?;
        let header = encode::deserialize::<block::Header>(&v).map_err(E::Consensus)?;

        Ok(model::GetBlockHeader(header))
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, GetBlockHeaderError> {
        Ok(self.into_model()?.0)
    }
}

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderError {
    /// Conversion of hex data to bytes failed.
    Hex(hex::HexToBytesError),
    /// Consensus decoding of bytes to header failed.
    Consensus(encode::Error),
}

impl fmt::Display for GetBlockHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockHeaderError::*;

        match *self {
            Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
            Consensus(ref e) => write_err!(f, "consensus decoding of bytes to header failed"; e),
        }
    }
}

impl std::error::Error for GetBlockHeaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockHeaderError::*;

        match *self {
            Hex(ref e) => Some(e),
            Consensus(ref e) => Some(e),
        }
    }
}

/// Result of JSON-RPC method `getblockheader` with verbosity set to `true`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader `<hash>`.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
/// > 2. verbose         (boolean, optional, default=true) true for a json object, false for the hex encoded data
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockHeaderVerbose {
    /// The block hash (same as provided).
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i64,
    /// The block height or index.
    pub height: u64,
    /// The block version.
    pub version: i32,
    /// The block version formatted in hexadecimal.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root.
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u64,
    /// The median block time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// The nonce.
    pub nonce: u64,
    /// The bits.
    pub bits: String,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the current chain (in hex).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block.
    #[serde(rename = "nTx")]
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available).
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

impl GetBlockHeaderVerbose {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        use GetBlockHeaderVerboseError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let version = block::Version::from_consensus(self.version);
        let merkle_root = self.merkle_root.parse::<TxMerkleNode>().map_err(E::MerkleRoot)?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.bits).map_err(E::ChainWork)?;
        let previous_block_hash = self
            .previous_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::PreviousBlockHash))
            .transpose()?;
        let next_block_hash = self
            .next_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::NextBlockHash))
            .transpose()?;

        Ok(model::GetBlockHeaderVerbose {
            hash,
            confirmations: self.confirmations,
            height: self.height,
            version,
            merkle_root,
            time: self.time,
            median_time: self.median_time,
            nonce: self.nonce,
            bits,
            difficulty: self.difficulty,
            chain_work,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, hex::HexToArrayError> { todo!() }
}

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderVerboseError {
    /// Conversion of `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

/// Result of JSON-RPC method `getblockstats`.
///
/// > getblockstats hash_or_height ( stats )
///
/// > Returns the number of blocks in the longest blockchain.
/// > getblockstats hash_or_height ( stats )
/// >
/// > Compute per block statistics for a given window. All amounts are in satoshis.
/// > It won't work for some heights with pruning.
/// > It won't work without -txindex for utxo_size_inc, *fee or *feerate stats.
/// >
/// > Arguments:
/// > 1. "hash_or_height"     (string or numeric, required) The block hash or height of the target block
/// > 2. "stats"              (array,  optional) Values to plot, by default all values (see result below)
/// >     [
/// >       "height",         (string, optional) Selected statistic
/// >       "time",           (string, optional) Selected statistic
/// >       ,...
/// >     ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
// FIXME: Should these fields be u64 or u32?
pub struct GetBlockStats {
    /// Average fee in the block.
    #[serde(rename = "avgfee")]
    pub average_fee: u64,
    // TODO: Remember these docs will become silently stale when unit changes in a later version of Core.
    /// Average feerate (in satoshis per virtual byte).
    #[serde(rename = "avgfeerate")]
    pub average_fee_rate: u64,
    /// Average transaction size.
    #[serde(rename = "avgtxsize")]
    pub average_tx_size: u64,
    /// The block hash (to check for potential reorgs).
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per
    /// virtual byte).
    #[serde(rename = "feerate_percentiles")]
    pub fee_rate_percentiles: [u64; 5],
    /// The height of the block.
    pub height: u64,
    /// The number of inputs (excluding coinbase).
    #[serde(rename = "ins")]
    pub inputs: u64,
    /// Maximum fee in the block.
    #[serde(rename = "maxfee")]
    pub max_fee: u64,
    /// Maximum feerate (in satoshis per virtual byte).
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: u64,
    /// Maximum transaction size.
    #[serde(rename = "maxtxsize")]
    pub max_tx_size: u64,
    /// Truncated median fee in the block.
    #[serde(rename = "medianfee")]
    pub median_fee: u64,
    /// The block median time past.
    #[serde(rename = "mediantime")]
    pub median_time: u32,
    /// Truncated median transaction size
    #[serde(rename = "mediantxsize")]
    pub median_tx_size: u64,
    /// Minimum fee in the block.
    #[serde(rename = "minfee")]
    pub minimum_fee: u64,
    /// Minimum feerate (in satoshis per virtual byte).
    #[serde(rename = "minfeerate")]
    pub minimum_fee_rate: u64,
    /// Minimum transaction size.
    #[serde(rename = "mintxsize")]
    pub minimum_tx_size: u64,
    /// The number of outputs.
    #[serde(rename = "outs")]
    pub outputs: u64,
    /// The block subsidy.
    pub subsidy: u64,
    /// Total size of all segwit transactions.
    #[serde(rename = "swtotal_size")]
    pub segwit_total_size: u64,
    /// Total weight of all segwit transactions divided by segwit scale factor (4).
    #[serde(rename = "swtotal_weight")]
    pub segwit_total_weight: u64,
    /// The number of segwit transactions.
    #[serde(rename = "swtxs")]
    pub segwit_txs: u64,
    /// The block time.
    pub time: u32,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee]).
    pub total_out: u64,
    /// Total size of all non-coinbase transactions.
    pub total_size: u64,
    /// Total weight of all non-coinbase transactions divided by segwit scale factor (4).
    pub total_weight: u64,
    /// The fee total.
    #[serde(rename = "totalfee")]
    pub total_fee: u64,
    /// The number of transactions (excluding coinbase).
    pub txs: u64,
    /// The increase/decrease in the number of unspent outputs.
    pub utxo_increase: u64,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar).
    #[serde(rename = "utxo_size_inc")]
    pub utxo_size_increase: u64,
}

impl GetBlockStats {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockStats, hex::HexToArrayError> {
        let block_hash = self.block_hash.parse::<BlockHash>()?;
        let fee_rate_percentiles = self
            .fee_rate_percentiles
            .iter()
            .map(|vb| FeeRate::from_sat_per_vb(*vb))
            .collect::<Vec<Option<FeeRate>>>();

        // `FeeRate::sat_per_vb` returns an option if value overflows.
        let average_fee_rate = FeeRate::from_sat_per_vb(self.average_fee_rate);
        let max_fee_rate = FeeRate::from_sat_per_vb(self.max_fee_rate);
        let minimum_fee_rate = FeeRate::from_sat_per_vb(self.minimum_fee_rate);

        // FIXME: Double check that these values are virtual bytes and not weight units.
        let segwit_total_weight = Weight::from_vb(self.segwit_total_weight);
        let total_weight = Weight::from_vb(self.total_weight);

        Ok(model::GetBlockStats {
            average_fee: Amount::from_sat(self.average_fee),
            average_fee_rate,
            average_tx_size: self.average_tx_size,
            block_hash,
            fee_rate_percentiles,
            height: self.height,
            inputs: self.inputs,
            max_fee: Amount::from_sat(self.max_fee),
            max_fee_rate,
            max_tx_size: self.max_tx_size,
            median_fee: Amount::from_sat(self.median_fee),
            median_time: self.median_time,
            median_tx_size: self.median_tx_size,
            minimum_fee: Amount::from_sat(self.minimum_fee),
            minimum_fee_rate,
            minimum_tx_size: self.minimum_tx_size,
            outputs: self.outputs,
            subsidy: Amount::from_sat(self.subsidy),
            segwit_total_size: self.segwit_total_size,
            segwit_total_weight,
            segwit_txs: self.segwit_txs,
            time: self.time,
            total_out: Amount::from_sat(self.total_out),
            total_size: self.total_size,
            total_weight,
            total_fee: Amount::from_sat(self.total_fee),
            txs: self.txs,
            utxo_increase: self.utxo_increase,
            utxo_size_increase: self.utxo_size_increase,
        })
    }
}

/// Result of JSON-RPC method `getchaintips`.
///
/// > Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetChainTips(pub Vec<ChainTips>);

/// An individual list item from the result of JSON-RPC method `getchaintips`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ChainTips {
    /// Height of the chain tip.
    pub height: u64,
    /// Block hash of the tip.
    pub hash: String,
    /// Zero for main chain.
    #[serde(rename = "branchlen")]
    pub branch_length: u64,
    /// "active" for the main chain.
    pub status: ChainTipsStatus,
}

/// The `status` field from an individual list item from the result of JSON-RPC method `getchaintips`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChainTipsStatus {
    /// This branch contains at least one invalid block.
    Invalid,
    /// Not all blocks for this branch are available, but the headers are valid.
    HeadersOnly,
    /// All blocks are available for this branch, but they were never fully validated.
    ValidHeaders,
    /// This branch is not part of the active chain, but is fully validated.
    ValidFork,
    /// This is the tip of the active main chain, which is certainly valid.
    Active,
}

impl GetChainTips {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetChainTips, hex::HexToArrayError> {
        let v = self.0.into_iter().map(|item| item.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetChainTips(v))
    }
}

impl ChainTips {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ChainTips, hex::HexToArrayError> {
        let hash = self.hash.parse::<BlockHash>()?;

        Ok(model::ChainTips {
            height: self.height,
            hash,
            branch_length: self.branch_length,
            status: self.status.into_model(),
        })
    }
}

impl ChainTipsStatus {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::ChainTipsStatus {
        use model::ChainTipsStatus::*;

        match self {
            Self::Invalid => Invalid,
            Self::HeadersOnly => HeadersOnly,
            Self::ValidHeaders => ValidHeaders,
            Self::ValidFork => ValidFork,
            Self::Active => Active,
        }
    }
}

/// Result of JSON-RPC method `getchaintxstats`.
///
/// > getchaintxstats ( nblocks blockhash )
/// >
/// > Compute statistics about the total number and rate of transactions in the chain.
/// >
/// > Arguments:
/// > 1. nblocks      (numeric, optional) Size of the window in number of blocks (default: one month).
/// > 2. "blockhash"  (string, optional) The hash of the block that ends the window.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window in UNIX format.
    pub time: u32,
    /// The total number of transactions in the chain up to that point.
    #[serde(rename = "txcount")]
    pub tx_count: u64,
    /// The hash of the final block in the window.
    pub window_final_block_hash: String,
    /// Size of the window in number of blocks.
    pub window_block_count: u64,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0.
    pub window_tx_count: Option<u64>,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0.
    pub window_interval: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0.
    #[serde(rename = "txrate")]
    pub tx_rate: Option<u64>,
}

impl GetChainTxStats {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetChainTxStats, hex::HexToArrayError> {
        let window_final_block_hash = self.window_final_block_hash.parse::<BlockHash>()?;

        Ok(model::GetChainTxStats {
            time: self.time,
            tx_count: self.tx_count,
            window_final_block_hash,
            window_block_count: self.window_block_count,
            window_tx_count: self.window_tx_count,
            window_interval: self.window_interval,
            tx_rate: self.tx_rate,
        })
    }
}

/// Result of JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
///
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
/// >
/// > Result:
/// > n.nnn       (numeric) the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetDifficulty(pub f64);

impl GetDifficulty {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetDifficulty { model::GetDifficulty(self.0) }
}

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to false.
///
/// > getmempoolancestors txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool ancestors.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
/// > 2. verbose                  (boolean, optional, default=false) True for a json object, false for array of transaction ids
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestors(pub Vec<String>);

impl GetMempoolAncestors {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolAncestors, encode::FromHexError> {
        let v = self
            .0
            .iter()
            .map(|t| encode::deserialize_hex::<Txid>(t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetMempoolAncestors(v))
    }
}

/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolAncestorsVerbose {}

impl GetMempoolAncestorsVerbose {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetMempoolAncestorsVerbose {
        model::GetMempoolAncestorsVerbose {}
    }
}

/// Result of JSON-RPC method `gettxout`.
///
/// > gettxout "txid" n ( include_mempool )
/// >
/// > Returns details about an unspent transaction output.
/// >
/// > Arguments:
/// > 1. txid               (string, required) The transaction id
/// > 2. n                  (numeric, required) vout number
/// > 3. include_mempool    (boolean, optional, default=true) Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations.
    pub confirmations: u32,
    /// The transaction value in BTC.
    pub value: f64,
    /// The script pubkey.
    #[serde(rename = "scriptPubkey")]
    pub script_pubkey: ScriptPubkey,
    /// Coinbase or not.
    pub coinbase: bool,
}

/// A script pubkey.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ScriptPubkey {
    /// Script assembly.
    pub asm: String,
    /// Script hex.
    pub hex: String,
    // TODO: Add support for deprecatedrpc=addresses
    // #[serde(rename = "reqSigs")]
    // pub req_sigs: u64,
    /// The type, eg pubkeyhash
    #[serde(rename = "type")]
    pub type_: String,
    /// bitcoin address
    pub address: String,
    // TODO: Add support for deprecatedrpc=addresses
    // pub addressess: Vec<String>,
}

impl GetTxOut {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOut, GetTxOutError> {
        use GetTxOutError as E;

        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;

        let tx_out = TxOut {
            value: Amount::from_btc(self.value).map_err(E::Value)?,
            script_pubkey: ScriptBuf::from_hex(&self.script_pubkey.hex).map_err(E::ScriptPubkey)?,
        };

        let address = self.script_pubkey.address.parse::<Address<NetworkUnchecked>>().map_err(E::Address)?.assume_checked();

        Ok(model::GetTxOut {
            best_block,
            confirmations: self.confirmations,
            tx_out,
            address,
            coinbase: self.coinbase,
        })
    }
}

/// Error when converting a `GetTxOut` type into the model type.
#[derive(Debug)]
pub enum GetTxOutError {
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the transaction `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// Conversion of the transaction `address` field failed.
    Address(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => write_err!(f, "conversion of the `best_block` field failed"; e),
            Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
        }
    }
}

impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => Some(e),
            Value(ref e) => Some(e),
            ScriptPubkey(ref e) => Some(e),
            Address(ref e) => Some(e),
        }
    }
}
