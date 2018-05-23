use logic::address::get_address_chopped;
use std::collections::HashMap;
use std::sync::Mutex;
use utils::types::*;


lazy_static! {
    static ref UTXOS: Mutex<HashMap<String, Vec<String>>> = Default::default();
    static ref BALANCES: Mutex<HashMap<String, i32>> = Default::default();
    static ref TXNS: Mutex<HashMap<i32, (Vec<String>, Vec<UTXOOutput>)>> = Default::default();

}
/**
    to_utxo will grab a payment address jayson in the form of a &str
    then it break it up into it's pieces by using `get_address_chopped`.
    The parts of this address pay:sov:<address><checksum>. The UTXO then
    is constructed in a vector using the parts we broke up but in this
    vector we add a sequence number to our address. Then we return this
    with the original colons.
*/
pub fn to_utxo (payment_address: &str, seq_no: i32) -> Option<String> {

    let address_chopped  = get_address_chopped(payment_address, true).unwrap();
    let utxo = vec![address_chopped.get(0).unwrap().to_string(),
                    address_chopped.get(1).unwrap().to_string(),
                    seq_no.to_string() + "_" + address_chopped.get(2).unwrap()];
    Some(utxo.join(":"))
}

/**
    `from_utxo` will take in a utxo &str and then revert the process of `to_utxo`
*/
pub fn from_utxo (utxo: &str) -> Option<(i32, String)>{
    let utxo_chopped = get_address_chopped(utxo, true).unwrap();
    let address_seq = utxo_chopped.get(2).unwrap();
    let address_seq_split : Vec<&str>= address_seq.split("_").collect();
    let seq_no = match address_seq_split.get(0).unwrap().to_string().parse() {
        Ok(v) => v,
        Err(_) => return None
    };
    let address = vec![utxo_chopped.get(0).unwrap().to_string(), utxo_chopped.get(1).unwrap().to_string(), address_seq_split.get(1).unwrap().to_string()];
    Some((seq_no, address.join(":")))
}

/**
   `get_utxos_by_payment_address` will take in a payment address and then
   using the match and vectors method `get` to build a vec<string>. From
   libnullpay.
*/
pub fn get_utxos_by_payment_address(payment_address: String) -> Vec<String> {
    let utxos = UTXOS.lock().unwrap();
    match utxos.get(&payment_address) {
        Some(v) => v.clone(),
        None => Vec::new()
    }
}


/**

*/
pub fn get_txn(seq_no: i32) -> Option<(Vec<String>, Vec<UTXOOutput>)> {
    let txns = TXNS.lock().unwrap();
    txns.get(&seq_no).map(|&(ref a, ref b)| (a.clone(), b.clone()))
}

pub fn get_utxo_info(utxo: String) -> Option<UTXOInfo> {
    let (seq_no, payment_address) = match from_utxo(utxo.as_str()) {
        Some(e) => e,
        None => return None
    };

    match get_txn(seq_no).map(|(_, outputs)| {
        outputs.into_iter().find(|out| out.payment_address == payment_address).map(|out| {
            UTXOInfo {
                input: utxo,
                amount: out.amount,
                extra: out.extra,
            }
        })
    }) {
        Some(Some(o)) => Some(o),
        _ => None
    }
}


#[cfg(test)]
mod utx_test {
    use super::*;
    #[test]
    fn testing_string (){
        assert_eq!(String::from("hello"),to_utxo("pay:sov:01234567890123456789012345678901XXXX", 89).unwrap())
    }
}