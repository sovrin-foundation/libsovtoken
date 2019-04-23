//! Contains constants for transaction types


// TODO: Make them part of an Enum

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

pub const NYM: &'static str = "1";

pub const ATTRIB: &'static str = "100";

pub const GET_ATTRIB: &'static str = "104";


/**
    #description
    A transaction type submitted by anyone to get the current Ledger authentication rules (including fees costs of every transaction).
*/

pub const GET_AUTH_RULE: &'static str = "121";
