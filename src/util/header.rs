use http::HeaderMap;
use std::collections::HashMap;

pub fn to_hashmap(header_map: &HeaderMap) -> HashMap<String, String> {
    let mut hash_map = HashMap::with_capacity(header_map.len());

    for (key, value) in header_map.iter() {
        if let Ok(value_str) = value.to_str() {
            hash_map.insert(key.as_str().to_string(), value_str.to_string());
        }
    }

    hash_map
}
