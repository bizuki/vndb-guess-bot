use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Trait {
    #[vndb_field(skip)]
    pub id: String,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    pub sexual: Option<bool>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub char_count: Option<i32>,
}
