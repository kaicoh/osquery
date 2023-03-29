use serde::Serialize;

mod match_all;
mod match_bool_prefix;
mod match_phrase;
mod match_phrase_prefix;
mod mtch;
mod multi_match;
pub mod options;
mod query_string;
mod simple_query_string;

pub use match_all::MatchAll;
pub use match_bool_prefix::MatchBoolPrefix;
pub use match_phrase::MatchPhrase;
pub use match_phrase_prefix::MatchPhrasePrefix;
pub use mtch::Match;
pub use multi_match::MultiMatch;
pub use query_string::QueryString;
pub use simple_query_string::SimpleQueryString;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullText {
    Match(Box<Match>),
    MultiMatch(Box<MultiMatch>),
    MatchBoolPrefix(Box<MatchBoolPrefix>),
    MatchPhrase(Box<MatchPhrase>),
    MatchPhrasePrefix(Box<MatchPhrasePrefix>),
    QueryString(Box<QueryString>),
    SimpleQueryString(Box<SimpleQueryString>),
    MatchAll(Box<MatchAll>),
}

macro_rules! from_types {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for FullText {
                fn from(val: $ty) -> Self {
                    Self::$ty(Box::new(val.into()))
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
    MatchPhrasePrefix,
    QueryString,
    SimpleQueryString,
    MatchAll
}
