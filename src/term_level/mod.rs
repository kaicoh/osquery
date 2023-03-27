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
pub use fuzzy::Fuzzy;
pub use ids::Ids;
pub use prefix::Prefix;
pub use range::Range;
pub use regexp::Regexp;
pub use term::Term;
pub use terms::Terms;
pub use terms_set::TermsSet;
pub use wildcard::Wildcard;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TermLevel {
    Term(Term),
    Terms(Terms),
    TermsSet(TermsSet),
    Ids(Ids),
    Range(Range),
    Prefix(Prefix),
    Exists(Exists),
    Fuzzy(Fuzzy),
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
    Range,
    Prefix,
    Exists,
    Fuzzy,
    Wildcard,
    Regexp
}
