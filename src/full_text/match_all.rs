use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Default, Clone)]
pub struct MatchAll;

impl MatchAll {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serialize for MatchAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = serializer.serialize_map(Some(0))?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = MatchAll::new();
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({});

        assert_eq!(json, expected);
    }
}
