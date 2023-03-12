use serde::Serialize;

mod term_level;

use term_level::TermLevel;

#[derive(Debug, Clone, Serialize)]
pub struct QueryBuilder<T: Serialize> {
    term_level: Option<TermLevel<T>>,
}

impl<T: Serialize> Default for QueryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Serialize> QueryBuilder<T> {
    pub fn new() -> Self {
        Self { term_level: None }
    }

    /// Build query type "term"
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::term("line_id", 61809).build();
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
    pub fn term<S: Into<String>>(field: S, value: T) -> Self {
        Self {
            term_level: Some(TermLevel::term(field, value)),
        }
    }

    /// Build query type "terms"
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::terms("line_id", vec![61809, 61810]).build();
    /// let json = serde_json::to_value(query).unwrap();
    ///
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "terms": {
    ///             "line_id": [61809, 61810]
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn terms<S, U>(field: S, value: U) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self {
            term_level: Some(TermLevel::terms(field, value)),
        }
    }

    pub fn build(self) -> Query<T> {
        Query {
            query: self.term_level.unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Query<T: Serialize> {
    query: TermLevel<T>,
}
