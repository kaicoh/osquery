use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Exists {
    field: String,
}

impl Exists {
    pub fn new<T: Into<String>>(field: T) -> Self {
        Self {
            field: field.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let exists = Exists::new("speaker");
        let json = serde_json::to_value(exists).unwrap();

        let expected = serde_json::json!({
            "field": "speaker",
        });

        assert_eq!(json, expected);
    }
}
