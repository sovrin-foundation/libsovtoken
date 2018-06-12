//! Contains constants for transaction types


/**
    # description
    This is the transaction type used for Minting
*/
pub const MINT_PUBLIC : &'static str = "10000";

/**
    #description
    A transaction type for transferring tokens from one address to a different address
*/

pub const XFER_PUBLIC: &'static str = "10001";

/**
    # description
    This is the transaction type for getting a list of UTXOs associated with an address
*/
pub const GET_UTXO: &'static str = "10002";

/**
    #description
    A transaction type submitted by Sovrin Trustees to set the Fees to process a transaction
*/

pub const SET_FEES: &'static str = "20000";

/**
    #description
    A transaction type submitted by anyone to get the current Fees costs of every transaction
*/

pub const GET_FEES: &'static str = "20000";