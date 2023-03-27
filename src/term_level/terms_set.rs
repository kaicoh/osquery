use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct TermsSet {
    field: Option<String>,
    values: Vec<Value>,
    min_should_match_field: Option<String>,
    min_should_match_script: Option<String>,
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
        Self {
            values: values.into_iter().map(|v| v.into()).collect(),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        let mut values = self.values;
        values.push(value.into());

        Self { values, ..self }
    }

    pub fn minimum_should_match_field<T>(self, field: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            min_should_match_field: Some(field.into()),
            ..self
        }
    }

    pub fn minimum_should_match_script<T>(self, script: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            min_should_match_script: Some(script.into()),
            ..self
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct TermsSetValues<'a> {
    terms: &'a Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_field: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_script: Option<MinShouldMatchScript<'a>>,
}

#[derive(Debug, serde::Serialize)]
struct MinShouldMatchScript<'a> {
    source: &'a str,
}

impl Serialize for TermsSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;

        let val = TermsSetValues {
            terms: &self.values,
            minimum_should_match_field: self.min_should_match_field.as_deref(),
            minimum_should_match_script: self
                .min_should_match_script
                .as_ref()
                .map(|s| MinShouldMatchScript { source: s }),
        };

        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &val)?;
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
