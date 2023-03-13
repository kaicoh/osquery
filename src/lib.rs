use serde::Serialize;

pub mod term_level;

pub use term_level::TermLevel;

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

    /// Sets "term" query.
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

    /// Sets "terms" query.
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .terms("line_id", vec![61809, 61810])
    ///     .build();
    ///
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
    pub fn terms<S, U>(self, field: S, value: U) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self {
            term_level: Some(TermLevel::terms(field, value)),
        }
    }

    /// Sets "terms_set" query
    ///
    /// ```
    /// use osquery::{term_level::MinShouldMatch, QueryBuilder};
    ///
    /// let query = QueryBuilder::new()
    ///     .terms_set(
    ///         "classes",
    ///         vec!["CS101", "CS102", "MATH101"],
    ///         MinShouldMatch::field("min_required"),
    ///     )
    ///     .build();
    ///
    /// let json = serde_json::to_value(query).unwrap();
    ///
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "terms_set": {
    ///             "classes": {
    ///                 "terms": ["CS101", "CS102", "MATH101"],
    ///                 "minimum_should_match_field": "min_required",
    ///             },
    ///         },
    ///     },
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn terms_set<S, U>(
        self,
        field: S,
        value: U,
        min_should_match: term_level::MinShouldMatch,
    ) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self {
            term_level: Some(TermLevel::terms_set(field, value, min_should_match)),
        }
    }

    /// Sets "ids" query.
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::<u64>::new()
    ///     .ids(vec![34229, 91296])
    ///     .build();
    ///
    /// let json = serde_json::to_value(query).unwrap();
    ///
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "ids": {
    ///             "values": [34229, 91296]
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn ids<S: IntoIterator<Item = u64>>(self, values: S) -> Self {
        Self {
            term_level: Some(TermLevel::ids(values)),
        }
    }

    /// Sets "range" query.
    ///
    /// ```
    /// use osquery::{term_level::Range, QueryBuilder};
    ///
    /// let query = QueryBuilder::new()
    ///     .range("line_id", Range::gte(10).and_lte(20))
    ///     .build();
    ///
    /// let json = serde_json::to_value(query).unwrap();
    ///
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "range": {
    ///             "line_id": {
    ///                 "gte": 10,
    ///                 "lte": 20,
    ///             }
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn range<S: Into<String>>(self, field: S, value: term_level::Range<T>) -> Self {
        Self {
            term_level: Some(TermLevel::range(field, value)),
        }
    }

    /// Sets "prefix" query.
    ///
    /// ```
    /// use osquery::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .prefix("speaker", "KING")
    ///     .build();
    ///
    /// let json = serde_json::to_value(query).unwrap();
    ///
    /// let expected = serde_json::json!({
    ///     "query": {
    ///         "prefix": {
    ///             "speaker": "KING",
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn prefix<S: Into<String>>(self, field: S, value: T) -> Self {
        Self {
            term_level: Some(TermLevel::prefix(field, value)),
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
