use serde::Serialize;
use serde_json::Value;

mod mtch;
mod multi_match;

use mtch::Match;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullText {
    Match(Match)
}

impl FullText {
    pub fn match_query<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self::Match(Match::new(field, value))
    }
}
