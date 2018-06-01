//! Contains constants for transaction types


/**
    # description
    This is the transaction type used for Minting
*/
const MINT_PUBLIC : String = "10000";

/**
    #description
    A transaction type for transferring tokens from one address to a different address
*/

const XFER_PUBLIC: String = "10001";

/**
    # description
    This is the transaction type for getting a list of UTXOs associated with an address
*/
const GET_UTXO: String = "10002";

/**
    #description
    A transaction type submitted by Sovrin Trustees to set the Fees to process a transaction
*/

const SET_FEES: String = "20000";

/**
    #description
    A transaction type submitted by anyone to get the current Fees costs of every transaction
*/

const GET_FEES: String = "20000";