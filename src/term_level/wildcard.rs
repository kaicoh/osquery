use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Wildcard {
    field: String,
    value: Value,
}

impl Wildcard {
    pub fn new<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self {
            field: field.into(),
            value: value.into(),
        }
    }
}

impl Serialize for Wildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &Value> = HashMap::new();
        map.insert("value", &self.value);

        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field, &map)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let wc = Wildcard::new("speaker", "H*Y");
        let json = serde_json::to_value(wc).unwrap();

        let expected = serde_json::json!({
            "speaker": {
                "value": "H*Y",
            },
        });

        assert_eq!(json, expected);
    }
}
