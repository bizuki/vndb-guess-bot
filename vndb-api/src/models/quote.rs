use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

use crate::models::{character::{Character, CharacterFields}, vn::{Vn, VnFields}};

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Quote {
    #[vndb_field(skip)]
    pub id: String,
    pub quote: Option<String>,
    pub score: Option<i32>,
    pub vn: Option<Vn>,
    pub character: Option<Character>,
}
