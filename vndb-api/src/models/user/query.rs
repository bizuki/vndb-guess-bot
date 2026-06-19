use std::collections::BTreeMap;

#[cfg(any(feature = "reqwest-client", feature = "mock-client"))]
use vndb_api_macros_support::selector::VndbSelector;

use crate::{
    ids::UserId,
    models::user::{User, UserFields},
};

pub type UserLookupResponse = BTreeMap<String, Option<User>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserLookup {
    Id(UserId),
    Username(String),
}

impl UserLookup {
    pub fn id(id: UserId) -> Self {
        Self::Id(id)
    }

    pub fn username(username: impl Into<String>) -> Self {
        Self::Username(username.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Id(id) => id.as_str(),
            Self::Username(username) => username,
        }
    }
}

impl From<UserId> for UserLookup {
    fn from(id: UserId) -> Self {
        Self::Id(id)
    }
}

#[derive(Debug, Clone)]
pub struct UserLookupQuery {
    pub queries: Vec<UserLookup>,
    pub fields: Vec<UserFields>,
}

impl UserLookupQuery {
    pub fn new(
        queries: impl IntoIterator<Item = UserLookup>,
        fields: impl IntoIterator<Item = UserFields>,
    ) -> Self {
        Self {
            queries: queries.into_iter().collect(),
            fields: fields.into_iter().collect(),
        }
    }

    pub fn single(query: UserLookup, fields: impl IntoIterator<Item = UserFields>) -> Self {
        Self::new([query], fields)
    }

    #[cfg(any(feature = "reqwest-client", feature = "mock-client"))]
    pub(crate) fn selected_fields(&self) -> String {
        self.fields
            .iter()
            .map(VndbSelector::selector)
            .collect::<Vec<_>>()
            .join(",")
    }
}
