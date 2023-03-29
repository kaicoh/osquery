use serde::ser::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Fuzziness {
    Auto,
    Uint(u64),
}

impl<T: Into<u64>> From<T> for Fuzziness {
    fn from(val: T) -> Self {
        Self::Uint(val.into())
    }
}

impl Serialize for Fuzziness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Fuzziness::Auto => serializer.serialize_str("AUTO"),
            Fuzziness::Uint(u) => serializer.serialize_u64(*u),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rewrite {
    ConstantScore,
    ScoringBoolean,
    ConstantScoreBoolean,
    #[serde(rename = "top_terms_N")]
    TopTermsN,
    #[serde(rename = "top_terms_boost_N")]
    TopTermsBoostN,
    #[serde(rename = "top_terms_blended_freqs_N")]
    TopTermsBlendedFreqsN,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_fuziness_auto() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Fuzziness::Auto,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "AUTO"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_fuziness_uint() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Fuzziness::Uint(30),
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": 30
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_operator_and() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Operator::And,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "and"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_operator_or() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Operator::Or,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "or"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_constant_score() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::ConstantScore,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "constant_score"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_scoring_boolean() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::ScoringBoolean,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "scoring_boolean"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_constant_score_boolean() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::ConstantScoreBoolean,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "constant_score_boolean"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_top_terms_n() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::TopTermsN,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "top_terms_N"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_top_terms_boost_n() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::TopTermsBoostN,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "top_terms_boost_N"
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rewrite_top_terms_blended_freqs_n() {
        let json = serde_json::to_value(serde_json::json!({
            "value": Rewrite::TopTermsBlendedFreqsN,
        }))
        .unwrap();
        let expected = serde_json::json!({
            "value": "top_terms_blended_freqs_N"
        });
        assert_eq!(json, expected);
    }
}
