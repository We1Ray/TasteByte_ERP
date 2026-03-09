use serde_json::Value;
use std::collections::BTreeMap;

/// Compute a deep diff between two JSON values.
/// Returns a JSON object with "added", "removed", and "changed" keys.
pub fn json_diff(old: &Value, new: &Value) -> Value {
    let mut added = BTreeMap::new();
    let mut removed = BTreeMap::new();
    let mut changed = BTreeMap::new();

    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            for (k, v) in old_map {
                if let Some(new_v) = new_map.get(k) {
                    if v != new_v {
                        changed.insert(k.clone(), serde_json::json!({ "from": v, "to": new_v }));
                    }
                } else {
                    removed.insert(k.clone(), v.clone());
                }
            }
            for (k, v) in new_map {
                if !old_map.contains_key(k) {
                    added.insert(k.clone(), v.clone());
                }
            }
        }
        _ => {
            if old != new {
                changed.insert(
                    "_root".to_string(),
                    serde_json::json!({ "from": old, "to": new }),
                );
            }
        }
    }

    serde_json::json!({
        "added": added,
        "removed": removed,
        "changed": changed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn detect_added_keys() {
        let old = json!({"a": 1});
        let new = json!({"a": 1, "b": 2});
        let diff = json_diff(&old, &new);
        assert_eq!(diff["added"]["b"], json!(2));
        assert!(diff["removed"].as_object().unwrap().is_empty());
        assert!(diff["changed"].as_object().unwrap().is_empty());
    }

    #[test]
    fn detect_removed_keys() {
        let old = json!({"a": 1, "b": 2});
        let new = json!({"a": 1});
        let diff = json_diff(&old, &new);
        assert_eq!(diff["removed"]["b"], json!(2));
        assert!(diff["added"].as_object().unwrap().is_empty());
    }

    #[test]
    fn detect_changed_values() {
        let old = json!({"a": 1, "b": "hello"});
        let new = json!({"a": 1, "b": "world"});
        let diff = json_diff(&old, &new);
        assert_eq!(diff["changed"]["b"]["from"], json!("hello"));
        assert_eq!(diff["changed"]["b"]["to"], json!("world"));
        assert!(diff["added"].as_object().unwrap().is_empty());
        assert!(diff["removed"].as_object().unwrap().is_empty());
    }
}
