use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone)]
pub struct Regexp<T: Serialize> {
    field: String,
    value: T,
}

impl<T: Serialize> Regexp<T> {
    pub fn new<S: Into<String>>(field: S, value: T) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl<T: Serialize> Serialize for Regexp<T> {
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
