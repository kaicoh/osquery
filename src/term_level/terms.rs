use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Terms {
    field: String,
    value: Vec<Value>,
}

impl Terms {
    pub fn new<F, V, T>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: IntoIterator<Item = T>,
        T: Into<Value>,
    {
        Self {
            field: field.into(),
            value: value.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl Serialize for Terms {
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
        let term = Terms::new("line_id", vec![61809, 61810]);
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "line_id": [61809, 61810]
        });

        assert_eq!(json, expected);
    }
}
