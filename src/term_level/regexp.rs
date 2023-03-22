use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Regexp {
    field: String,
    value: Value,
}

impl Regexp {
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

impl Serialize for Regexp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field, &self.value)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let wc = Regexp::new("play_name", "[a-zA-Z]amlet");
        let json = serde_json::to_value(wc).unwrap();

        let expected = serde_json::json!({
            "play_name": "[a-zA-Z]amlet",
        });

        assert_eq!(json, expected);
    }
}
