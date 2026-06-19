use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use super::InvalidFilterValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Result<Self, InvalidFilterValue> {
        if width == 0 || height == 0 {
            return Err(InvalidFilterValue::NonPositiveResolution);
        }

        Ok(Self { width, height })
    }
}

impl VndbFilterValueType for Resolution {}

impl IntoVndbFilterValue<Resolution> for Resolution {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        VndbFilterValue::Array(vec![
            VndbFilterValue::Integer(i64::from(self.width)),
            VndbFilterValue::Integer(i64::from(self.height)),
        ])
    }
}
