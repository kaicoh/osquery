use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Ids {
    values: Vec<Value>,
}

impl Ids {
    pub fn new<V, S>(values: V) -> Self
    where
        V: IntoIterator<Item = S>,
        S: Into<Value>,
    {
        Self {
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let ids = Ids::new(vec![34229, 91296]);
        let json = serde_json::to_value(ids).unwrap();

        let expected = serde_json::json!({
            "values": [34229, 91296],
        });

        assert_eq!(json, expected);
    }
}
