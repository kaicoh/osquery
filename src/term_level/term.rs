use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Term {
    field: Option<String>,
    value: Option<Value>,
}

impl Term {
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

impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: HashMap<&str, &Value> = HashMap::new();
        map.insert("value", self.value.as_ref().unwrap_or(&Value::Null));

        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(self.field.as_deref().unwrap_or_default(), &map)?;
        term.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = Term::new().field("line_id").value(61809);

        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "line_id": {
                "value": 61809
            }
        });

        assert_eq!(json, expected);
    }
}
