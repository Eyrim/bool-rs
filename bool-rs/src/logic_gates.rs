use std::{fmt, num::TryFromIntError, str::FromStr};

pub enum Gate {
    And,
    Or,
}

pub trait LogicGate {
    fn test(&self, input: Vec<i8>) -> bool;
}

#[derive(Debug)]
pub struct UnknownLogicGateError {
    pub unrecognised_gate: String,
}

impl UnknownLogicGateError {
    pub fn new(unrecognised_gate: String) -> UnknownLogicGateError {
        UnknownLogicGateError { unrecognised_gate }
    }
}

impl fmt::Display for UnknownLogicGateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unknown logic gate, no gate matching {}",
            self.unrecognised_gate
        )
    }
}

impl TryFrom<&str> for Gate {
    type Error = UnknownLogicGateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Gate::Or),
            "." => Ok(Gate::And),
            _ => Err(UnknownLogicGateError::new(value.to_owned())),
        }
    }
}

impl LogicGate for Gate {
    fn test(&self, input: Vec<i8>) -> bool {
        match self {
            Gate::And => and(input),
            Gate::Or => or(input),
        }
    }
}

fn and(input: Vec<i8>) -> bool {
    let expected_count = input.len();

    // TODO: will this affect the vector in other places? or just this scope?
    input.into_iter().filter(is_true).count() == expected_count
}

fn or(input: Vec<i8>) -> bool {
    input.iter().any(is_true)
}

fn is_true(i: &i8) -> bool {
    *i == 1
}
