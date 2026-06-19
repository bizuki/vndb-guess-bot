use serde::Deserialize;
use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

use crate::{
    ids::{ReleaseId, VnId},
    models::common::{ExtLink, ExtLinkFields},
    models::{
        producer::{Producer, ProducerFields},
        vn::{Vn, VnFields, VnImage, VnImageFields},
    },
};

#[derive(Deserialize, Debug, VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Release {
    #[vndb_field(skip)]
    #[vndb_sort]
    pub id: ReleaseId,
    #[vndb_sort]
    pub title: Option<String>,
    pub alttitle: Option<String>,
    #[vndb_field(nested)]
    pub languages: Option<Vec<ReleaseLanguage>>,
    pub platforms: Option<Vec<String>>,
    #[vndb_field(nested)]
    pub media: Option<Vec<ReleaseMedia>>,
    #[vndb_field(nested)]
    pub vns: Option<Vec<ReleaseVnLink>>,
    #[vndb_field(nested)]
    pub producers: Option<Vec<ReleaseProducer>>,
    #[vndb_field(nested)]
    pub images: Option<Vec<ReleaseImage>>,
    #[vndb_sort]
    pub released: Option<String>,
    pub minage: Option<i32>,
    pub patch: Option<bool>,
    pub freeware: Option<bool>,
    pub uncensored: Option<bool>,
    pub official: Option<bool>,
    pub has_ero: Option<bool>,
    #[serde(deserialize_with = "deserialization::deserialize_resolution")]
    pub resolution: Option<ReleaseResolution>,
    pub engine: Option<String>,
    pub voiced: Option<i32>,
    pub notes: Option<String>,
    pub gtin: Option<String>,
    pub catalog: Option<String>,
    #[vndb_field(nested)]
    pub extlinks: Option<Vec<ExtLink>>,
}

mod deserialization {
    use serde::{de, Deserialize};

    #[derive(Deserialize)]
    #[serde(untagged)]
    #[allow(dead_code)]
    enum ReleaseResolution {
        NonStandard(String),
        Standard([u32; 2]),
    }

    pub fn deserialize_resolution<'de, D>(
        deserializer: D,
    ) -> Result<Option<super::ReleaseResolution>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let resolution: Option<ReleaseResolution> = de::Deserialize::deserialize(deserializer)?;

        Ok(resolution.map(|res| match res {
            ReleaseResolution::NonStandard(_) => super::ReleaseResolution::NonStandard,
            ReleaseResolution::Standard([width, height]) => {
                super::ReleaseResolution::Standard { width, height }
            }
        }))
    }
}

#[derive(Debug)]
pub enum ReleaseResolution {
    NonStandard,
    Standard { width: u32, height: u32 },
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ReleaseLanguage {
    pub lang: Option<String>,
    pub title: Option<String>,
    pub latin: Option<String>,
    pub mtl: Option<bool>,
    pub main: Option<bool>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ReleaseMedia {
    pub medium: Option<String>,
    pub qty: Option<i32>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ReleaseVnLink {
    pub rtype: Option<String>,
    #[serde(flatten)]
    #[vndb_field(flatten, boxed)]
    pub vn: Option<Vn>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ReleaseProducer {
    pub developer: Option<bool>,
    pub publisher: Option<bool>,
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub producer: Option<Producer>,
}

#[derive(Deserialize, Debug, VndbFieldsEnum)]
pub struct ReleaseImage {
    #[serde(flatten)]
    #[vndb_field(flatten)]
    pub image_fields: VnImage,
    pub r#type: Option<String>,
    pub vn: Option<VnId>,
    pub languages: Option<Vec<String>>,
    pub photo: Option<bool>,
}
