use crate::options::{Fuzziness, Operator};
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct MatchBoolPrefix {
    field: Option<String>,
    value: MatchBoolPrefixValues,
}

impl MatchBoolPrefix {
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
        let value = MatchBoolPrefixValues {
            query: Some(val.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn query<T: Into<Value>>(self, value: T) -> Self {
        self.value(value)
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        let value = MatchBoolPrefixValues {
            fuzziness: Some(fuzziness.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn fuzzy_transpositions(self, fuzzy_transpositions: bool) -> Self {
        let value = MatchBoolPrefixValues {
            fuzzy_transpositions: Some(fuzzy_transpositions),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn operator<T: Into<Operator>>(self, operator: T) -> Self {
        let value = MatchBoolPrefixValues {
            operator: Some(operator.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn minimum_should_match<T: Into<u64>>(self, min_should_match: T) -> Self {
        let value = MatchBoolPrefixValues {
            minimum_should_match: Some(min_should_match.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        let value = MatchBoolPrefixValues {
            analyzer: Some(analyzer.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn prefix_length<T: Into<u64>>(self, prefix_length: T) -> Self {
        let value = MatchBoolPrefixValues {
            prefix_length: Some(prefix_length.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn max_expansions<T: Into<u64>>(self, max_expansions: T) -> Self {
        let value = MatchBoolPrefixValues {
            max_expansions: Some(max_expansions.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
struct MatchBoolPrefixValues {
    query: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,
}

impl Serialize for MatchBoolPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let query = MatchBoolPrefix::new()
            .field("title")
            .value("rises wi")
            .fuzziness(Fuzziness::Auto)
            .analyzer("standard");
        let json = serde_json::to_value(query).unwrap();

        let expected = serde_json::json!({
            "title": {
                "query": "rises wi",
                "fuzziness": "AUTO",
                "analyzer": "standard"
            }
        });

        assert_eq!(json, expected);
    }
}
