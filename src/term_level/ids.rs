use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Ids {
    values: Vec<u64>,
}

impl Ids {
    pub fn new<T: IntoIterator<Item = u64>>(values: T) -> Self {
        Self {
            values: values.into_iter().collect(),
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
