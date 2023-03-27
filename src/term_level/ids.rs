use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Ids {
    values: Vec<Value>,
}

impl Ids {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn values<V, T>(self, values: V) -> Self
    where
        V: IntoIterator<Item = T>,
        T: Into<Value>,
    {
        Self {
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        let mut values = self.values;
        values.push(value.into());
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_to_json() {
        let ids = Ids::new().values(vec![34229, 91296]);

        let json = serde_json::to_value(ids).unwrap();

        let expected = serde_json::json!({
            "values": [34229, 91296],
        });

        assert_eq!(json, expected);
    }
}
