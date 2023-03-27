use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Exists {
    field: Option<String>,
}

impl Exists {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let exists = Exists::new().field("speaker");

        let json = serde_json::to_value(exists).unwrap();

        let expected = serde_json::json!({
            "field": "speaker",
        });

        assert_eq!(json, expected);
    }
}
