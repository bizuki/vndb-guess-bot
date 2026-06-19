extern crate self as vndb_api;

#[allow(dead_code)]
pub mod filter {
    use std::marker::PhantomData;

    pub struct FilterField<F, V, Ops, Nullability> {
        field: F,
        _value: PhantomData<V>,
        _ops: PhantomData<Ops>,
        _nullability: PhantomData<Nullability>,
    }

    impl<F, V, Ops, Nullability> FilterField<F, V, Ops, Nullability> {
        pub fn __new(
            field: F,
            _serialize: fn(&F, &str, serde_json::Value) -> serde_json::Value,
        ) -> Self {
            Self {
                field,
                _value: PhantomData,
                _ops: PhantomData,
                _nullability: PhantomData,
            }
        }

        pub fn __into_field(self) -> F {
            self.field
        }
    }

    pub struct NestedFilterField<F, Child> {
        _field: PhantomData<F>,
        _child: PhantomData<Child>,
    }

    impl<F, Child> NestedFilterField<F, Child> {
        pub const fn __new(_path: &'static str) -> Self {
            Self {
                _field: PhantomData,
                _child: PhantomData,
            }
        }
    }

    pub struct EqualityOps;
    pub struct OrderedOps;
    pub struct NonNullable;
    pub struct Nullable;
    pub struct StringValue;
    pub struct IntegerValue;
    pub struct NumberValue;
    pub struct BooleanValue;
    pub struct IntegerBooleanValue;
    pub struct TagFilterValue;
    pub struct TraitFilterValue;
    pub struct LabelFilterValue;
    pub struct ExtlinkFilterValue;
    pub struct Birthday;
    pub struct Resolution;
    pub struct VndbFilterValue;
}
