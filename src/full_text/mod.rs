use serde::Serialize;

mod match_bool_prefix;
mod match_phrase;
mod match_phrase_prefix;
mod mtch;
mod multi_match;

pub use match_bool_prefix::MatchBoolPrefix;
pub use match_phrase::MatchPhrase;
pub use match_phrase_prefix::MatchPhrasePrefix;
pub use mtch::Match;
pub use multi_match::MultiMatch;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullText {
    Match(Match),
    MultiMatch(MultiMatch),
    MatchBoolPrefix(MatchBoolPrefix),
    MatchPhrase(MatchPhrase),
    MatchPhrasePrefix(MatchPhrasePrefix),
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
    MatchBoolPrefix,
    MatchPhrase,
    MatchPhrasePrefix
}
