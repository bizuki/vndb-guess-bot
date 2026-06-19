use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::{
    ids::CharacterId,
    models::{
        release::{Release, ReleaseFields},
        traits::{Trait, TraitFields},
        vn::{Vn, VnFields},
    },
};

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Character {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: CharacterId,

    #[vndb_sort]
    pub name: Option<String>,
    pub original: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    #[vndb_field(nested)]
    pub image: Option<CharacterImage>,
    pub blood_type: Option<String>,
    pub height: Option<i32>,
    pub weight: Option<i32>,
    pub bust: Option<i32>,
    pub waist: Option<i32>,
    pub hips: Option<i32>,
    pub cup: Option<String>,
    pub age: Option<i32>,
    pub birthday: Option<Vec<i32>>,
    pub sex: Option<Vec<Option<String>>>,
    pub gender: Option<Vec<Option<String>>>,
    #[vndb_field(nested)]
    pub vns: Option<Vec<CharacterVnLink>>,
    #[vndb_field(nested)]
    pub traits: Option<Vec<CharacterTrait>>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct CharacterImage {
    pub id: Option<String>,
    pub url: Option<String>,
    pub dims: Option<Vec<i32>>,
    pub sexual: Option<f64>,
    pub violence: Option<f64>,
    pub votecount: Option<i32>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct CharacterVnLink {
    pub spoiler: Option<i32>,
    pub role: Option<String>,
    #[serde(flatten)]
    #[vndb_field(flatten, boxed)]
    pub vn: Option<Vn>,
    #[vndb_field(nested, boxed)]
    pub release: Option<Release>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct CharacterTrait {
    pub spoiler: Option<i32>,
    pub lie: Option<bool>,
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub trait_fields: Option<Trait>,
}
