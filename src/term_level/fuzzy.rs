use crate::options::Rewrite;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Fuzzy {
    field: Option<String>,
    value: FuzzyValues,
}

impl Fuzzy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, val: T) -> Self {
        let value = FuzzyValues {
            value: Some(val.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    /// Sets fuzziness field.
    pub fn fuzziness<S: Into<String>>(self, f: S) -> Self {
        let value = FuzzyValues {
            fuzziness: Some(f.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    /// Sets max_expansions field.
    pub fn max_expansions<S: Into<u64>>(self, m: S) -> Self {
        let value = FuzzyValues {
            max_expansions: Some(m.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    /// Sets prefix_length field.
    pub fn prefix_length<S: Into<u64>>(self, p: S) -> Self {
        let value = FuzzyValues {
            prefix_length: Some(p.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    /// Sets transpositions field.
    pub fn transpositions(self, t: bool) -> Self {
        let value = FuzzyValues {
            transpositions: Some(t),
            ..self.value
        };
        Self { value, ..self }
    }

    /// Sets rewrite field.
    pub fn rewrite<S: Into<Rewrite>>(self, r: S) -> Self {
        let value = FuzzyValues {
            rewrite: Some(r.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
struct FuzzyValues {
    value: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Rewrite>,
}

impl Serialize for Fuzzy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        term.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = Fuzzy::new()
            .field("speaker")
            .value("HALET")
            .fuzziness("2")
            .max_expansions(40 as u64)
            .prefix_length(0 as u64)
            .transpositions(true)
            .rewrite(Rewrite::ConstantScore);

        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "speaker": {
                "value": "HALET",
                "fuzziness": "2",
                "max_expansions": 40,
                "prefix_length": 0,
                "transpositions": true,
                "rewrite": "constant_score"
            }
        });

        assert_eq!(json, expected);
    }
}
