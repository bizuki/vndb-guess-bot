pub mod field;
pub mod operators;
pub mod values;

use serde::ser::SerializeSeq;

use crate::filter::operators::{CombiningOperator, FieldOperator};

pub use field::{EqualityOps, FilterField, NestedFilterField, NonNullable, Nullable, OrderedOps};
pub use values::{
    BooleanValue, IntegerBooleanValue, IntegerValue, IntoVndbFilterValue, NumberValue, StringValue,
    VndbFilterValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VndbFilterValue {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<VndbFilterValue>),
}

impl VndbFilterValue {
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn into_json(self) -> serde_json::Value {
        match self {
            Self::String(value) => serde_json::Value::String(value),
            Self::Integer(value) => serde_json::Value::Number(value.into()),
            Self::Number(value) => serde_json::Number::from_f64(value)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Self::Boolean(value) => serde_json::Value::Bool(value),
            Self::Null => serde_json::Value::Null,
            Self::Array(values) => serde_json::Value::Array(
                values.into_iter().map(VndbFilterValue::into_json).collect(),
            ),
        }
    }
}

impl serde::Serialize for VndbFilterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(value) => serializer.serialize_str(value),
            Self::Integer(value) => serializer.serialize_i64(*value),
            Self::Number(value) => serializer.serialize_f64(*value),
            Self::Boolean(value) => serializer.serialize_bool(*value),
            Self::Null => serializer.serialize_none(),
            Self::Array(values) => serde::Serialize::serialize(values, serializer),
        }
    }
}

#[doc(hidden)]
pub type PredicateSerializer<F> = fn(&F, &str, serde_json::Value) -> serde_json::Value;

#[derive(Debug, Clone)]
pub enum VndbFilter<VndbModelField> {
    And(Vec<VndbFilter<VndbModelField>>),
    Or(Vec<VndbFilter<VndbModelField>>),
    Json(serde_json::Value),
    Predicate {
        field: VndbModelField,
        op: FieldOperator,
        value: VndbFilterValue,
        serialize: PredicateSerializer<VndbModelField>,
    },
}

impl<VndbModelField> VndbFilter<VndbModelField> {
    pub fn predicate(
        field: VndbModelField,
        op: FieldOperator,
        value: VndbFilterValue,
        serialize: PredicateSerializer<VndbModelField>,
    ) -> Self {
        Self::Predicate {
            field,
            op,
            value,
            serialize,
        }
    }

    pub fn and(filters: Vec<Self>) -> Self {
        Self::And(filters)
    }

    pub fn or(filters: Vec<Self>) -> Self {
        Self::Or(filters)
    }

    #[doc(hidden)]
    pub fn json(value: serde_json::Value) -> Self {
        Self::Json(value)
    }
}

impl<VndbModelField> serde::Serialize for VndbFilter<VndbModelField> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::And(filters) => {
                serialize_combination(serializer, CombiningOperator::And, filters)
            }
            Self::Or(filters) => serialize_combination(serializer, CombiningOperator::Or, filters),
            Self::Json(value) => value.serialize(serializer),
            Self::Predicate {
                field,
                op,
                value,
                serialize,
            } => {
                let value = value.clone().into_json();
                let predicate = serialize(field, op.as_str(), value);

                predicate.serialize(serializer)
            }
        }
    }
}

fn serialize_combination<S, F>(
    serializer: S,
    operator: CombiningOperator,
    filters: &[VndbFilter<F>],
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(Some(filters.len() + 1))?;
    seq.serialize_element(operator.as_str())?;

    for filter in filters {
        seq.serialize_element(filter)?;
    }

    seq.end()
}
