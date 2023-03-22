use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Term {
    field: String,
    value: Value,
}

impl Term {
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

impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &Value> = HashMap::new();
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
        let term = Term::new("line_id", 61809);
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "line_id": {
                "value": 61809
            }
        });

        assert_eq!(json, expected);
    }
}
