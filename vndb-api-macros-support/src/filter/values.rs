use crate::filter::VndbFilterValue;

pub trait VndbFilterValueType {}

pub trait IntoVndbFilterValue<Kind>
where
    Kind: VndbFilterValueType,
{
    fn into_vndb_filter_value(self) -> VndbFilterValue;
}

#[derive(Debug, Clone, Copy)]
pub struct StringValue;

#[derive(Debug, Clone, Copy)]
pub struct IntegerValue;

#[derive(Debug, Clone, Copy)]
pub struct NumberValue;

#[derive(Debug, Clone, Copy)]
pub struct BooleanValue;

#[derive(Debug, Clone, Copy)]
pub struct IntegerBooleanValue;

impl VndbFilterValueType for StringValue {}
impl VndbFilterValueType for IntegerValue {}
impl VndbFilterValueType for NumberValue {}
impl VndbFilterValueType for BooleanValue {}
impl VndbFilterValueType for IntegerBooleanValue {}

impl IntoVndbFilterValue<StringValue> for String {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::String(self)
    }
}

impl IntoVndbFilterValue<StringValue> for &str {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::String(self.to_owned())
    }
}

impl IntoVndbFilterValue<BooleanValue> for bool {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Boolean(self)
    }
}

impl IntoVndbFilterValue<IntegerBooleanValue> for bool {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Integer(if self { 1 } else { 0 })
    }
}

macro_rules! impl_integer_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoVndbFilterValue<IntegerValue> for $ty {
                fn into_vndb_filter_value(self) -> VndbFilterValue {
                    VndbFilterValue::Integer(self.into())
                }
            }
        )*
    };
}

impl_integer_value!(i8, i16, i32, i64, u8, u16, u32);

macro_rules! impl_checked_integer_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoVndbFilterValue<IntegerValue> for $ty {
                fn into_vndb_filter_value(self) -> VndbFilterValue {
                    let value = i64::try_from(self)
                        .expect("VNDB integer filter value is outside the supported i64 range");

                    VndbFilterValue::Integer(value)
                }
            }
        )*
    };
}

impl_checked_integer_value!(i128, isize, u64, u128, usize);

impl IntoVndbFilterValue<NumberValue> for f64 {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Number(self)
    }
}

impl IntoVndbFilterValue<NumberValue> for f32 {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Number(self.into())
    }
}

impl IntoVndbFilterValue<NumberValue> for i32 {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Integer(self.into())
    }
}

impl IntoVndbFilterValue<NumberValue> for i64 {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Integer(self)
    }
}
