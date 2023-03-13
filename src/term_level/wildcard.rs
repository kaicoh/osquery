use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Wildcard<T: Serialize> {
    field: String,
    value: T,
}

impl<T: Serialize> Wildcard<T> {
    pub fn new<S: Into<String>>(field: S, value: T) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl<T: Serialize> Serialize for Wildcard<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &T> = HashMap::new();
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
