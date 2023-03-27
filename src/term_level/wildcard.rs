use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Wildcard {
    field: Option<String>,
    value: Option<Value>,
}

impl Wildcard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        Self {
            value: Some(value.into()),
            ..self
        }
    }
}

impl Serialize for Wildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &Value> = HashMap::new();
        map.insert("value", self.value.as_ref().unwrap_or(&Value::Null));

        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &map)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let wc = Wildcard::new().field("speaker").value("H*Y");

        let json = serde_json::to_value(wc).unwrap();

        let expected = serde_json::json!({
            "speaker": {
                "value": "H*Y",
            },
        });

        assert_eq!(json, expected);
    }
}
