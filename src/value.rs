use std::{num::ParseFloatError, str::FromStr};
#[derive(Debug, Clone)]
pub struct Value {
    pub name: Option<String>,
    pub value: f64,
}
impl FromStr for Value {
    type Err = ParseFloatError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(':').collect();
        let name = {
            if split.len() > 1 {
                Some(split[1].to_string())
            } else {
                None
            }
        };
        let value = split[0].parse::<f64>()?;
        Ok(Value { name, value })
    }
}
