use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct TermsSet {
    field: Option<String>,
    value: TermsSetValues,
}

impl TermsSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn values<V, T>(self, values: V) -> Self
    where
        V: IntoIterator<Item = T>,
        T: Into<Value>,
    {
        let value = TermsSetValues {
            terms: values.into_iter().map(|v| v.into()).collect(),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn value<T: Into<Value>>(self, val: T) -> Self {
        let mut terms = self.value.terms;
        terms.push(val.into());

        let value = TermsSetValues {
            terms,
            ..self.value
        };

        Self { value, ..self }
    }

    pub fn minimum_should_match_field<T>(self, field: T) -> Self
    where
        T: Into<String>,
    {
        let value = TermsSetValues {
            minimum_should_match_field: Some(field.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn minimum_should_match_script<T>(self, script: T) -> Self
    where
        T: Into<String>,
    {
        let value = TermsSetValues {
            minimum_should_match_script: Some(MinShouldMatchScript {
                source: script.into()
            }),
            ..self.value
        };
        Self { value, ..self }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
struct TermsSetValues {
    terms: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_field: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_script: Option<MinShouldMatchScript>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct MinShouldMatchScript {
    source: String,
}

impl Serialize for TermsSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json_using_min_should_match_field() {
        let terms_set = TermsSet::new()
            .field("classes")
            .values(vec!["CS101", "CS102", "MATH101"])
            .minimum_should_match_field("min_required");

        let json = serde_json::to_value(terms_set).unwrap();

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
        let terms_set = TermsSet::new()
            .field("classes")
            .value("CS101")
            .value("CS102")
            .value("MATH101")
            .minimum_should_match_script("Math.min(params.num_terms, doc['min_required'].value)");

        let json = serde_json::to_value(terms_set).unwrap();

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
