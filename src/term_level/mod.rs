use serde::Serialize;

mod exists;
mod fuzzy;
mod ids;
mod prefix;
mod range;
mod term;
mod terms;
mod terms_set;

use exists::Exists;
use fuzzy::TermFuzzy;
use ids::Ids;
use prefix::Prefix;
use range::TermRange;
use term::Term;
use terms::Terms;
use terms_set::TermsSet;

pub use fuzzy::Fuzzy;
pub use range::Range;
pub use terms_set::MinShouldMatch;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TermLevel<T: Serialize> {
    Term(Term<T>),
    Terms(Terms<T>),
    TermsSet(TermsSet<T>),
    Ids(Ids),
    Range(TermRange<T>),
    Prefix(Prefix<T>),
    Exists(Exists),
    Fuzzy(TermFuzzy<T>),
}

impl<T: Serialize> TermLevel<T> {
    pub fn term<S: Into<String>>(field: S, value: T) -> Self {
        Self::Term(Term::new(field, value))
    }

    pub fn terms<S, U>(field: S, value: U) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self::Terms(Terms::new(field, value))
    }

    pub fn terms_set<S, U>(field: S, value: U, min_should_match: MinShouldMatch) -> Self
    where
        S: Into<String>,
        U: IntoIterator<Item = T>,
    {
        Self::TermsSet(TermsSet::new(field, value, min_should_match))
    }

    pub fn ids<S: IntoIterator<Item = u64>>(values: S) -> Self {
        Self::Ids(Ids::new(values))
    }

    pub fn range<S: Into<String>>(field: S, value: Range<T>) -> Self {
        Self::Range(TermRange::new(field, value))
    }

    pub fn prefix<S: Into<String>>(field: S, value: T) -> Self {
        Self::Prefix(Prefix::new(field, value))
    }

    pub fn exists<S: Into<String>>(field: S) -> Self {
        Self::Exists(Exists::new(field))
    }

    pub fn fuzzy<S: Into<String>>(field: S, value: Fuzzy<T>) -> Self {
        Self::Fuzzy(TermFuzzy::new(field, value))
    }
}
