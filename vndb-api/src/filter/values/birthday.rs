use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use super::InvalidFilterValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Birthday {
    month: u8,
    day: u8,
}

impl Birthday {
    pub fn new(month: u8, day: u8) -> Result<Self, InvalidFilterValue> {
        if !(1..=12).contains(&month) {
            return Err(InvalidFilterValue::InvalidBirthdayMonth);
        }

        if !(1..=31).contains(&day) {
            return Err(InvalidFilterValue::InvalidBirthdayDay);
        }

        Ok(Self { month, day })
    }
}

impl VndbFilterValueType for Birthday {}

impl IntoVndbFilterValue<Birthday> for Birthday {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Array(vec![
            VndbFilterValue::Integer(self.month.into()),
            VndbFilterValue::Integer(self.day.into()),
        ])
    }
}
