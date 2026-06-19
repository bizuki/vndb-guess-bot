use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::{
    ids::VnId,
    models::common::{ExtLink, ExtLinkFields},
    models::{
        character::{Character, CharacterFields},
        producer::{Producer, ProducerFields},
        release::{Release, ReleaseFields},
        staff::{Staff, StaffFields},
        tag::{Tag, TagFields},
    },
};

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Vn {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: VnId,

    #[vndb_sort]
    pub title: Option<String>,
    pub alttitle: Option<String>,
    #[vndb_field(nested)]
    pub titles: Option<Vec<VnTitle>>,
    pub aliases: Option<Vec<String>>,

    pub olang: Option<String>,

    pub devstatus: Option<u8>,
    #[vndb_sort]
    pub released: Option<String>,

    pub languages: Option<Vec<String>>,

    pub platforms: Option<Vec<String>>,
    #[vndb_field(nested)]
    pub image: Option<VnImage>,

    pub length: Option<i32>,
    pub length_minutes: Option<i32>,
    pub length_votes: Option<i32>,

    pub description: Option<String>,

    pub average: Option<i32>,
    #[vndb_sort]
    pub rating: Option<i32>,
    #[vndb_sort]
    pub votecount: Option<i32>,

    #[vndb_field(nested)]
    pub screenshots: Option<Vec<VnScreenshot>>,
    #[vndb_field(nested)]
    pub relations: Option<Vec<VnRelation>>,
    #[vndb_field(nested)]
    pub tags: Option<Vec<VnTag>>,
    #[vndb_field(nested)]
    pub developers: Option<Vec<Producer>>,
    #[vndb_field(nested)]
    pub editions: Option<Vec<VnEdition>>,
    #[vndb_field(nested)]
    pub staff: Option<Vec<VnStaffEntry>>,
    #[vndb_field(nested)]
    pub va: Option<Vec<VnVaEntry>>,
    #[vndb_field(nested)]
    pub extlinks: Option<Vec<ExtLink>>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnTitle {
    pub lang: Option<String>,
    pub title: Option<String>,
    pub latin: Option<String>,
    pub official: Option<bool>,
    pub main: Option<bool>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnImage {
    pub id: Option<String>,
    pub url: Option<String>,
    pub dims: Option<Vec<i32>>,
    pub sexual: Option<f64>,
    pub violence: Option<f64>,
    pub votecount: Option<i32>,
    pub thumbnail: Option<String>,
    pub thumbnail_dims: Option<Vec<i32>>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnScreenshot {
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub image: VnImage,
    #[vndb_field(nested, boxed)]
    pub release: Option<Release>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnRelation {
    pub relation: Option<String>,
    pub relation_official: Option<bool>,
    #[serde(flatten)]
    #[vndb_field(flatten, boxed)]
    pub vn: Vn,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnTag {
    pub rating: Option<f64>,
    pub spoiler: Option<u8>,
    pub lie: Option<bool>,
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub tag: Option<Tag>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnEdition {
    pub eid: Option<i32>,
    pub lang: Option<String>,
    pub name: Option<String>,
    pub official: Option<bool>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnStaffEntry {
    pub eid: Option<i32>,
    pub role: Option<String>,
    pub note: Option<String>,
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub staff: Option<Staff>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnVaEntry {
    pub note: Option<String>,
    #[vndb_field(nested)]
    pub staff: Option<Staff>,
    #[vndb_field(nested, boxed)]
    pub character: Option<Character>,
}
