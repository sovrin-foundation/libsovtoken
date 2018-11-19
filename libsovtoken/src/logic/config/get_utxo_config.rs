/*!
 *  Defines structure and implementation of getUtxoRequest
 *  these are the structures for the [`build_get_utxo_txn_handler`]
 *
 *  [`build_get_utxo_txn_handler`]: ../../../api/fn.build_utxo_txn_handler.html
 */

use logic::address::strip_qualifier_from_address;
use logic::request::Request;
use utils::constants::txn_types::GET_UTXO;
use logic::address::verkey_from_unqualified_address;

/**
 *  Json config to customize [`build_get_utxo_txn_handler`]
 *
 *  [`build_get_utxo_txn_handler`]: ../../../api/fn.build_get_utxo_txn_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetUtxoOperationRequest {
    address : String,
    #[serde(rename = "type")]
    req_type: String
}

impl GetUtxoOperationRequest {
    pub fn new(address : String) -> Request<GetUtxoOperationRequest> {
        let unqualified_address: String = strip_qualifier_from_address(&address);
        let identifier = verkey_from_unqualified_address(&unqualified_address).ok();
        let req = GetUtxoOperationRequest {
            address : unqualified_address,
            req_type : GET_UTXO.to_string(),
        };
        return Request::new(req, identifier);
    }
}


#[cfg(test)]
mod get_utxo_config_tests {

    use logic::address::{qualified_address_from_verkey, verkey_from_unqualified_address};
    use utils::logger::init_log;
    use super::*;

    // This test ensures TOK-239 is fixed
    #[test]
    fn address_correct_removes_sovrin_id() {

        init_log();

        let ver_key: String = "EFfodscoymgdJDuM885uEWmgCcA25P6VR6TjVqsYZLW3".to_string();
        let payment_address: String = qualified_address_from_verkey(&ver_key).unwrap();

        let utxo_request = GetUtxoOperationRequest::new(String::from(payment_address));

        trace!("utxo_request => {:?}", utxo_request);

        assert_eq!(ver_key, verkey_from_unqualified_address(&utxo_request.operation.address).unwrap());
    }
}