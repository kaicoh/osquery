use serde::Serialize;

mod match_bool_prefix;
mod mtch;
mod multi_match;

pub use match_bool_prefix::MatchBoolPrefix;
pub use mtch::Match;
pub use multi_match::MultiMatch;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullText {
    Match(Match),
    MultiMatch(MultiMatch),
    MatchBoolPrefix(MatchBoolPrefix),
}

macro_rules! from_types {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for FullText {
                fn from(val: $ty) -> Self {
                    Self::$ty(val.into())
                }
            }
        )*
    }
}

from_types! {
    Match,
    MultiMatch,
    MatchBoolPrefix
}
