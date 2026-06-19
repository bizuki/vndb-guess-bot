use std::marker::PhantomData;

use serde::Serialize;

use crate::filter::operators::FieldOperator;
use crate::filter::values::{IntegerBooleanValue, IntoVndbFilterValue, VndbFilterValueType};
use crate::filter::{PredicateSerializer, VndbFilter, VndbFilterValue};

#[derive(Debug, Clone, Copy)]
pub struct EqualityOps;

#[derive(Debug, Clone, Copy)]
pub struct OrderedOps;

#[derive(Debug, Clone, Copy)]
pub struct NonNullable;

#[derive(Debug, Clone, Copy)]
pub struct Nullable;

pub trait SupportsEquality {}

pub trait SupportsOrdering: SupportsEquality {}

impl SupportsEquality for EqualityOps {}
impl SupportsEquality for OrderedOps {}
impl SupportsOrdering for OrderedOps {}

#[derive(Debug, Clone)]
pub struct FilterField<F, V, Ops, Nullability>
where
    V: VndbFilterValueType,
{
    field: F,
    serialize: PredicateSerializer<F>,
    _value: PhantomData<V>,
    _ops: PhantomData<Ops>,
    _nullability: PhantomData<Nullability>,
}

impl<F, V, Ops, Nullability> FilterField<F, V, Ops, Nullability>
where
    V: VndbFilterValueType,
{
    #[doc(hidden)]
    pub fn __new(field: F, serialize: PredicateSerializer<F>) -> Self {
        Self {
            field,
            serialize,
            _value: PhantomData,
            _ops: PhantomData,
            _nullability: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn __into_field(self) -> F {
        self.field
    }

    fn predicate(self, op: FieldOperator, value: VndbFilterValue) -> VndbFilter<F> {
        VndbFilter::predicate(self.field, op, value, self.serialize)
    }
}

impl<F, V, Ops, Nullability> FilterField<F, V, Ops, Nullability>
where
    V: VndbFilterValueType,
    Ops: SupportsEquality,
{
    pub fn eq<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::Eq, value.into_vndb_filter_value())
    }

    pub fn ne<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::NotEqual, value.into_vndb_filter_value())
    }
}

impl<F, V, Ops, Nullability> FilterField<F, V, Ops, Nullability>
where
    V: VndbFilterValueType,
    Ops: SupportsOrdering,
{
    pub fn gt<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::Gt, value.into_vndb_filter_value())
    }

    pub fn gte<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::Gte, value.into_vndb_filter_value())
    }

    pub fn lt<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::Lt, value.into_vndb_filter_value())
    }

    pub fn lte<T>(self, value: T) -> VndbFilter<F>
    where
        T: IntoVndbFilterValue<V>,
    {
        self.predicate(FieldOperator::Lte, value.into_vndb_filter_value())
    }
}

impl<F, V, Ops> FilterField<F, V, Ops, Nullable>
where
    V: VndbFilterValueType,
    Ops: SupportsEquality,
{
    pub fn is_null(self) -> VndbFilter<F> {
        self.predicate(FieldOperator::Eq, VndbFilterValue::Null)
    }

    pub fn is_not_null(self) -> VndbFilter<F> {
        self.predicate(FieldOperator::NotEqual, VndbFilterValue::Null)
    }
}

impl<F, Ops, Nullability> FilterField<F, IntegerBooleanValue, Ops, Nullability>
where
    Ops: SupportsEquality,
{
    pub fn is_true(self) -> VndbFilter<F> {
        self.predicate(
            FieldOperator::Eq,
            IntoVndbFilterValue::<IntegerBooleanValue>::into_vndb_filter_value(true),
        )
    }

    pub fn is_false(self) -> VndbFilter<F> {
        self.predicate(
            FieldOperator::NotEqual,
            IntoVndbFilterValue::<IntegerBooleanValue>::into_vndb_filter_value(true),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NestedFilterField<F, Child> {
    path: &'static str,
    _field: PhantomData<F>,
    _child: PhantomData<Child>,
}

impl<F, Child> NestedFilterField<F, Child> {
    #[doc(hidden)]
    pub const fn __new(path: &'static str) -> Self {
        Self {
            path,
            _field: PhantomData,
            _child: PhantomData,
        }
    }

    pub fn matches(self, filter: VndbFilter<Child>) -> VndbFilter<F>
    where
        VndbFilter<Child>: Serialize,
    {
        self.nested_predicate("=", filter)
    }

    pub fn not_matches(self, filter: VndbFilter<Child>) -> VndbFilter<F>
    where
        VndbFilter<Child>: Serialize,
    {
        self.nested_predicate("!=", filter)
    }

    fn nested_predicate(self, operator: &str, filter: VndbFilter<Child>) -> VndbFilter<F>
    where
        VndbFilter<Child>: Serialize,
    {
        let value = serde_json::to_value(filter)
            .expect("serializing a typed VNDB child filter should not fail");
        VndbFilter::json(serde_json::json!([self.path, operator, value]))
    }
}
