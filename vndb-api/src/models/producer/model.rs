use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::{
    ids::ProducerId,
    models::common::{ExtLink, ExtLinkFields},
};

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Producer {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: ProducerId,
    #[vndb_sort]
    pub name: Option<String>,
    pub original: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub lang: Option<String>,
    #[serde(rename = "type")]
    #[vndb_field(rename = "type")]
    pub producer_type: Option<String>,
    pub description: Option<String>,
    #[vndb_field(nested)]
    pub extlinks: Option<Vec<ExtLink>>,
}
