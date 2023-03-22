use serde::Serialize;
use serde_json::Value;

mod exists;
mod fuzzy;
mod ids;
mod prefix;
mod range;
mod regexp;
mod term;
mod terms;
mod terms_set;
mod wildcard;

use exists::Exists;
use fuzzy::TermFuzzy;
use ids::Ids;
use prefix::Prefix;
use range::TermRange;
use regexp::Regexp;
use term::Term;
use terms::Terms;
use terms_set::TermsSet;
use wildcard::Wildcard;

pub use fuzzy::Fuzzy;
pub use range::Range;
pub use terms_set::MinShouldMatch;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TermLevel {
    Term(Term),
    Terms(Terms),
    TermsSet(TermsSet),
    Ids(Ids),
    Range(TermRange),
    Prefix(Prefix),
    Exists(Exists),
    Fuzzy(TermFuzzy),
    Wildcard(Wildcard),
    Regexp(Regexp),
}

impl TermLevel {
    pub fn term<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self::Term(Term::new(field, value))
    }

    pub fn terms<F, V, S>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        Self::Terms(Terms::new(field, value))
    }

    pub fn terms_set<F, V, S>(field: F, value: V, min_should_match: MinShouldMatch) -> Self
    where
        F: Into<String>,
        V: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        Self::TermsSet(TermsSet::new(field, value, min_should_match))
    }

    pub fn ids<V, S>(values: V) -> Self
    where
        V: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        Self::Ids(Ids::new(values))
    }

    pub fn range<F: Into<String>>(field: F, value: Range) -> Self {
        Self::Range(TermRange::new(field, value))
    }

    pub fn prefix<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self::Prefix(Prefix::new(field, value))
    }

    pub fn exists<F: Into<String>>(field: F) -> Self {
        Self::Exists(Exists::new(field))
    }

    pub fn fuzzy<F: Into<String>>(field: F, value: Fuzzy) -> Self {
        Self::Fuzzy(TermFuzzy::new(field, value))
    }

    pub fn wildcard<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self::Wildcard(Wildcard::new(field, value))
    }

    pub fn regexp<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<Value>,
    {
        Self::Regexp(Regexp::new(field, value))
    }
}
