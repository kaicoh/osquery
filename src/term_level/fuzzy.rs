use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Fuzzy {
    field: Option<String>,
    value: Option<Value>,
    fuzziness: Option<String>,
    max_expansions: Option<u64>,
    prefix_length: Option<u64>,
    transpositions: Option<bool>,
    rewrite: Option<String>,
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

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        Self {
            value: Some(value.into()),
            ..self
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

#[derive(Debug, Clone, serde::Serialize)]
struct FuzzyValues<'a> {
    value: &'a Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<&'a u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<&'a u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transpositions: Option<&'a bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<&'a str>,
}

impl Serialize for Fuzzy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut term = serializer.serialize_map(Some(1))?;

        let val = FuzzyValues {
            value: self.value.as_ref().unwrap_or(&Value::Null),
            fuzziness: self.fuzziness.as_deref(),
            max_expansions: self.max_expansions.as_ref(),
            prefix_length: self.prefix_length.as_ref(),
            transpositions: self.transpositions.as_ref(),
            rewrite: self.rewrite.as_deref(),
        };

        term.serialize_entry(self.field.as_deref().unwrap_or_default(), &val)?;
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
            .rewrite("constant_score");

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
