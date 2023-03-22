use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TermRange {
    field: String,
    value: Range,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Range {
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<Value>,
}

macro_rules! setter {
    ($($attr:ident, $and_attr:ident),*) => {
        impl Range {
            $(
                pub fn $attr<S: Into<Value>>(val: S) -> Self {
                    Self {
                        $attr: Some(val.into()),
                        ..Self::default()
                    }
                }

                pub fn $and_attr<S: Into<Value>>(self, val: S) -> Self {
                    Self {
                        $attr: Some(val.into()),
                        ..self
                    }
                }
            )*
        }
    };
}

setter!(gte, and_gte, lte, and_lte, gt, and_gt, lt, and_lt);

impl TermRange {
    pub fn new<F: Into<String>>(field: F, value: Range) -> Self {
        Self {
            field: field.into(),
            value,
        }
    }
}

impl Serialize for TermRange {
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
