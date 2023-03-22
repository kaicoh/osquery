use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TermFuzzy {
    field: String,
    value: Fuzzy,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Fuzzy {
    value: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<String>,
}

impl Fuzzy {
    /// Create fuzzy parameter.
    pub fn new<S: Into<Value>>(value: S) -> Self {
        Self {
            value: value.into(),
            fuzziness: None,
            max_expansions: None,
            prefix_length: None,
            transpositions: None,
            rewrite: None,
        }
    }

    /// Sets fuzziness field.
    pub fn fuzziness<S: Into<String>>(self, f: S) -> Self {
        Self {
            fuzziness: Some(f.into()),
            ..self
        }
    }

    /// Sets max_expansions field.
    pub fn max_expansions<S: Into<u64>>(self, m: S) -> Self {
        Self {
            max_expansions: Some(m.into()),
            ..self
        }
    }

    /// Sets prefix_length field.
    pub fn prefix_length<S: Into<u64>>(self, p: S) -> Self {
        Self {
            prefix_length: Some(p.into()),
            ..self
        }
    }

    /// Sets transpositions field.
    pub fn transpositions(self, t: bool) -> Self {
        Self {
            transpositions: Some(t),
            ..self
        }
    }

    /// Sets rewrite field.
    pub fn rewrite<S: Into<String>>(self, r: S) -> Self {
        Self {
            rewrite: Some(r.into()),
            ..self
        }
    }
}

impl TermFuzzy {
    pub fn new<S: Into<String>>(field: S, value: Fuzzy) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl Serialize for TermFuzzy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(&self.field, &self.value)?;
        term.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = TermFuzzy::new("speaker", Fuzzy::new("HALET"));
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "speaker": {
                "value": "HALET",
            }
        });

        assert_eq!(json, expected);
    }
}
