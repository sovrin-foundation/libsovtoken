use serde_json;
use ErrorCode;
use logic::xfer_payload::Extra;

pub type TaaAcceptance = serde_json::Value;

const META_FIELD_NAME: &str = "taaAcceptance";

pub fn extract_taa_acceptance_from_extra(extra: Option<Extra>) -> Result<(Option<Extra>, Option<TaaAcceptance>), ErrorCode> {
    match extra {
        Some(Extra(serde_json::Value::Object(mut extra))) => {
            let meta = extra.remove(META_FIELD_NAME);
            let extra = if extra.is_empty() { None } else { Some(Extra(json!(extra))) };
            Ok((extra, meta))
        }
        Some(extra) => {
            Ok((Some(extra), None))
        }
        None => Ok((None, None))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn extract_taa_acceptance_from_extra_works() {
        let taa_acceptance = json!({
            "mechanism": "acceptance type 1",
            "taaDigest": "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e",
            "time": 123456789,
        });

        let extra = Extra(json!({
            "taaAcceptance": taa_acceptance.clone()
        }));

        let expected_taa = taa_acceptance.clone();

        let (extra, taa_acceptance) = extract_taa_acceptance_from_extra(Some(extra)).unwrap();
        assert_eq!(None, extra);
        assert_eq!(expected_taa, taa_acceptance.unwrap());
    }

    #[test]
    pub fn extract_taa_acceptance_from_extra_works_for_some_extra_data() {
        let taa_acceptance = json!({
            "mechanism": "acceptance type 1",
            "taaDigest": "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e",
            "time": 123456789,
        });

        let extra = Extra(json!({
            "data": "some data",
            "taaAcceptance": taa_acceptance.clone()
        }));

        let expected_extra = Extra(json!({"data": "some data"}));
        let expected_taa = taa_acceptance.clone();

        let (extra, taa_acceptance) = extract_taa_acceptance_from_extra(Some(extra)).unwrap();
        assert_eq!(expected_extra, extra.unwrap());
        assert_eq!(expected_taa, taa_acceptance.unwrap());
    }

    #[test]
    pub fn extract_taa_acceptance_from_extra_works_for_no_taa_acceptance() {
        let extra = Extra(json!({
            "data": "some data",
        }));

        let expected_extra = Extra(json!({"data": "some data"}));

        let (extra, taa_acceptance) = extract_taa_acceptance_from_extra(Some(extra)).unwrap();
        assert_eq!(expected_extra, extra.unwrap());
        assert_eq!(None, taa_acceptance);
    }

    #[test]
    pub fn extract_taa_acceptance_from_extra_works_for_empty() {
        let (extra, taa_acceptance) = extract_taa_acceptance_from_extra(None).unwrap();
        assert_eq!(None, extra);
        assert_eq!(None, taa_acceptance);
    }
}