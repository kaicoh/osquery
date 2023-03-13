use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Fuzzy<T: Serialize> {
    field: String,
    value: T,
}

impl<T: Serialize> Fuzzy<T> {
    pub fn new<S: Into<String>>(field: S, value: T) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl<T: Serialize> Serialize for Fuzzy<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &T> = HashMap::new();
        map.insert("value", &self.value);

        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(&self.field, &map)?;
        term.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = Fuzzy::new("speaker", "HALET");
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "speaker": {
                "value": "HALET",
            }
        });

        assert_eq!(json, expected);
    }
}
