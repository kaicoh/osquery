use serde::Serialize;

mod term;

use term::Term;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TermLevel<T: Serialize> {
    Term(Term<T>),
}

impl<T: Serialize> TermLevel<T> {
    pub fn term<S: Into<String>>(field: S, value: T) -> Self {
        Self::Term(Term::new(field, value))
    }
}
