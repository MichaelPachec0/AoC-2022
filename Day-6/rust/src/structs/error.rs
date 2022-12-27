use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct ComputeErr {
    details: String,
}

impl Display for ComputeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ComputeErr {
    fn description(&self) -> &str {
        &self.details
    }
}

impl ComputeErr {
    pub fn new(details: &str) -> Self {
        Self {
            details: details.to_string(),
        }
    }
}
