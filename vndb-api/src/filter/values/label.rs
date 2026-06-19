use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use crate::ids::{LabelId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelFilterValue {
    Id(LabelId),
    UserLabel { user: UserId, label: LabelId },
}

impl LabelFilterValue {
    pub fn id(id: LabelId) -> Self {
        Self::Id(id)
    }

    pub fn user_label(user: UserId, label: LabelId) -> Self {
        Self::UserLabel { user, label }
    }
}

impl VndbFilterValueType for LabelFilterValue {}

impl IntoVndbFilterValue<LabelFilterValue> for LabelFilterValue {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        match self {
            Self::Id(id) => VndbFilterValue::Integer(id.0),
            Self::UserLabel { user, label } => VndbFilterValue::Array(vec![
                VndbFilterValue::String(user.0),
                VndbFilterValue::Integer(label.0),
            ]),
        }
    }
}
