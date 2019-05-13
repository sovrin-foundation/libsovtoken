use serde_json;
use ErrorCode;

pub type TaaAcceptance = serde_json::Value;

const META_FIELD_NAME: &str = "taaAcceptance";

pub fn extract_taa_acceptance_from_extra(extra: Option<String>) -> Result<(Option<String>, Option<TaaAcceptance>), ErrorCode> {
    match extra {
        Some(extra_) => {
            let extra = serde_json::from_str(&extra_).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?;

            match extra {
                Some(serde_json::Value::Object(mut extra)) => {
                    let meta = extra.remove(META_FIELD_NAME);
                    let extra_json =
                        if extra.is_empty() {
                            None
                        } else {
                            Some(serde_json::to_string(&extra).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?)
                        };
                    let meta = meta.map(|meta_| json!({META_FIELD_NAME: meta_}));
                    Ok((extra_json, meta))
                }
                Some(extra) => {
                    let extra_json = serde_json::to_string(&extra).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?;
                    Ok((Some(extra_json), None))
                }
                None => Ok((None, None))
            }
        }
        None => Ok((None, None))
    }
}