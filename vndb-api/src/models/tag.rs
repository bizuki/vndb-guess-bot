use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Tag {
    #[vndb_field(skip)]
    pub id: String,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    pub vn_count: Option<i32>,
}
