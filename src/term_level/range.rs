use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone)]
pub struct TermRange<T: Serialize> {
    field: String,
    value: Range<T>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Range<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<T>,
}

impl<T: Serialize> Default for Range<T> {
    fn default() -> Self {
        Self {
            gte: None,
            gt: None,
            lte: None,
            lt: None,
        }
    }
}

impl<T: Serialize> Range<T> {
    /// Creates Range with gte field.
    pub fn gte(value: T) -> Self {
        Self {
            gte: Some(value),
            ..Self::default()
        }
    }

    /// Creates Range with gt field.
    pub fn gt(value: T) -> Self {
        Self {
            gt: Some(value),
            ..Self::default()
        }
    }

    /// Creates Range with lte field.
    pub fn lte(value: T) -> Self {
        Self {
            lte: Some(value),
            ..Self::default()
        }
    }

    /// Creates Range with lt field.
    pub fn lt(value: T) -> Self {
        Self {
            lt: Some(value),
            ..Self::default()
        }
    }

    /// Sets value to gte field.
    pub fn and_gte(self, value: T) -> Self {
        Self {
            gte: Some(value),
            ..self
        }
    }

    /// Sets value to gt field.
    pub fn and_gt(self, value: T) -> Self {
        Self {
            gt: Some(value),
            ..self
        }
    }

    /// Sets value to lte field.
    pub fn and_lte(self, value: T) -> Self {
        Self {
            lte: Some(value),
            ..self
        }
    }

    /// Sets value to lt field.
    pub fn and_lt(self, value: T) -> Self {
        Self {
            lt: Some(value),
            ..self
        }
    }
}

impl<T: Serialize> TermRange<T> {
    pub fn new<S: Into<String>>(field: S, value: Range<T>) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl<T: Serialize> Serialize for TermRange<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field, &self.value)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let term = TermRange::new("line_id", Range::gte(10).and_lte(20));
        let json = serde_json::to_value(term).unwrap();

        let expected = serde_json::json!({
            "line_id": {
                "gte": 10,
                "lte": 20,
            },
        });

        assert_eq!(json, expected);
    }
}
