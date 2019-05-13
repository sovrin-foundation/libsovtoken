use serde_json;
use ErrorCode;

pub type TaaAcceptance = serde_json::Value;

const META_FIELD_NAME: &str = "taaAcceptance";

pub fn extract_taa_acceptance_from_extra(extra: Option<serde_json::Value>) -> Result<(Option<serde_json::Value>, Option<TaaAcceptance>), ErrorCode> {
    match extra {
        Some(serde_json::Value::Object(mut extra)) => {
            let meta = extra.remove(META_FIELD_NAME);
            let extra = if extra.is_empty() { None } else { Some(json!(extra)) };
            let meta = meta.map(|meta_| json!({META_FIELD_NAME: meta_}));
            Ok((extra, meta))
        }
        Some(extra) => {
            Ok((Some(extra), None))
        }
        None => Ok((None, None))
    }
}