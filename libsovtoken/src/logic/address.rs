/*!
    Methods for dealing with addresses, pub keys and private keys.

    ## Terms
    ### qualifier
    specifies which payment handler the address belongs too.
    e.g. `"pay:sov:"`

    ### unqualified address
    `<verkey><checksum>`
    e.g. `"WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2"`

    ### qualified address
    `<qualifier><verkey><checksum>`
    e.g. `"pay:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2"`

*/

use serde_json;
use std::{io, str};

use ErrorCode;
use logic::parsers::common::TXO;
use utils::json_conversion::{JsonDeserialize, JsonSerialize};
use utils::constants::general::{PAYMENT_ADDRESS_QUALIFIER, TXO_QUALIFIER};
use utils::base58::{IntoBase58, FromBase58};

// Following lengths are in bytes
pub const VERKEY_LEN: usize = 32;

// ASSUMPTION: Qualifier is always considered as ASCII, when this assumption becomes false,
// the following logic will break since byte size changes.
// TODO: It is better to have a lazy_static
pub const ADDRESS_QUAL_LEN: usize = 8;

pub const ADDRESS_CHECKSUM_LEN: usize = 4;

pub const ADDRESS_LEN: usize = VERKEY_LEN + ADDRESS_CHECKSUM_LEN + ADDRESS_QUAL_LEN;

/**
    Removes the "pay:sov:" from an address.
    Leaves the verkey with the checksum.

    ```
    use sovtoken::logic::address::unqualified_address_from_address;
    let address = String::from("pay:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2");
    let verkey_checksum = unqualified_address_from_address(&address).unwrap();
    assert_eq!(verkey_checksum, String::from("WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2"));
    ```
*/
pub fn unqualified_address_from_address(fq_address: &str) -> Result<String, ErrorCode> {
    validate_address(&fq_address)?;
    return Ok(strip_qualifier_from_address(&fq_address));
}

/** 
    Adds a checksum to a verkey.

    Returns an unqualified address <verkey><checksum>
    ```
    use sovtoken::logic::address::unqualified_address_from_verkey;
    let verkey = "EFfodscoymgdJDuM885uEWmgCcA25P6VR6TjVqsYZLW3";
    let address = unqualified_address_from_verkey(verkey).unwrap();

    let expected_address = String::from("2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt");
    assert_eq!(expected_address, address);
    ```
*/
pub fn unqualified_address_from_verkey(verkey: &str) -> Result<String, ErrorCode> {
    let vk_bytes = verkey.from_base58()
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    if vk_bytes.len() != VERKEY_LEN {
        return Err(ErrorCode::CommonInvalidStructure);
    }
    Ok(vk_bytes.into_base58_check())
}

/**
    Form a qualified address from a unqualified one.

    ```
    use sovtoken::logic::address::address_from_unqualified_address;
    let verkey = "2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt";
    let address = address_from_unqualified_address(verkey).unwrap();

    let expected_address = String::from("pay:sov:2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt");
    assert_eq!(expected_address, address);
    ```
*/
pub fn address_from_unqualified_address(unqual_address: &str) -> Result<String, ErrorCode> {
    verkey_from_unqualified_address(unqual_address)?;
    Ok(format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, unqual_address))
}

/**
    Form a qualified address from a verkey.

    ```
    use sovtoken::logic::address::qualified_address_from_verkey;
    let verkey = "EFfodscoymgdJDuM885uEWmgCcA25P6VR6TjVqsYZLW3";
    let address = qualified_address_from_verkey(verkey).unwrap();

    let expected_address = String::from("pay:sov:2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt");
    assert_eq!(expected_address, address);
    ```
*/
pub fn qualified_address_from_verkey(verkey: &str) -> Result<String, ErrorCode> {
    let address = unqualified_address_from_verkey(verkey)?;
    return Ok(format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, address));
}


/**
    `validate_address` checks that a qualified address is formatted
    as `pay:sov:<verkey><checksum>` and the verkey is valid. Returns
    the verkey.
   
    ```
    use sovtoken::logic::address::validate_address;
    let address = String::from("pay:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2");
    let verkey = validate_address(&address).unwrap();
    assert_eq!(verkey, String::from("5ZTeJT5ykaWmZErwkM6qdF3RYN7gVXRTmVn4QdpzZ7BJ"));
    ```
*/


pub fn validate_address(fully_qualified_address: &str) -> Result<String, ErrorCode> {
    if !fully_qualified_address.starts_with(&PAYMENT_ADDRESS_QUALIFIER) {
        error!("Payment address should start with a correct qualifier {}", PAYMENT_ADDRESS_QUALIFIER);
        return Err(ErrorCode::CommonInvalidStructure);
    }

    let address = strip_qualifier_from_address(&fully_qualified_address);
    verkey_from_unqualified_address(&address)
}

pub fn verkey_from_unqualified_address(unqualified_address: &str) -> Result<String, ErrorCode> {
    match unqualified_address.from_base58_check().map_err(map_err_err!()) {
        Ok(vk) => {
            if vk.len() != VERKEY_LEN {
                error!("Incorrect verkey length, expected {:?}, real {:?}", VERKEY_LEN, vk.len());
                return Err(ErrorCode::CommonInvalidStructure)
            } else {
                return Ok(vk.into_base58());
            }
        },
        Err(_) => return Err(ErrorCode::CommonInvalidStructure)
    }
}

/**
    `string_to_txo` checks that the string is formatted as `txo:sov:<base58-encoded json>` and parses it to TXO struct.
    Returns TXO.

    ```
    use sovtoken::logic::parsers::common::TXO;
    use sovtoken::logic::address::string_to_txo;
    let txo_str = "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgbtS5cegZzNp7WmMSpWL";
    let result_txo = string_to_txo(txo_str).unwrap();
    assert_eq!(TXO {address:"pay:sov:iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd".to_string(), seq_no: 1}, result_txo);
    ```
*/
pub fn string_to_txo(txo_str: &str) -> Result<TXO, serde_json::Error> {
    if !txo_str.starts_with(TXO_QUALIFIER) {
        return Err(serde_json::Error::io(io::ErrorKind::InvalidInput.into()));
    }
    let json_u8 = (&txo_str[TXO_QUALIFIER.len()..]).from_base58_check()
        .map_err(|_| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
    let json = str::from_utf8(&json_u8)
        .map_err(|_| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
    TXO::from_json(json)
}

/**
    `txo_to_string` serialize TXO to json and encodes it with base58. After that string is prepended with `txo:sov` prefix.
    Return String.

    ```
    use sovtoken::logic::parsers::common::TXO;
    use sovtoken::logic::address::txo_to_string;
    let result_txo_str = "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgbtS5cegZzNp7WmMSpWL";
    let txo = TXO {address:"pay:sov:iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd".to_string(), seq_no: 1};
    assert_eq!(txo_to_string(&txo).unwrap(), result_txo_str);
    ```
*/
pub fn txo_to_string(txo: &TXO) ->  Result<String, ErrorCode> {
    let temp = txo.to_json()
        .map_err(|_| ErrorCode::CommonInvalidState)?
        .as_bytes().into_base58_check();
    Ok(TXO_QUALIFIER.to_string() + &temp)
}

/**
    takes an "address" and returns "pay:sov" plus address.
    there is no validation that the address is valid

    ```
    use sovtoken::logic::address::add_qualifer_to_address;
    let address = String::from("WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2");
    let qualifed_address = add_qualifer_to_address(&address);
    assert_eq!(qualifed_address, String::from("pay:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2"));
    ```

*/
pub fn add_qualifer_to_address(address : &str) -> String {
    return format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, address);
}

/**
    Takes a fully qualified address and returns the unqualified address.
    Unqualified address is <verkey><checksum> without the "pay:sov"
*/
pub fn strip_qualifier_from_address(address : &str) -> String {
    return address[ADDRESS_QUAL_LEN..].to_string();
}

/*
    Methods "private" (aka not exported from this module)

    KEEP all public methods above
*/



#[cfg(test)]
pub mod address_tests {
    use utils::random::rand_bytes;
    use utils::constants::general::PAYMENT_ADDRESS_QUALIFIER;

    use super::*;

    fn validate_address_invalid_verkey_len(length: usize) {
        assert_ne!(length, VERKEY_LEN);
        let vk_bytes = rand_bytes(length);
        let verkey = vk_bytes.into_base58();
        let invalid_address = qualified_address_from_verkey(&verkey);
        assert!(invalid_address.is_err())
    }

    pub fn gen_random_base58_verkey() -> String {
        let vk_bytes = rand_bytes(VERKEY_LEN);
        vk_bytes.into_base58()
    }

    pub fn gen_random_base58_address() -> String {
        let verkey = gen_random_base58_verkey();
        unqualified_address_from_verkey(&verkey).unwrap()
    }

    fn gen_random_qualified_address() -> String {
        qualified_address_from_verkey(&gen_random_base58_verkey()).unwrap()
    }

    fn replace_char_at(s: &str, idx: usize, c: char) -> String {
        // Taken from https://stackoverflow.com/a/27320653
        let mut r = String::with_capacity(s.len());
        for (i, d) in s.char_indices() {
            r.push(if i == idx { c } else { d });
        }
        r
    }

    #[test]
    fn test_unqualified_address_from_verkey_success() {
        let vk_bytes = rand_bytes(VERKEY_LEN);
        let verkey = vk_bytes.into_base58();
        let address = unqualified_address_from_verkey(&verkey).unwrap();
        let address_bytes = address.from_base58().unwrap();

        assert!(address.len() > verkey.len());
        assert_eq!(address_bytes.len() - vk_bytes.len(), ADDRESS_CHECKSUM_LEN);
    }

    #[test]
    fn test_verkey_invalid_address_length_long_and_short() {
        validate_address_invalid_verkey_len(30);
        validate_address_invalid_verkey_len(40);
        validate_address_invalid_verkey_len(55);
    }

    #[test]
    fn test_address_invalid_qualifier() {
        let address = gen_random_base58_address();
        let invalid_address = format!("pat:sov:{}", address);
        let result = validate_address(&invalid_address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_verkey_from_qualified_address() {
        let verkey = gen_random_base58_verkey();
        let address = qualified_address_from_verkey(&verkey).unwrap();
        let result = validate_address(&address);
        let verkey_extracted = result.unwrap();
        assert_eq!(verkey_extracted, verkey);
    }

    #[test]
    fn test_qualified_address_invalid_length_verkey() {
        let vk_bytes = rand_bytes(VERKEY_LEN+1);
        let address = vk_bytes.into_base58_check();
        let result = validate_address(&address);
        let error = result.unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_strip_qualifier() {
        let address = gen_random_base58_address();
        let valid_fq_address = format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, address);
        assert_eq!(strip_qualifier_from_address(&valid_fq_address), address);
    }

    #[test]
    fn test_invalid_checksum_in_address() {
        let verkey = gen_random_base58_verkey();
        let address = unqualified_address_from_verkey(&verkey).unwrap();
        let addr_len = address.len();
        // Mess up the checksum
        let mut bad_addr = replace_char_at(&address, addr_len-1, 'a');
        bad_addr = replace_char_at(&bad_addr, addr_len-2, 'b');

        let fq_address = format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, bad_addr);
        let error = validate_address(&fq_address).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_unqualified_address_invalid_qualifier() {
        let address = gen_random_base58_address();
        let invalid_address = format!("pat:sov:{}", address);
        let error = unqualified_address_from_address(&invalid_address).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn test_success_validate_qualified_address_from_verkey() {
        let address = gen_random_qualified_address();

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let qualifer = &address[0..ADDRESS_QUAL_LEN];
        let result_address = &address[ADDRESS_QUAL_LEN..];

        assert_eq!(PAYMENT_ADDRESS_QUALIFIER, qualifer, "PAYMENT_ADDRESS_QUALIFIER not found");
        assert_eq!(VERKEY_LEN + ADDRESS_CHECKSUM_LEN, result_address.from_base58().unwrap().len(), "address is not 36 bytes");
        assert_eq!(VERKEY_LEN, result_address.from_base58_check().unwrap().len(), "verkey is not 32 bytes");
    }

    #[test]
    fn test_to_and_fro() {
        let addresses = vec![
            "2Viu9qrpqM48PSw3vdoQoFKP5AvYTChUZhwWtCydfW9iu7ftRt",
            "C1iM7fr4cT32J3DuwKDQDPLbNhN7NaEk9ex2ictk86Lg1ZKC9",
            "zivqx63btpvxCM2Aj7hqVMBkbB84v7aJ5xDC6MNQj7MSPFJN1",
            "28dLM4uKiPa2cyLuUsEpKDa8HyvcTMTmg6ji5X23eLA8jZCJAv",
            "TKe9eXtchV71J2qXX5HwP8rbkTBStnEEkMwQkHie265VtRSbs"
        ];
        let verkeys = vec![
            "EFfodscoymgdJDuM885uEWmgCcA25P6VR6TjVqsYZLW3",
            "2gcGb3qbTGNc5zkdcBq9Kq4nQutptt7ofoFVRTmxAnJc",
            "9pdZM4dWas2WsQkiD1H57yT8qwME6T38fS2M6AwmDR2v",
            "B2gfDbd9EBh7Acs3x3cqgWebTApqZvuSKKhSocKzM4Cq",
            "52JU5iD4ryAkjpYLb58qwY48sGQZGYq3gQs1uqY3o1oz"
        ];
    
        for i in 0..5 {
            let a = unqualified_address_from_verkey(verkeys[i]).unwrap();
            assert_eq!(&a, &addresses[i]);
            let fa = format!("{}{}", PAYMENT_ADDRESS_QUALIFIER, &addresses[i]);
            assert_eq!(validate_address(&fa).unwrap(), verkeys[i])
        }
    }
}
