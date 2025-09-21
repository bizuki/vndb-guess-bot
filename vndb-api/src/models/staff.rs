use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Staff {
    #[vndb_field(skip)]
    pub id: String,
    pub aid: Option<i32>,
    pub ismain: Option<bool>,
    pub name: Option<String>,
    pub original: Option<String>,
    pub lang: Option<String>,
    pub gender: Option<String>,
    pub description: Option<String>,
    pub extlinks: Option<Vec<StaffExtLink>>,
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
