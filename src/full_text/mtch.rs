use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Match {
    field: String,
    value: Value,
}

impl Match {
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

impl Serialize for Match {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(&self.field, &self.value)?;
        term.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = Match::new("title", "wind");
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "title": "wind"
        });

        assert_eq!(json, expected);
    }
}
