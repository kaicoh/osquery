use serde::Serialize;

mod mtch;
mod multi_match;

pub use mtch::Match;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullText {
    Match(Match),
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
    Match
}
