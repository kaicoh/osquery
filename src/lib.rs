use serde::Serialize;

pub mod full_text;
pub mod term_level;

use term_level::TermLevel;

#[derive(Debug, Clone)]
pub struct Query;

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

impl Query {
    pub fn new() -> Self {
        Self
    }

    /// Build "term_level" query
    ///
    /// ```
    /// use osquery::{term_level::Term, Query};
    ///
    /// let query = Query::new()
    ///     .term_level(
    ///         Term::new()
    ///             .field("line_id")
    ///             .value(61809)
    ///     );
    ///
    /// let json = serde_json::to_value(query).unwrap();
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "term": {
    ///             "line_id": {
    ///                 "value": 61809
    ///             }
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn term_level<T: Into<TermLevel>>(self, query: T) -> TermLevelQuery {
        TermLevelQuery {
            query: query.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TermLevelQuery {
    query: TermLevel,
}
