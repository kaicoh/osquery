use serde::Serialize;

mod term;
mod terms;

use term::Term;
use terms::Terms;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TermLevel<T: Serialize> {
    Term(Term<T>),
    Terms(Terms<T>),
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
}
