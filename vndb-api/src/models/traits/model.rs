use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::ids::TraitId;

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Trait {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: TraitId,
    #[vndb_sort]
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    pub sexual: Option<bool>,
    pub group_id: Option<TraitId>,
    pub group_name: Option<String>,
    #[vndb_sort]
    pub char_count: Option<i32>,
}
