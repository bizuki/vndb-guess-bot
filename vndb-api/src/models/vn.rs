use serde::Deserialize;
use vndb_api_derive::VndbFieldsEnum;

use crate::models::{
    character::{Character, CharacterFields}, 
    common::{ExtLink, ExtLinkFields}, 
    producer::{Producer, ProducerFields}, 
    release::{Release, ReleaseFields}, 
    staff::{Staff, StaffFields}, 
    tag::{Tag, TagFields}
};

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct Vn {
    #[vndb_field(skip)]
    pub id: String,
    
    pub title: Option<String>,
    pub alttitle: Option<String>,
    pub titles: Option<Vec<VnTitle>>,
    pub aliases: Option<Vec<String>>,

    pub olang: Option<String>,
    
    pub devstatus: Option<u8>,
    pub released: Option<String>,
    
    pub languages: Option<Vec<String>>,
    
    pub platforms: Option<Vec<String>>,
    pub image: Option<VnImage>,

    pub length: Option<i32>,
    pub length_minutes: Option<i32>,
    pub length_votes: Option<i32>,

    pub description: Option<String>,

    pub average: Option<i32>,
    pub rating: Option<i32>,
    pub votecount: Option<i32>,

    pub screenshots: Option<Vec<VnScreenshot>>,
    pub relations: Option<Vec<VnRelation>>,
    pub tags: Option<Vec<VnTag>>,
    pub developers: Option<Vec<Producer>>,
    pub editions: Option<Vec<VnEdition>>,
    pub staff: Option<Vec<VnStaffEntry>>,
    pub va: Option<Vec<VnVaEntry>>,
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
    pub image: VnImage,
    pub release: Option<Release>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnRelation {
    pub relation: Option<String>,
    pub relation_official: Option<bool>,
    #[serde(flatten)]
    pub vn: Vn,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnTag {
    pub rating: Option<f64>,
    pub spoiler: Option<u8>,
    pub lie: Option<bool>,
    #[serde(flatten)]
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
    pub staff: Option<Staff>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct VnVaEntry {
    pub note: Option<String>,
    pub staff: Option<Staff>,
    pub character: Option<Character>,
}