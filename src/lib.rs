use serde::Serialize;

mod term_level;

use term_level::TermLevel;

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryBuilder<T: Serialize> {
    term_level: Option<TermLevel<T>>,
}

impl<T: Serialize> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            term_level: None
        }
    }

    /// Sets query type "term"
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .term("line_id", 61809)
    ///     .build();
    ///
    /// let json = serde_json::to_value(query).unwrap();
    ///
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
    pub fn term<S: Into<String>>(self, field: S, value: T) -> Self {
        Self {
            term_level: Some(TermLevel::term(field, value)),
        }
    }

    pub fn build(self) -> Query<T> {
        Query {
            query: self.term_level.unwrap()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Query<T: Serialize> {
    query: TermLevel<T>,
}
