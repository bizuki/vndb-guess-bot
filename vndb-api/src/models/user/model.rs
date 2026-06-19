use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

use crate::ids::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, VndbFieldsEnum)]
pub struct User {
    #[vndb_field(skip)]
    pub id: UserId,
    #[vndb_field(skip)]
    pub username: String,
    pub lengthvotes: Option<u64>,
    pub lengthvotes_sum: Option<u64>,
}
