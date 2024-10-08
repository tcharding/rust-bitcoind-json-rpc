// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `addmultisigaddress`.
#[macro_export]
macro_rules! impl_client_v17__addmultisigaddress {
    () => {
        impl Client {
            pub fn add_multisig_address_with_keys(&self, nrequired: u32, keys: Vec<PublicKey>) -> Result<Addmultisigaddress> {
                self.call("addmultisigaddress", &[nrequired.into(), keys.into_json()?])
            }

            pub fn add_multisig_address_with_addresses(&self, nrequired: u32, keys: Vec<Address>) -> Result<Addmultisigaddress> {
                self.call("addmultisigaddress", &[nrequired.into(), keys.into_json()?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `bumpfee`.
#[macro_export]
macro_rules! impl_client_v17__bumpfee {
    () => {
        impl Client {
            pub fn bump_fee(&self, txid: Txid) -> Result<BumpFee> {
                self.call("bumpfee", &[txid.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `createwallet`
#[macro_export]
macro_rules! impl_client_v17__createwallet {
    () => {
        impl Client {
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `dumpprivkey`.
#[macro_export]
macro_rules! impl_client_v17__dumpprivkey {
    () => {
        impl Client {
            pub fn dump_priv_key(&self, address: &Address) -> Result<DumpPrivKey> {
                self.call("dumpprivkey", &[address.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `dumpwallet`.
#[macro_export]
macro_rules! impl_client_v17__dumpwallet {
    () => {
        impl Client {
            // filename is either absolute or relative to bitcoind.
            pub fn dump_wallet(&self, filename: &Path) -> Result<DumpWallet> {
                self.call("dumpwallet", &[filename.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getaddressesbylabel`
#[macro_export]
macro_rules! impl_client_v17__getaddressesbylabel {
    () => {
        impl Client {
            pub fn get_addresses_by_label(&self, label: &str) -> Result<GetAddressesByLabel> {
                self.call("getaddressesbylabel", &[label.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getaddressinfo`
#[macro_export]
macro_rules! impl_client_v17__getaddressinfo {
    () => {
        impl Client {
            pub fn get_address_info(&self, address: &Address) -> Result<GetAddressInfo> {
                self.call("getaddressinfo", &[address.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `unloadwallet`
#[macro_export]
macro_rules! impl_client_v17__unloadwallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet: &str) -> Result<()> {
                self.call("unloadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `loadwallet`
#[macro_export]
macro_rules! impl_client_v17__loadwallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, wallet: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getbalance`
#[macro_export]
macro_rules! impl_client_v17__getbalance {
    () => {
        impl Client {
            pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getnewaddress`
#[macro_export]
macro_rules! impl_client_v17__getnewaddress {
    () => {
        impl Client {
            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address(&self) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address()?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an invalid address for the network its on.
                Ok(address)
            }

            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address_with_type(&self, ty: AddressType) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address_with_type(ty)?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an invalid address for the network its on.
                Ok(address)
            }

            pub fn get_new_address(&self) -> Result<GetNewAddress> {
                self.call("getnewaddress", &[])
            }

            pub fn get_new_address_with_type(&self, ty: AddressType) -> Result<GetNewAddress> {
                self.call("getnewaddress", &["".into(), into_json(ty)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `sendtoaddress`
#[macro_export]
macro_rules! impl_client_v17__sendtoaddress {
    () => {
        impl Client {
            pub fn send_to_address(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<SendToAddress> {
                let mut args = [address.to_string().into(), into_json(amount.to_btc())?];
                self.call("sendtoaddress", handle_defaults(&mut args, &["".into(), "".into()]))
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `gettransaction`
#[macro_export]
macro_rules! impl_client_v17__gettransaction {
    () => {
        impl Client {
            pub fn get_transaction(&self, txid: Txid) -> Result<GetTransaction> {
                self.call("gettransaction", &[into_json(txid)?])
            }
        }
    };
}
