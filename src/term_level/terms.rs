use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone)]
pub struct Terms<T: Serialize> {
    field: String,
    value: Vec<T>,
}

impl<T: Serialize> Terms<T> {
    pub fn new<S, U>(field: S, value: U) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self {
            field: field.into(),
            value: value.into_iter().collect(),
        }
    }
}

impl<T: Serialize> Serialize for Terms<T> {
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
