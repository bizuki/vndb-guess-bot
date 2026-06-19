use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use crate::ids::TraitId;

use super::SpoilerLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraitFilterValue {
    Id(TraitId),
    Spoiler { id: TraitId, spoiler: SpoilerLevel },
}

impl TraitFilterValue {
    pub fn id(id: TraitId) -> Self {
        Self::Id(id)
    }

    pub fn spoiler(id: TraitId, spoiler: SpoilerLevel) -> Self {
        Self::Spoiler { id, spoiler }
    }
}

impl VndbFilterValueType for TraitFilterValue {}

impl IntoVndbFilterValue<TraitFilterValue> for TraitFilterValue {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        match self {
            Self::Id(id) => VndbFilterValue::String(id.0),
            Self::Spoiler { id, spoiler } => VndbFilterValue::Array(vec![
                VndbFilterValue::String(id.0),
                VndbFilterValue::Integer(spoiler.code()),
            ]),
        }
    }
}
