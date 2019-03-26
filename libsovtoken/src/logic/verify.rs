use libc::c_char;

use ErrorCode;
use logic::did::Did;
use utils::constants::general::{JsonCallback, JsonCallbackUnwrapped};
use utils::ffi_support::string_from_char_ptr;
use logic::parsers::common::TXO;

type DeserializedArguments<'a> = (Option<Did<'a>>, TXO, JsonCallbackUnwrapped);

pub fn deserialize<'a>(
    did: *const c_char,
    txo: *const c_char,
    cb: JsonCallback
) -> Result<DeserializedArguments<'a>, ErrorCode> {
    trace!("logic::verify::deserialize >> did: {:?}, txo: {:?}", did, txo);
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;
    trace!("Unwrapped callback.");

    let did = opt_res_to_res_opt!(
        Did::from_pointer(did)
            .map(|did| {
                did.validate()
                    .map_err(map_err_err!())
                    .or(Err(ErrorCode::CommonInvalidStructure))
            })
    )?;

    debug!("Converted did pointer to string >>> {:?}", did);

    let txo = string_from_char_ptr(txo)
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted txo pointer to string >>> {:?}", txo);

    let txo = TXO::from_libindy_string(&txo)
        .map_err(map_err_err!())
        .map_err(|_| ErrorCode::CommonInvalidStructure)?;
    debug!("Deserialized txo: {:?}", txo);

    trace!("logic::verify::deserialize << did: {:?}, txo: {:?}", did, txo);
    Ok((did, txo, cb))
}

#[cfg(test)]
mod test_deserialize {
    use super::*;
    use utils::test::default;
    use logic::parsers::common::TXO;
    use utils::ffi_support::c_pointer_from_str;

    #[test]
    pub fn deserialize_works() {
        let did = c_pointer_from_str("Th7MpTaRZVRYnPiabds81Y");
        let payment_address = "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string();
        let txo = TXO { address: payment_address.clone(), seq_no: 1 }.to_libindy_string().unwrap();
        let txo_c = c_pointer_from_str(&txo);
        let cb = default::empty_callback_string;

        let (did, txo, _) = super::deserialize(did, txo_c, Some(cb)).unwrap();
        assert_eq!(String::from(did.unwrap()), "Th7MpTaRZVRYnPiabds81Y");
        assert_eq!(txo.seq_no, 1);
        assert_eq!(txo.address, payment_address);
    }

    #[test]
    pub fn deserialize_works_for_empty_did() {
        let did = c_pointer_from_str("");
        let payment_address = "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string();
        let txo = TXO { address: payment_address.clone(), seq_no: 1 }.to_libindy_string().unwrap();
        let txo_c = c_pointer_from_str(&txo);
        let cb = default::empty_callback_string;

        let ec = super::deserialize(did, txo_c, Some(cb)).unwrap_err();
        assert_eq!(ec, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    pub fn deserialize_works_for_invalid_txo() {
        let did = c_pointer_from_str("Th7MpTaRZVRYnPiabds81Y");
        let txo_c = c_pointer_from_str("");
        let cb = default::empty_callback_string;

        let ec = super::deserialize(did, txo_c, Some(cb)).unwrap_err();
        assert_eq!(ec, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    pub fn deserialize_works_for_null_cb() {
        let did = c_pointer_from_str("");
        let payment_address = "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC".to_string();
        let txo = TXO { address: payment_address.clone(), seq_no: 1 }.to_libindy_string().unwrap();
        let txo_c = c_pointer_from_str(&txo);

        let ec = super::deserialize(did, txo_c, None).unwrap_err();
        assert_eq!(ec, ErrorCode::CommonInvalidStructure);
    }
}