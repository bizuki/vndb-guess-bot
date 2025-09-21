use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ExtLink {
    pub url: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
}
