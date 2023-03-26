use serde::Serialize;

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

pub use exists::Exists;
pub use fuzzy::TermFuzzy;
pub use ids::Ids;
pub use prefix::Prefix;
pub use range::TermRange;
pub use regexp::Regexp;
pub use term::Term;
pub use terms::Terms;
pub use terms_set::TermsSet;
pub use wildcard::Wildcard;

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

macro_rules! from_term_types {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for TermLevel {
                fn from(val: $ty) -> Self {
                    Self::$ty(val.into())
                }
            }
        )*
    }
}

from_term_types! {
    Term,
    Terms,
    TermsSet,
    Ids,
    Prefix,
    Exists,
    Wildcard,
    Regexp
}

impl From<TermRange> for TermLevel {
    fn from(val: TermRange) -> Self {
        Self::Range(val)
    }
}

impl From<TermFuzzy> for TermLevel {
    fn from(val: TermFuzzy) -> Self {
        Self::Fuzzy(val)
    }
}
