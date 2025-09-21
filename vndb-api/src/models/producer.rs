use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

use crate::models::common::{ExtLink, ExtLinkFields};

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Producer {
    #[vndb_field(skip)]
    pub id: String,
    pub name: Option<String>,
    pub original: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub lang: Option<String>,
    #[serde(rename = "type")]
    pub producer_type: Option<String>,
    pub description: Option<String>,
    pub extlinks: Option<Vec<ExtLink>>,
}
