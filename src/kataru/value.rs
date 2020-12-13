use crate::error::ValidationError;
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    None,
    String(String),
    Number(f64),
    Bool(bool),
}

impl AddAssign<&Self> for Value {
    fn add_assign(&mut self, rhs: &Self) {
        match (&self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => *self = Self::Number(n1 + n2),
            _ => (),
        }
    }
}

impl SubAssign<&Self> for Value {
    fn sub_assign(&mut self, rhs: &Self) {
        match (&self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => *self = Self::Number(*n1 - n2),
            _ => *self = Self::None,
        }
    }
}

impl Value {
    pub fn same_type(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Value::Bool(_), Value::Bool(_)) => true,
            (Value::Number(_), Value::Number(_)) => true,
            (Value::String(_), Value::String(_)) => true,
            _ => false,
        }
    }

    // pub fn is_eq(&self, rhs: &Self) -> Result<bool, ValidationError> {
    //     match (self, rhs) {
    //         (Value::Bool(b1), Value::Bool(b2)) => Ok(b1 == b2),
    //         (Value::Number(n1), Value::Number(n2)) => Ok(n1 == n2),
    //         (Value::String(s1), Value::String(s2)) => Ok(s1 == s2),
    //         _ => Err(verror!("Cannot compare types {:?} and {:?}", self, rhs)),
    //     }
    // }

    // pub fn is_lt(&self, rhs: &Self) -> Result<bool, ValidationError> {
    //     match (self, rhs) {
    //         (Value::Number(n1), Value::Number(n2)) => Ok(n1 < n2),
    //         _ => Err(verror!("Cannot compare types {:?} and {:?}", self, rhs)),
    //     }
    // }

    // pub fn is_leq(&self, rhs: &Self) -> Result<bool, ValidationError> {
    //     match (self, rhs) {
    //         (Value::Number(n1), Value::Number(n2)) => Ok(n1 <= n2),
    //         _ => Err(verror!("Cannot compare types {:?} and {:?}", self, rhs)),
    //     }
    // }

    fn from_yaml(yaml_value: serde_yaml::Value) -> Result<Self, ValidationError> {
        match yaml_value {
            serde_yaml::Value::Bool(b) => Ok(Value::Bool(b)),
            serde_yaml::Value::String(s) => Ok(Value::String(s)),
            serde_yaml::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap())),
            _ => Err(verror!("Cannot create value from {:?}", yaml_value)),
        }
    }

    pub fn parse(text: &str) -> Result<Value, ValidationError> {
        match serde_yaml::from_str(&text) {
            Ok(r) => Self::from_yaml(r),
            Err(e) => Err(verror!("{}", e)),
        }
    }
}
