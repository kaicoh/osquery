use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Range {
    field: Option<String>,
    gte: Option<Value>,
    gt: Option<Value>,
    lte: Option<Value>,
    lt: Option<Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
struct RangeValues<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<&'a Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<&'a Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<&'a Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<&'a Value>,
}

impl Range {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }
}

macro_rules! setter {
    ($($attr:ident),*) => {
        impl Range {
            $(
                pub fn $attr<T: Into<Value>>(self, val: T) -> Self {
                    Self {
                        $attr: Some(val.into()),
                        ..self
                    }
                }
            )*
        }
    };
}

setter! { gte, lte, gt, lt }

impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;

        let val = RangeValues {
            gte: self.gte.as_ref(),
            gt: self.gt.as_ref(),
            lte: self.lte.as_ref(),
            lt: self.lt.as_ref(),
        };

        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &val)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let range = Range::new().field("line_id").gte(10).lte(20);

        let json = serde_json::to_value(range).unwrap();

        let expected = serde_json::json!({
            "line_id": {
                "gte": 10,
                "lte": 20,
            },
        });

        assert_eq!(json, expected);
    }
}
