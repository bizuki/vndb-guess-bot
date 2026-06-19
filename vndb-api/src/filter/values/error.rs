use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidFilterValue {
    #[error("external link site cannot be empty")]
    EmptyExtlinkSite,
    #[error("external link URL cannot be empty")]
    EmptyExtlinkUrl,
    #[error("birthday month must be 1 through 12")]
    InvalidBirthdayMonth,
    #[error("birthday day must be 1 through 31")]
    InvalidBirthdayDay,
    #[error("resolution dimensions must be positive")]
    NonPositiveResolution,
    #[error("tag level must be between 0 and 3")]
    InvalidTagLevel,
    #[error("integer value is outside the supported VNDB filter range")]
    IntegerOutOfRange,
}

impl From<TryFromIntError> for InvalidFilterValue {
    fn from(_: TryFromIntError) -> Self {
        Self::IntegerOutOfRange
    }
}
