use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::{
    ids::QuoteId,
    models::{
        character::{Character, CharacterFields},
        vn::{Vn, VnFields},
    },
};

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
pub struct Quote {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: QuoteId,
    pub quote: Option<String>,
    #[vndb_sort]
    pub score: Option<i32>,
    #[vndb_field(nested)]
    pub vn: Option<Vn>,
    #[vndb_field(nested)]
    pub character: Option<Character>,
}
