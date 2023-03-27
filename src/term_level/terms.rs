use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Terms {
    field: Option<String>,
    values: Vec<Value>,
}

impl Terms {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn values<V, T>(self, values: V) -> Self
    where
        V: IntoIterator<Item = T>,
        T: Into<Value>,
    {
        Self {
            values: values.into_iter().map(|v| v.into()).collect(),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        let mut values = self.values;
        values.push(value.into());

        Self { values, ..self }
    }
}

impl Serialize for Terms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.values)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let terms = Terms::new().field("line_id").values(vec![61809, 61810]);
        let json = serde_json::to_value(terms).unwrap();

        let expected = serde_json::json!({
            "line_id": [61809, 61810]
        });

        assert_eq!(json, expected);
    }
}
