// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use std::fmt;
use std::str::FromStr;

use bitcoin::address::NetworkUnchecked;
use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{address, hex, Address, Amount, SignedAmount, Transaction, Txid};
use internals::write_err;
use serde::{Deserialize, Serialize};

use crate::model;

// # Notes
//
// The following structs are very similar but have slightly different fields and docs.
// - GetTransaction
// - ListSinceLastBlockTransaction
// - ListTransactionsItem

/// Returned as part of `getaddressesbylabel` and `getaddressinfo`
pub enum AddressPurpose {
    /// A send-to address.
    Send,
    /// A receive-from address.
    Receive,
}

impl AddressPurpose {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddressPurpose, AddressPurposeError> {
        use AddressPurpose::*;
        
        match self {
            Send => model::AddressPurpose::Send,
            Receive => model::AddressPurpos::Receive,
        }
    }
}

/// The category of a transaction.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionCategory {
    /// Transaction is a send.
    Send,
    /// Transactions is a receive.
    Receive,
}

impl TransactionCategory {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::TransactionCategory, TransactionCategoryError> {
        use TransactionCategory::*;
        
        match self {
            Send => model::TransactionCategory::Send,
            Receive => model::TransactionCategory::Receive,
        }
    }
}

/// Whether this transaction can be RBF'ed.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bip125Replacable {
    /// Yes, can be replaced due to BIP-125 (RBF).
    Yes,
    /// No, cannot be replaced due to BIP-125 (RBF).
    No,
    /// RBF unknown.
    Unknown,
}

impl Bip125Replaceable {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Bip125Replaceable, Bip125ReplaceableError> {
        use Bip125Replaceable::*;
        
        match self {
            Yes => model::Bip125Replaceable::Yes,
            No => model::Bip125Replaceable::No,
            Unknown => model::Bip125Replaceable::Unknown,
        }
    }
}

/// Result of the JSON-RPC method `addmultisigaddress`.
///
/// > addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
/// >
/// > Add a nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// > Each key is a Bitcoin address or hex-encoded public key.
/// > This functionality is only intended for use with non-watchonly addresses.
/// > See `importaddress` for watchonly p2sh address support.
/// > If 'label' is specified, assign address to that label.
///
/// > Arguments:
/// > 1. nrequired                      (numeric, required) The number of required signatures out of the n keys or addresses.
/// > 2. "keys"                         (string, required) A json array of bitcoin addresses or hex-encoded public keys
/// > 3. "label"                        (string, optional) A label to assign the addresses to.
/// > 4. "address_type"                 (string, optional) The address type to use. Options are "legacy", "p2sh-segwit", and "bech32". Default is set by -addresstype.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddMultisigAddress {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
}

impl AddMultisigAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddMultisigAddress, AddMultisigAddressError> {
        use GetMultisigAddressError as E;

        let address = Address::from_str(&self.script_pubkey.address).map_err(E::Address)?;
        let redeem_script = ScriptBuf::from_hex(&self.script_pubkey.hex).map_err(E::RedeemScript)?,

        Ok(model::AddMultisigAddress { address, redeem_script })
    }
}

/// Error when converting a `AddMultisigAddress` type into the model type.
#[derive(Debug)]
pub enum AddMultisigAddressError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
}

impl fmt::Display for AddMultisigAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddMultisigAddressError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            RedeemScript(ref e) =>
                write_err!(f, "conversion of the `redeem_script` field failed"; e),
        }
    }
}

impl std::error::Error for AddMultisigAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use AddMultisigAddressError::*;

        match *self {
            Address(ref e) => Some(e),
            RedeemScript(ref e) => Some(e),        }
    }
}

/// Result of the JSON-RPC method `bumpfee`.
///
/// > bumpfee "txid" ( options )
/// >
/// > Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// > An opt-in RBF transaction with the given txid must be in the wallet.
/// > The command will pay the additional fee by decreasing (or perhaps removing) its change output.
/// > If the change output is not big enough to cover the increased fee, the command will currently fail
/// > instead of adding new inputs to compensate. (A future implementation could improve this.)
/// > The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// > By default, the new fee will be calculated automatically using estimatesmartfee.
/// > The user can specify a confirmation target for estimatesmartfee.
/// > Alternatively, the user can specify totalFee, or use RPC settxfee to set a higher fee rate.
/// > At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// > returned by getnetworkinfo) to enter the node's mempool.
/// >
/// > Arguments:
/// > 1. txid                  (string, required) The txid to be bumped
/// > 2. options               (object, optional) - Elided, see Core docs for info.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BumpFee {
    /// The id of the new transaction.
    pub txid: String,
    /// Fee of the replaced transaction.
    #[serde(rename = "origfee")]
    pub original_fee: u64,
    /// Fee of the new transaction.
    pub fee: u64,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

impl BumpFee {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::BumpFee, encode::FromHexError> {
        use AddMultisigAddressError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let originl_fee = Amount::from_sat(original_fee);
        let fee = Amount::from_sat(fee);

        Ok(model::BumpFee { txid, original_fee, fee, errors: self.errors })
    }
}

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys )
/// >
/// > Creates and loads a new wallet.
/// >
/// > Arguments:
/// > 1. "wallet_name"          (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys   (boolean, optional, default: false) Disable the possibility of private keys (only watchonlys are possible in this mode).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warning: String,
}

impl CreateWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: vec![self.warning] }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

/// Result of the JSON-RPC method `dumpprivkey`.
///
/// > dumpprivkey "address"
/// >
/// > Reveals the private key corresponding to 'address'.
/// > Then the importprivkey can be used with this output
/// >
/// > Arguments:
/// > 1. "address"   (string, required) The bitcoin address for the private key
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpPrivKey {
    /// The private key.
    pub key: String,
}

impl DumpPrivKey {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::CreateWalletv, key::FromWifError> {
        let key = self.key.parse::<PrivateKey>()?;
        Ok(model::DumpPrivKey { key })
    }

    /// Returns the dumped key.
    pub fn key(self) -> Result<model::CreateWalletv, key::FromWifError> {
        self.into_model()?.key
    }
}

/// Result of the JSON-RPC method `dumpwallet`.
///
/// > dumpwallet "filename"
/// >
/// > Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// > Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// > Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// > only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The filename with path (either absolute or relative to bitcoind)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpWallet {
    /// The filename with full absolute path.
    #[serde(rename = "filename")]
    pub file_name: String,
}

/// Result of the JSON-RPC method `getaddressesbylabel`.
///
/// > getaddressesbylabel "label"
/// >
/// > Returns the list of addresses assigned the specified label.
/// >
/// > Arguments:
/// > 1. "label"  (string, required) The label.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressesByLabel {
    /// Map of address to information about address.
    pub addresses: BTreeMap<String, AddressPurpose>,
}

impl GetAddressByLabel {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressByLabel, GetAddressByLabelError> {
        let mut addresses = BTreeMap::new();

        for (k, v) in self.addresses.iter() {
            let address = k.parse::<Address<NetworkUnchecked>>.()?.assume_checked();
            let purpose = v.into_model()?;
            addresses.insert(address, purpose);
        }

        Ok(model::GetAddressByLabel { addresses })
    }
}

/// Core returned an undocumented/invalid purpose.
#[derive(debug)]
pub enum GetAddressesByLabelError {
    /// Conversion of an address string failed.
    Address(address::ParseError),
    /// Conversion of a purpose string failed.
    Purpose(PurposeError),
};

impl fmt::Display for GetAddressesByLabelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetAddressesByLabelError::*;

        match *self {
            Address(ref e) => write_err!(f, "invalid address in map"; e),
            Purpose(ref e) => write_err!(f, "invalid purpose in map"; e),
        }
    }
}

impl std::error::Error for GetAddressesByLabelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetAddressesByLabelError::*;

        match *self {
            Address(ref e) => Some(e),
            Purpose(ref e) => Some(e),
        }
    }
}

/// Result of the JSON-RPC method `getaddressinfo`.
///
/// > getaddressinfo "address"
/// >
/// > Return information about the given bitcoin address. Some information requires the address
/// > to be in the wallet.
/// >
/// > Arguments:
/// > 1. "address"                    (string, required) The bitcoin address to get the information of.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfo {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex encoded scriptPubKey generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// If the address is yours or not.
    #[serde(rename = "ismine")]
    pub is_mine: bool,
    /// If the address is watchonly.
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<i32>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<GetAddressInfoScriptType>,
    /// The redeemscript for the p2sh address.
    pub hex: Optional<String>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Vec<String>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<i32>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// If the address is compressed.
    #[serde(rename = "iscompressed")]
    pub is_compressed: bool,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// DEPRECATED. The account associated with the address, "" is the default account.
    pub account: String,
    /// The creation time of the key if available in seconds since epoch (Jan 1 1970 GMT).
    pub timestamp: Option<u32>,
    /// The HD keypath if the key is HD and available.
    #[serde(rename = "hdkeypath")]
    pub hd_key_path: Option<String>,
    /// The Hash160 of the HD seed.
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// Alias for hdseedid maintained for backwards compatibility.
    ///
    /// Will be removed in V0.18.
    #[serde(rename = "hdmasterkeyid")]
    pub hd_master_key_id: Option<String>,
    /// Array of labels associated with the address.
    pub labels: Vec<GetAddressInfoLabel>,
}

impl GetAddressInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfo, GetAddressInfoError> {
        use GetAddressInfoError as E;

        let address = self.address.parse::<Address<NetworkChecked>>().map_err(E::Address)?.assume_checked();
        let script_pubkey = ScriptBuf::from_hex(self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program) {
            (Some(v), Some(hex)) => {
                if v > u8::MAX || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version = WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(hex).map_err(E::WitnessProgramBytes)?;
                let witness_program = WitnessProgram::new(witness_version, bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            } 
            _ => (None, None),          // TODO: Think more if catchall is ok.
        };
        let redeem_script = self.hex.map(|hex| ScriptBuf::from_hex(hex).map_err(E::Hex)).transpose().map_err(E::Hex)?;
        let pubkeys = self.pubkeys.iter().map(|s| s.parse::<PublicKey>()).collect::<Result<Vec<_>, _>>().map_err(E::Pubkeys)?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).collect::<Result<PublicKey, _>>().transpose().map_err(E::Pubkey)?;
        let embedded = self.embedded.into_model()?;
        let hd_key_path = self.hd_key_path.parse::<bip32::DerivationPath>().transpose().map_err(E::HdKeyPath)?;
        let hd_seed_id = self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;
        let labels = self.labels.into_model().map_err(E::Labels)?;

        Ok(model::GetAddressInfo {
            address,
            script_pubkey: self.script_pubkey,
            is_mine: self.is_mine,
            is_watch_only: self.is_watch_only,
            is_script: self.is_script,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script: self.script,
            hex: redeem_script,
            pubkeys,
            sigs_required: self.sigs_required,
            pubkey,
            embedded,
            is_compressed: self.is_compressed,
            label: self.label,
            timestamp: self.timestamp,
            hd_key_path,
            hd_seed_id,
            labels,
        })
    }
}

/// Error when converting a `GetAddressInfo` type into the model type.
#[derive(Debug)]
pub enum GetAddressInfoError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToArrayError),
    /// The `witness_version` field's value was too big for a u8.
    WitnessVersionValue(i32),
    /// Conversion of the `witness_version` field failed.
    WitnessVersion(witness_version::TryFromError),
    /// Conversion of the `witness_program` field failed.
    WitnessProgram(witness_program::Error),
    /// Conversion of the `hex` field failed.
    Hex(hex::HexToArrayError),
    /// Conversion of the `pubkeys` field failed.
    Pubkeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    Pubkey(key::ParsePublicKeyError),
    /// Conversion of the `embedded` field failed.
    Embedded(GetAddressInfoEmbeddedError),
    /// Conversion of the `hd_key_path` field failed.
    HdKeyPath(hex::HexToArrayError),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
    /// Conversion of the `labels` field failed.
    Labels(AddressLabelError),
}

impl fmt::Display for GetAddressInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetAddressInfoError as E;

        match *self {
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::ScriptPubkey(ref e) => write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            E::WitnessVersion(v) => write!(f, "invalid witness version number: {}", v),
            E::WitnessVersion(ref e) => write_err!(f, "conversion of the `witness_version` field failed"; e),
            E::WitnessProgram(ref e) => write_err!(f, "conversion of the `witness_program` field failed"; e),
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Pubkeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
            E::Embedded(ref e) => write_err!(f, "conversion of the `embedde` field failed"; e),
            E::HdKeyPath(ref e) => write_err!(f, "conversion of the `hd_key_path` field failed"; e),
            E::HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
            E::Labels(ref e) => write_err!(f, "conversion of the `labels` field failed"; e),
        }
    }
}

impl std::error::Error for GetAddressInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetAddressInfoError as E;

        match *self {
            E::Address(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
            E::WitnessVersion(v) => None,
            E::WitnessVersion(ref e) => Some(e),
            E::WitnessProgram(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Pubkeys(ref e) => Some(e),
            E::Pubkey(ref e) => Some(e),
            E::Embedded(ref e) => Some(e),
            E::HdKeyPath(ref e) => Some(e),
            E::HdSeedId(ref e) => Some(e),
            E::Labels(ref e) => Some(e),
        }
    }
}

/// The `script` field of `GetAddressInfo` (and `GetAddressInfoEmbedded`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum GetAddressInfoScriptType {
    /// Non-standard output script type.
    #[serde(rename = "nonstandard")]
    NonStandard,
    /// Pubkey output script.
    #[serde(rename = "pubkey")]
    Pubkey,
    /// Pubkey hash output script.
    #[serde(rename = "pubkeyhash")]
    PubkeyHash,
    /// Script hash output script.
    #[serde(rename = "scripthash")]
    ScriptHash,
    /// Multisig output script.
    #[serde(rename = "multisig")]
    Multisig,
    /// Null data for output script.
    #[serde(rename = "nulldata")]
    NullData,
    /// Witness version 0 key hash output script.
    #[serde(rename = "witness_v0_keyhash")]
    WitnessV0KeyHash,
    /// Witness version 0 script hash output script.
    #[serde(rename = "witness_v0_scripthash")]
    WitnessV0ScriptHash,
    /// Witness unknown for output script.
    #[serde(rename = "witness_unknown")]
    WitnessUnknown,
}

impl GetAddressInfoScriptType {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::ScriptType {
        use GetAddressInfoScriptType as V; // V for version specific.
        use model::ScriptType as M;        // M for model.

        let model = match *self {
            V::NonStandard => M::NonStandard,
            V::Pubkey => M::Pubkey,
            V::PubkeyHash => M::PubkeyHash,
            V::ScriptHash => M::ScriptHash,
            V::Multisig => M::Multisig,
            V::NullData => M::NullData,
            V::WitnessV0KeyHash => M::WitnessV0KeyHash,
            V::WitnessV0ScriptHash => M::WitnessV0ScriptHash,
            V::WitnessVersion => M::WitnessVersion,
        };
        Ok(model)
    }
}

/// The `label` field of `GetAddressInfo` (and `GetAddressInfoEmbedded`).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfoLabel {
    /// The label.
    pub name: String,
    /// Purpose of address ("send" for sending address, "receive" for receiving address).
    pub purpose: Purpose,
}

impl GetAddressInfoLabel {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddressLabel, PurposeError> {
        Ok(model::AddressLabel {
            name: self.name,
            purpose: self.purpose.into_model()?,
        })
    }
}

/// The `embedded` field of `GetAddressInfo`.
///
/// It includes all getaddressinfo output fields for the embedded address, excluding metadata
/// ("timestamp", "hdkeypath", "hdseedid") and relation to the wallet ("ismine", "iswatchonly",
/// "account").
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfoEmbedded {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex encoded scriptPubKey generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<i32>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<GetAddressInfoScript>,
    /// The redeemscript for the p2sh address.
    pub hex: Optional<String>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Vec<String>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<i32>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// If the address is compressed.
    #[serde(rename = "iscompressed")]
    pub is_compressed: true,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// Array of labels associated with the address.
    pub labels: Vec<GetAddressInfoLabel>,
}

impl GetAddressInfoEmbedded {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfoEmbedded, GetAddressInfoError> {
        todo!("Copy GetAddressInfo::into_model once that builds")
    }
}

/// Result of the JSON-RPC method `getbalance`.
///
/// > getbalance ( "(dummy)" minconf include_watchonly )
/// >
/// > Returns the total available balance.
/// > The available balance is what the wallet considers currently spendable, and is
/// > thus affected by options which limit spendability such as -spendzeroconfchange.
/// >
/// > Arguments:
/// > 1. (dummy)           (string, optional) Remains for backward compatibility. Must be excluded or set to "*".
/// > 2. minconf           (numeric, optional, default=0) Only include transactions confirmed at least this many times.
/// > 3. include_watchonly (bool, optional, default=false) Also include balance in watch-only addresses (see 'importaddress')
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(pub f64);

impl GetBalance {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalance, ParseAmountError> {
        let amount = Amount::from_btc(self.0)?;
        Ok(model::GetBalance(amount))
    }

    /// Converts json straight to a `bitcoin::Amount`.
    pub fn balance(self) -> Result<Amount, ParseAmountError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress ( "label" "address_type" )
/// >
/// > Returns a new Bitcoin address for receiving payments.
/// > If 'label' is specified, it is added to the address book
/// > so payments received with the address will be associated with 'label'.
/// >
/// > Arguments:
/// > 1. "label"          (string, optional) The label name for the address to be linked to. If not provided, the default label "" is used. It can also be set to the empty string "" to represent the default label. The label does not need to exist, it will be created if there is no label by the given name.
/// > 2. "address_type"   (string, optional) The address type to use. Options are "legacy", "p2sh-segwit", and "bech32". Default is set by -addresstype.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNewAddress(pub String);

impl GetNewAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNewAddress, address::ParseError> {
        let address = Address::from_str(&self.0)?;
        Ok(model::GetNewAddress(address))
    }

    /// Converts json straight to a `bitcoin::Address`.
    pub fn address(self) -> Result<Address<NetworkUnchecked>, address::ParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

/// Result of the JSON-RPC method `getrawchangeaddress`.
///
/// > getrawchangeaddress ( "address_type" )
/// >
/// > Returns a new Bitcoin address, for receiving change.
/// > This is for use with raw transactions, NOT normal use.
/// >
/// > Arguments:
/// > 1. "address_type"           (string, optional) The address type to use. Options are "legacy", "p2sh-segwit", and "bech32". Default is set by -changetype.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawChangeAddress(pub String);

impl GetRawChangeAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawChangeAddress, address::ParseError> {
        let address = self.address.parse::<Address<_>>()?.assume_checked();
        Ok(model::GetRawChangeAddress(address))
    }
}

/// Result of the JSON-RPC method `getreceivedbyaddress`.
///
/// > getreceivedbyaddress "address" ( minconf )
/// >
/// > Returns the total amount received by the given address in transactions with at least minconf confirmations.
/// >
/// > Arguments:
/// > 1. "address"         (string, required) The bitcoin address for transactions.
/// > 2. minconf             (numeric, optional, default=1) Only include transactions confirmed at least this many times.
pub struct GetReceivedByAddress(pub f64); // Amount in BTC.

impl GetReceivedByAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetReceivedByAddress, ParseAmountError> {
        let amount = Amount::from_btc(self.amount)?;
        Ok(model::GetReceivedByAddress(amount))
    }
}

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
/// > 2. include_watchonly    (boolean, optional, default=false) Whether to include watch-only addresses in balance calculation and details[]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransaction {
    /// DEPRECATED. The account name.
    pub account: String,
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// The number of confirmations.
    pub confirmations: u32,
    // The docs say these two more fields should exist but integration
    // test fails if we include them them i.e., they are not returned by `v0.17.1`.
    /// The block hash.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: i64,
    /// The time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: u32,
    /// The transaction id.
    pub txid: String,
    /// The transaction time in seconds since epoch (1 Jan 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replacable,
    /// Transaction details.
    pub details: Vec<GetTransactionDetail>,
    /// Raw data for transaction.
    pub hex: String,
}

impl GetTransaction {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        // FIMXE: Use combinators.
        let fee = match self.fee {
            None => None,
            Some(f) => Some(SignedAmount::from_btc(f).map_err(E::Fee)?),
        };
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;

        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let mut details = vec![];
        for detail in self.details {
            let concrete = detail.into_model().map_err(E::Details)?;
            details.push(concrete);
        }

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            txid,
            time: self.time,
            time_received: self.time_received,
            bip125_replaceable: self.bip125_replaceable,
            details,
            tx,
        })
    }
}

/// Error when converting a `GetTransaction` type into the model type.
#[derive(Debug)]
pub enum GetTransactionError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the `details` field failed.
    Details(GetTransactionDetailError),
}

impl fmt::Display for GetTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionError as E;

        match *self {
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Tx(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Details(ref e) => write_err!(f, "conversion of the `details` field failed"; e),
        }
    }
}

impl std::error::Error for GetTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionError as E;

        match *self {
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Tx(ref e) => Some(e),
            E::Details(ref e) => Some(e),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetail {
    /// DEPRECATED. The account name involved in the transaction, can be "" for the default account.
    pub account: String,
    /// The bitcoin address involved in the transaction.
    pub address: String,
    /// The category, either 'send' or 'receive'.
    pub category: TransactionCategory,
    ///  The amount in BTC.
    pub amount: f64,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// the vout value.
    pub vout: u32,
    /// The amount of the fee.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
}

impl GetTransactionDetail {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransactionDetail, GetTransactionDetailError> {
        use GetTransactionDetailError as E;

        let address = Address::from_str(&self.address).map_err(E::Address)?;
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;

        Ok(model::GetTransactionDetail {
            address,
            category: self.category.into_model(),
            amount,
            label: self.label,
            vout: self.vout,
            fee,
            abandoned: self.abandoned,
        })
    }
}

/// Error when converting to a `v22::GetTransactionDetail` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetTransactionDetailError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for GetTransactionDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionDetailError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

impl std::error::Error for GetTransactionDetailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionDetailError as E;

        match *self {
            E::Address(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

/// Result of the JSON-RPC method `getunconfirmedbalance`.
///
/// > getunconfirmedbalance
/// > Returns the server's total unconfirmed balance
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetUnconfirmedBalance(pub f64); // Core docs are missing so this is just a guess.

impl GetUnconfirmedBalance {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetUnconfirmedBalance, ParseAmountError> {
        let amount = Amount::from_btc(self.amount)?;
        Ok(model::GetUnconfirmedBalance(amount))
    }
}

/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfo {
    /// The wallet name.
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// The wallet version.
    #[serde(rename = "walletversion")]
    pub wallet_version: i64,
    /// The total confirmed balance of the wallet in BTC.
    pub balance: f64,
    /// The total unconfirmed balance of the wallet in BTC.
    pub unconfirmed_balance: f64,
    /// The total immature balance of the wallet in BTC.
    pub immature_balance: f64,
    /// The total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// The timestamp (seconds since Unix epoch) of the oldest pre-generated key in the key pool.
    #[serde(rename = "keypoololdest")]
    pub keypool_oldest: i64,
    /// How many new keys are pre-generated (only counts external keys).
    #[serde(rename = "keypoolsize")]
    pub keypool_size: i64,
    /// How many new keys are pre-generated for internal use (used for change outputs, only appears
    /// if the wallet is using this feature, otherwise external keys are used).
    #[serde(rename = "keypoolsize_hd_internal")]
    pub keypool_size_hd_internal: i64,
    /// The timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked
    /// for transfers, or 0 if the wallet is locked.
    pub unlocked_until: u32,
    /// The transaction fee configuration, set in BTC/kB.
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    /// The Hash160 of the HD seed (only present when HD is enabled).
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// DEPRECATED. Alias for hdseedid retained for backwards-compatibility.
    #[serde(rename = "hdseedid")]
    pub hd_master_key_id: Option<String>,
    /// If privatekeys are disabled for this wallet (enforced watch-only wallet).
    pub private_keys_enabled: bool,
}

impl GetWalletInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
        use GetWalletInfoError as E;

        let balance = self.balance.parse::<Amount>().map_err(E::Balance)?;
        let unconfirmed_balance self.unconfirmed_balance.parse::<Amount>().map_err(E::UnconfirmedBalance)?;
        let immature_balance = self.immature_balance.parse::<Amount>().map_err(E::ImmatureBalance)?;
        let pay_tx_fee = super::btc_per_kb(self.pay_tx_fee);
        let hd_seed_id = self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;

        model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version: self.wallet_version,
            balance,
            unconfirmed_balance,
            immature_balance,
            tx_count: self.tx_count.into(),
            keypool_oldest: self.keypool_oldest.into(),
            keypool_size: self.keypool_size.into(),
            keypool_size_hd_internal: self.keypool_size_hd_internal.into(),
            unlocked_until: self.unlocked_until,
            pay_tx_fee,
            hd_seed_id,
            private_keys_enabled: self.private_keys_enabled,
        }
    }
}

/// Error when converting a `GetWalletInfo` type into the model type.
#[derive(Debug)]
pub enum GetWalletInfoError {
    /// Conversion of the `balance` field failed.
    Balance(ParseAmountError),
    /// Conversion of the `unconfirmed_balance` field failed.
    UnconfirmedBalance(ParseAmountError),
    /// Conversion of the `immature_balance` field failed.
    ImmatureBalance(ParseAmountError),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
}

impl fmt::Display for GetWalletinfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetWalletinfoError::*;

        match *self {
            Balance(ref e) => write_err!(f, "conversion of the `balance` field failed"; e),
            UnconfirmedBalance(ref e) => write_err!(f, "conversion of the `unconfirmed_balance` field failed"; e),
            ImmatureBalance(ref e) => write_err!(f, "conversion of the `immature_balance` field failed"; e),
            HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
        }
    }
}

impl std::error::Error for GetWalletInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetWalletinfoError::*;

        match *self {
            Balance(ref e) => Some(e),
            UnconfirmedBalance(ref e) => Some(e),
            ImmatureBalance(ref e) => Some(e),
            HdSeedId(ref e) => Some(e),
        }
    }
}

/// Result of the JSON-RPC method `listaddressgroupings`.
///
/// > listaddressgroupings
/// >
/// > Lists groups of addresses which have had their common ownership
/// > made public by common use as inputs or as the resulting change
/// > in past transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupings(Vec<Vec<ListAddressGroupingsItem>>);

/// List item type returned as part of `listaddressgroupings`.
// FIXME: The Core docs seem wrong, not sure what shape this should be?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupingsItem {
    /// The bitcoin address.
    pub address: String,
    /// The amount in BTC.
    pub amount: f64,
    /// The label.
    pub label: Option<String>,
}

impl ListAddressGroupingsError {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Foo, ListAddressGroupingsError> {
        use ListAddressGroupingsError as E;

        let address = self.address.parse::<Address<NetworkUnchecked>>().map_err(E::Address)?.assume_checked();
        let amount = self.amount.parse::<Amount>().map_err(E::Amount)?;

        model::ListAddressGroupings { address, amount, label: self.label }
    }
}

/// Error when converting a `ListAddressGroupings` type into the model type.
#[derive(Debug)]
pub enum ListAddressGroupingsError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
}

impl fmt::Display for ListAddressGroupingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListAddressGroupingsError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
        }
    }
}

impl std::error::Error for ListAddressGroupingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListAddressGroupingsError::*;

        match *self {
            Address(ref e) => Some(e),
            Amount(ref e) => Some(e),
        }
    }
}        

/// Result of the JSON-RPC method `listlabels`.
///
/// > listlabels ( "purpose" )
/// >
/// > Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
/// > 
/// > Arguments:
/// > 1. "purpose"    (string, optional) Address purpose to list labels for ('send','receive'). An empty string is the same as not providing this argument.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLabels(Vec<String>);

impl ListLabels {
    pub fn into_model(self) -> model::ListLabels {
        model::ListLabels(self.0)
    }
}

/// Result of the JSON-RPC method `listlockunspent`.
///
/// > listlockunspent
/// >
/// > Returns list of temporarily unspendable outputs.
/// > See the lockunspent call to lock and unlock transactions for spending.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspent(Vec<ListLockUnspentItem>);

/// List item returned as part of of `listlockunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspentItem {
    /// The transaction id locked.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
}  

impl ListLockUnspent {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListLockUnspent, hex::HexToArrayError> {
        let txid = self.txid.parse::<Txid>()?;
        model::ListLockUnspent { txid, vout: vout.into() }
    }
}

/// Result of the JSON-RPC method `listreceivedbyaddress`.
///
/// > listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )
/// >
/// > List balances by receiving address.
/// >
/// > Arguments:
/// > 1. minconf           (numeric, optional, default=1) The minimum number of confirmations before payments are included.
/// > 2. include_empty     (bool, optional, default=false) Whether to include addresses that haven't received any payments.
/// > 3. include_watchonly (bool, optional, default=false) Whether to include watch-only addresses (see 'importaddress').
/// > 4. address_filter    (string, optional) If present, only return information on this address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedbyAddress(Vec<ListReceivedbyAddressItem>);

/// List item returned as part of of `listreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedbyAddressItem {
    /// Only returned if imported addresses were involved in transaction.
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: bool,
    /// The receiving address.
    pub address: String,
    /// DEPRECATED. Backwards compatible alias for label.
    pub account: String,
    /// The total amount in BTC received by the address.
    pub amount: f64,
    /// The number of confirmations of the most recent transaction included.
    pub confirmations: i64,
    /// The label of the receiving address. The default label is "".
    pub label: String,
    /// The ids of transactions received with the address.
    pub txids: Vec<String>,
}

impl ListReceivedByAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByAddress, ListReceivedByAddressError> {
        use ListReceivedByAddressError as E;

        let address = self.address.parse::<Address<NetworkUnchecked>>().map_err(E::Address)?.assume_checked();
        let amount = self.amount.parse::<Amount>().map_err(E::Amount)?;
        let txids = self.txids.iter().map(|txid| txid.parse::<Txid>()).collect::<Result<Vec<_>, _>>().map_err(E::Txids)?;

        Ok(model::ListReceivedByAddress {
            involves_watch_only: self.involves_watch_only,
            address,
            amount,
            confirmations: self.confirmations.into(),
            label: self.label,
            txids,
        })
    }
}

/// Error when converting a `ListReceivedByAddress` type into the model type.
#[derive(Debug)]
pub enum ListReceivedByAddressError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of txid in the `txids` list (with index) failed.
    Txids(usize, hex::HexToArrayError),
}

impl fmt::Display for ListReceivedByAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListReceivedByAddressError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Txids(ref e) => write_err!(f, "conversion of the txid at index {} in the `txids` field failed", self.0; self.1),
        }
    }
}

impl std::error::Error for ListReceivedByAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListReceivedByAddressError::*;

        match *self {
            Address(ref e) => Some(e),
            Amount(ref e) => Some(e),
            Txids(ref e) => Some(e),
        }
    }
}

/// Result of the JSON-RPC method `listsinceblock`.
///
/// > listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )
/// >
/// > Get all transactions in blocks since block `blockhash`, or all transactions if omitted.
/// > If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// > Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
/// >
/// > Arguments:
/// > 1. "blockhash"            (string, optional) The block hash to list transactions since
/// > 2. target_confirmations:    (numeric, optional, default=1) Return the nth block hash from the main chain. e.g. 1 would mean the best block hash. Note: this is not used as a filter, but only affects [lastblock] in the return value
/// > 3. include_watchonly:       (bool, optional, default=false) Include transactions to watch-only addresses (see 'importaddress')
/// > 4. include_removed:         (bool, optional, default=true) Show transactions that were removed due to a reorg in the "removed" array
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListSinceBlock {
    /// All the transactions.
    pub transactions: Vec<ListSinceBlockTransaction>,
    /// Only present if `include_removed=true`.
    ///
    /// Note: transactions that were re-added in the active chain will appear as-is in this array,
    /// and may thus have a positive confirmation count.
    pub removed: Vec<ListSinceBlockTransaction>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain.
    ///
    /// This is typically used to feed back into listsinceblock the next time you call it. So you
    /// would generally use a target_confirmations of say 6, so you will be continually
    /// re-notified of transactions until they've reached 6 confirmations plus any new ones.
    #[serde(rename = "lastblock")]
    pub last_block: String,
}

impl ListSinceBlock {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListSinceBlock, ListSinceBlockError> {
        let transactions = self.transactions.map(|tx| tx.into_model()).collect::<Result<Vec<_>,>>().map_err(E::transactions)?;
        let removed = self.removed.map(|tx| tx.into_model()).collect::<Result<Vec<_>,>>().map_err(E::removed)?;
        let last_block = self.last_block.parse::<BlockHash>().map_err(E::last_block)?;

        Ok(model::ListSinceBlock {
            transactions, removed, last_block
        })
    }
}

/// Error when converting a `ListSinceBlock` type into the model type.
// This is similar but different to the `GetTransactionItem` type.
#[derive(Debug)]
pub enum ListSinceBlockError {
    /// Conversion of item in `transactions` list failed.
    Transactions(ListSinceBlockTransactionError),
    /// Conversion of item in `removed` list failed.
    Removed(ListSinceBlockTransactionError),
    /// Conversion of the `last_block` field failed.
    LastBlock(hex::HexToArrayError),
}

impl fmt::Display for ListSinceBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListSinceBlockError::*;

        match *self {
            Transactions(ref e) => write_err!(f, "conversion of the `transactions` field failed"; e),
            Removed(ref e) => write_err!(f, "conversion of the `removed` field failed"; e),
            LastBlock(ref e) => write_err!(f, "conversion of the `last_block` field failed"; e),
        }
    }
}

impl std::error::Error for ListSinceBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListSinceBlockError::*;

        match *self {
            Transactions(ref e) => Some(e),
            Removed(ref e) => Some(e),
            LastBlock(ref e) => Some(e),
        }
    }
}

/// Transaction item returned as part of `listsinceblock`.
// FIXME: These docs from Core seem to buggy, there is only partial mention of 'move' category?
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListSinceBlockTransaction {
    /// DEPRECATED. The account name associated with the transaction. Will be "" for the default account.
    pub account: String,
    /// The bitcoin address of the transaction.
    ///
    /// Not present for move transactions (category = move).
    pub address: String,
    // FIXME: Maybe there is a 'move' category too?
    /// The transaction category. 'send' has negative amounts, 'receive' has positive amounts.
    pub category: TransactionCategory,
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and for the 'move' category for moves outbound. It
    /// is positive for the 'receive' category, and for the 'move' category for inbound funds.
    pub amount: f64,
    /// The vout value.
    pub vout: i64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: f64,
    /// The number of confirmations for the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions. When it's < 0, it means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The block hash containing the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// The index of the transaction in the block that includes it.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "blockindex")]
    pub block_index: i64,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: u32,
    /// The transaction id.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub txid: Option<String>,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replacable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
}

// TODO: ListSinceBlockTransaction model stuff.

/// Result of the JSON-RPC method `listtransactions`.
///
/// > listtransactions (label count skip include_watchonly)
/// >
/// > If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// >
/// > Returns up to 'count' most recent transactions skipping the first 'from' transactions.
/// > Note that the "account" argument and "otheraccount" return value have been removed in V0.17. To use this RPC with an "account" argument, restart
/// > bitcoind with -deprecatedrpc=accounts
/// >
/// > Arguments:
/// > 1. "label"    (string, optional) If set, should be a valid label name to return only incoming transactions
/// >               with the specified label, or "*" to disable filtering and return all transactions.
/// > 2. count          (numeric, optional, default=10) The number of transactions to return
/// > 3. skip           (numeric, optional, default=0) The number of transactions to skip
/// > 4. include_watchonly (bool, optional, default=false) Include transactions to watch-only addresses (see 'importaddress')
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListTransactions(pub Vec<ListTransactionsItem>);

impl ListTransactions {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListTransactions, ListTransactionsItemError> {
        let transactions = self.0.iter().map(|tx| tx.into_model()).collect::<Result<Vec<_>>, _>()?;
        Ok(model::ListTransactions(transactions))
    }
}

/// Transaction item returned as part of `listtransactions`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListTransactionsItem {
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The transaction category.
    pub category: TransactionCategory, // FIXME: It appears ok to reuse this?
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and is positive for the 'receive' category.
    pub amount: f64,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// The vout value.
    pub vout: i64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: f64,
    /// The number of confirmations for the transaction.
    ///
    /// Negative confirmations indicate the transaction conflicts with the block chain.
    pub confirmations: i64,
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: bool,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: i64,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: u32,
    /// The transaction id.
    pub txid: String,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replacable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
}

// TODO: ListTransactionsItem into_model

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent ( minconf maxconf  ["addresses",...] [include_unsafe] [query_options])
/// >
/// > Returns array of unspent transaction outputs
/// > with between minconf and maxconf (inclusive) confirmations.
/// > Optionally filter to only include txouts paid to specified addresses.
/// >
/// > Arguments:
/// > 1. minconf          (numeric, optional, default=1) The minimum confirmations to filter
/// > 2. maxconf          (numeric, optional, default=9999999) The maximum confirmations to filter
/// > 3. "addresses"      (string) A json array of bitcoin addresses to filter
/// > 4. include_unsafe (bool, optional, default=true) Include outputs that are not safe to spend
/// >                  See description of "safe" attribute below.
/// > 5. query_options    (json, optional) JSON with query options
/// >     {
/// >       "minimumAmount"    (numeric or string, default=0) Minimum value of each UTXO in BTC
/// >       "maximumAmount"    (numeric or string, default=unlimited) Maximum value of each UTXO in BTC
/// >       "maximumCount"     (numeric or string, default=unlimited) Maximum number of UTXOs
/// >       "minimumSumAmount" (numeric or string, default=unlimited) Minimum sum value of all UTXOs in BTC
/// >     }
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListUnspent(Vec<ListUnspentItem>);

/// Unspent transaction output, returned as part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListUnspent {
    /// The transaction id.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The associated label, or "" for the default label.
    pub label: String,
    /// DEPRECATED. The account name associated with the transaction. Will be "" for the default account.
    pub account: String,
    /// The script key.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The number of confirmations.
    pub confirmations: u32,
    /// The redeemScript if scriptPubKey is P2SH.
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    /// Whether we have the private keys to spend this output.
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys.
    pub solvable: bool,
    /// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
    /// and unconfirmed replacement transactions are considered unsafe and are not eligible for
    /// spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
}

// TODO: ListUnspent model stuff.

/// Result of the JSON-RPC method `listwallets`.
///
/// > listwallets
/// > Returns a list of currently loaded wallets.
/// > For full information on the wallet, use "getwalletinfo"
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListWallets(pub Vec<String>);

impl ListWallets {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::ListWallets {
        model::ListWallets(self.0)
    }
}

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename"
/// >
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet (eg -zapwallettxes, upgradewallet, rescan, etc).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The wallet directory or .dat file.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warning: String,
}

impl LoadWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: vec![self.warning] }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

/// Result of the JSON-RPC method `sendtoaddress`.
///
/// > sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")
/// >
/// > Send an amount to a given address.
/// >
/// > Arguments:
/// > 1. "address"            (string, required) The bitcoin address to send to.
/// > 2. "amount"             (numeric or string, required) The amount in BTC to send. eg 0.1
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress(String);

impl SendToAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendToAddress, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendToAddress { txid })
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> { Ok(self.into_model()?.txid) }
}
