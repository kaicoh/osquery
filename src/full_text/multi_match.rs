use super::options::{Fuzziness, Operator};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Clone, Serialize)]
pub struct MultiMatch {
    query: Option<Value>,
    fields: Vec<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    typ: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tie_breaker: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_generate_synonyms_phrase_query: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zero_terms_query: Option<String>,
}

impl MultiMatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fields<F, T>(self, fields: F) -> Self
    where
        F: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            ..self
        }
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        let mut fields = self.fields;
        fields.push(field.into());
        Self { fields, ..self }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        Self {
            query: Some(value.into()),
            ..self
        }
    }

    pub fn query<T: Into<Value>>(self, query: T) -> Self {
        self.value(query)
    }

    pub fn typ<T: Into<String>>(self, typ: T) -> Self {
        Self {
            typ: Some(typ.into()),
            ..self
        }
    }

    pub fn operator<T: Into<Operator>>(self, operator: T) -> Self {
        Self {
            operator: Some(operator.into()),
            ..self
        }
    }

    pub fn minimum_should_match<T: Into<u64>>(self, minimum_should_match: T) -> Self {
        Self {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self
        }
    }

    pub fn tie_breaker<T: Into<f64>>(self, tie_breaker: T) -> Self {
        Self {
            tie_breaker: Some(tie_breaker.into()),
            ..self
        }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        Self {
            fuzziness: Some(fuzziness.into()),
            ..self
        }
    }

    pub fn fuzzy_transpositions(self, fuzzy_transpositions: bool) -> Self {
        Self {
            fuzzy_transpositions: Some(fuzzy_transpositions),
            ..self
        }
    }

    pub fn lenient(self, lenient: bool) -> Self {
        Self {
            lenient: Some(lenient),
            ..self
        }
    }

    pub fn prefix_length<T: Into<u64>>(self, prefix_length: T) -> Self {
        Self {
            prefix_length: Some(prefix_length.into()),
            ..self
        }
    }

    pub fn max_expansions<T: Into<u64>>(self, max_expansions: T) -> Self {
        Self {
            max_expansions: Some(max_expansions.into()),
            ..self
        }
    }

    pub fn auto_generate_synonyms_phrase_query(
        self,
        auto_generate_synonyms_phrase_query: bool,
    ) -> Self {
        Self {
            auto_generate_synonyms_phrase_query: Some(auto_generate_synonyms_phrase_query),
            ..self
        }
    }

    pub fn zero_terms_query<T: Into<String>>(self, zero_terms_query: T) -> Self {
        Self {
            zero_terms_query: Some(zero_terms_query.into()),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let query = MultiMatch::new()
            .fields(vec!["title^4", "description"])
            .query("wind")
            .typ("most_fields")
            .operator(Operator::And)
            .minimum_should_match(3 as u64);
        let json = serde_json::to_value(query).unwrap();

        let expected = serde_json::json!({
            "query": "wind",
            "fields": ["title^4", "description"],
            "type": "most_fields",
            "operator": "and",
            "minimum_should_match": 3
        });

        assert_eq!(json, expected);
    }
}
