use crate::error::ValidationError;

#[derive(Debug)]
pub enum Operator {
    ADD,
    SUB,
    SET,
}

impl Operator {
    pub fn parse(op: &str) -> Result<Self, ValidationError> {
        match op {
            "+=" => Ok(Self::ADD),
            "-=" => Ok(Self::SUB),
            "=" => Ok(Self::SET),
            _ => Err(verror!("No valid Operator matches {}", op)),
        }
    }
}
