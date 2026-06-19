use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::ids::StaffId;

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Staff {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: StaffId,
    pub aid: Option<i32>,
    pub ismain: Option<bool>,
    #[vndb_sort]
    pub name: Option<String>,
    pub original: Option<String>,
    pub lang: Option<String>,
    pub gender: Option<String>,
    pub description: Option<String>,
    #[vndb_field(nested)]
    pub extlinks: Option<Vec<StaffExtLink>>,
    #[vndb_field(nested)]
    pub aliases: Option<Vec<StaffAlias>>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct StaffExtLink {
    pub url: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct StaffAlias {
    pub aid: Option<i32>,
    pub name: Option<String>,
    pub latin: Option<String>,
    pub ismain: Option<bool>,
}
