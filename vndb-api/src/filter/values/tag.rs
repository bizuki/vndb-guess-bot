use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use crate::ids::TagId;

use super::InvalidFilterValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpoilerLevel {
    None,
    One,
    Two,
}

impl SpoilerLevel {
    pub(crate) const fn code(self) -> i64 {
        match self {
            Self::None => 0,
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TagLevel(pub(crate) f64);

impl TagLevel {
    pub fn new(value: f64) -> Result<Self, InvalidFilterValue> {
        if (0.0..=3.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidFilterValue::InvalidTagLevel)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagFilterValue {
    Id(TagId),
    Rated {
        id: TagId,
        spoiler: SpoilerLevel,
        level: TagLevel,
    },
}

impl TagFilterValue {
    pub fn id(id: TagId) -> Self {
        Self::Id(id)
    }

    pub fn rated(id: TagId, spoiler: SpoilerLevel, level: TagLevel) -> Self {
        Self::Rated { id, spoiler, level }
    }
}

impl VndbFilterValueType for TagFilterValue {}

impl IntoVndbFilterValue<TagFilterValue> for TagFilterValue {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        match self {
            Self::Id(id) => VndbFilterValue::String(id.0),
            Self::Rated { id, spoiler, level } => VndbFilterValue::Array(vec![
                VndbFilterValue::String(id.0),
                VndbFilterValue::Integer(spoiler.code()),
                VndbFilterValue::Number(level.0),
            ]),
        }
    }
}
