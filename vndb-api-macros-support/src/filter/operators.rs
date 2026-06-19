use std::{fmt, str::FromStr};

use bitmask_enum::bitmask;
use thiserror::Error;

#[bitmask(u8)]
pub enum FieldOperatorSet {
    Eq,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
    Equality = Self::Eq.or(Self::NotEqual).bits,
    Ordering = Self::Equality
        .or(Self::Gt)
        .or(Self::Gte)
        .or(Self::Lt)
        .or(Self::Lte)
        .bits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOperator {
    Eq,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl FieldOperator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::NotEqual => "!=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
        }
    }

    pub const fn operator_set(self) -> FieldOperatorSet {
        match self {
            Self::Eq => FieldOperatorSet::Eq,
            Self::NotEqual => FieldOperatorSet::NotEqual,
            Self::Gt => FieldOperatorSet::Gt,
            Self::Gte => FieldOperatorSet::Gte,
            Self::Lt => FieldOperatorSet::Lt,
            Self::Lte => FieldOperatorSet::Lte,
        }
    }
}

impl FromStr for FieldOperator {
    type Err = ParseFieldOperatorError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "=" => Ok(Self::Eq),
            "!=" => Ok(Self::NotEqual),
            ">" => Ok(Self::Gt),
            ">=" => Ok(Self::Gte),
            "<" => Ok(Self::Lt),
            "<=" => Ok(Self::Lte),
            _ => Err(ParseFieldOperatorError),
        }
    }
}

impl fmt::Display for FieldOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("invalid VNDB filter operator")]
pub struct ParseFieldOperatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombiningOperator {
    And,
    Or,
}

impl CombiningOperator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Or => "or",
        }
    }
}
