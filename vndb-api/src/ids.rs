use std::{borrow::Borrow, fmt, ops::Deref};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;
use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidId {
    #[error("VNDB id has the wrong prefix")]
    WrongPrefix,
    #[error("VNDB id must end with digits")]
    MissingDigits,
    #[error("label id cannot be negative")]
    NegativeLabelId,
}

fn validate_vndb_id(value: &str, prefix: &str) -> Result<(), InvalidId> {
    let suffix = value.strip_prefix(prefix).ok_or(InvalidId::WrongPrefix)?;

    if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(InvalidId::MissingDigits);
    }

    Ok(())
}

macro_rules! vndb_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub(crate) String);

        impl $name {
            pub const fn prefix() -> &'static str {
                $prefix
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Borrow<str> for $name {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = InvalidId;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                validate_vndb_id(value, $prefix)?;
                Ok(Self(value.to_owned()))
            }
        }

        impl TryFrom<String> for $name {
            type Error = InvalidId;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                validate_vndb_id(&value, $prefix)?;
                Ok(Self(value))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::try_from(value).map_err(serde::de::Error::custom)
            }
        }

        impl VndbFilterValueType for $name {}

        impl IntoVndbFilterValue<$name> for $name {
            fn into_vndb_filter_value(self) -> VndbFilterValue {
                VndbFilterValue::String(self.0)
            }
        }

        impl IntoVndbFilterValue<$name> for &$name {
            fn into_vndb_filter_value(self) -> VndbFilterValue {
                VndbFilterValue::String(self.0.clone())
            }
        }
    };
}

vndb_id!(VnId, "v");
vndb_id!(ReleaseId, "r");
vndb_id!(ProducerId, "p");
vndb_id!(CharacterId, "c");
vndb_id!(StaffId, "s");
vndb_id!(TagId, "g");
vndb_id!(TraitId, "i");
vndb_id!(UserId, "u");
vndb_id!(QuoteId, "q");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LabelId(pub(crate) i64);

impl LabelId {
    pub fn new(value: i64) -> Result<Self, InvalidId> {
        if value < 0 {
            Err(InvalidId::NegativeLabelId)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> i64 {
        self.0
    }
}
