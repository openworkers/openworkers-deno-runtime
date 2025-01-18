use std::collections::HashMap;

use deno_core::serde_json;

pub(crate) trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl ToJsonString for HashMap<String, String> {
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or("undefined".to_string())
    }
}

impl ToJsonString for Option<HashMap<String, String>> {
    fn to_json_string(&self) -> String {
        match self {
            Some(map) => map.to_json_string(),
            None => "undefined".to_string(),
        }
    }
}
