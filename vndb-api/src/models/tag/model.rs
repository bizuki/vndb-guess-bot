use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::ids::TagId;

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Tag {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: TagId,
    #[vndb_sort]
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    #[vndb_sort]
    pub vn_count: Option<i32>,
}
