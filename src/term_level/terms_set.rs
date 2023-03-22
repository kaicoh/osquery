use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TermsSet {
    field: String,
    value: Vec<Value>,
    min_should_match: MinShouldMatch,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MinShouldMatch {
    Field(String),
    Script { source: String },
}

impl MinShouldMatch {
    /// Creates condition "minimum_should_match_field".
    pub fn field<S: Into<String>>(field: S) -> Self {
        Self::Field(field.into())
    }

    /// Craetes condition "minimum_should_match_script".
    pub fn script<S: Into<String>>(script: S) -> Self {
        Self::Script {
            source: script.into(),
        }
    }
}

impl TermsSet {
    pub fn new<F, V, S>(field: F, value: V, min_should_match: MinShouldMatch) -> Self
    where
        F: Into<String>,
        V: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        Self {
            field: field.into(),
            value: value.into_iter().map(|v| v.into()).collect(),
            min_should_match,
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct TermsSetMinField<'a, T: Serialize> {
    terms: &'a Vec<T>,
    minimum_should_match_field: &'a MinShouldMatch,
}

#[derive(Debug, serde::Serialize)]
struct TermsSetMinScript<'a, T: Serialize> {
    terms: &'a Vec<T>,
    minimum_should_match_script: &'a MinShouldMatch,
}

impl Serialize for TermsSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;

        match &self.min_should_match {
            MinShouldMatch::Field(_) => {
                let val = TermsSetMinField {
                    terms: &self.value,
                    minimum_should_match_field: &self.min_should_match,
                };

                state.serialize_entry(&self.field, &val)?;
            }
            MinShouldMatch::Script { .. } => {
                let val = TermsSetMinScript {
                    terms: &self.value,
                    minimum_should_match_script: &self.min_should_match,
                };

                state.serialize_entry(&self.field, &val)?;
            }
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json_using_min_should_match_field() {
        let min = MinShouldMatch::field("min_required");
        let term = TermsSet::new("classes", vec!["CS101", "CS102", "MATH101"], min);
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "classes": {
                "terms": ["CS101", "CS102", "MATH101"],
                "minimum_should_match_field": "min_required",
            },
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_to_json_using_min_should_match_script() {
        let min = MinShouldMatch::script("Math.min(params.num_terms, doc['min_required'].value)");
        let term = TermsSet::new("classes", vec!["CS101", "CS102", "MATH101"], min);
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "classes": {
                "terms": ["CS101", "CS102", "MATH101"],
                "minimum_should_match_script": {
                    "source": "Math.min(params.num_terms, doc['min_required'].value)",
                },
            },
        });

        assert_eq!(json, expected);
    }
}
