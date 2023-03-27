use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Prefix {
    field: Option<String>,
    value: Option<Value>,
}

impl Prefix {
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

impl Serialize for Prefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(
            self.field.as_deref().unwrap_or_default(),
            self.value.as_ref().unwrap_or(&Value::Null),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = Prefix::new().field("speaker").value("KING");

        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "speaker": "KING"
        });

        assert_eq!(json, expected);
    }
}
