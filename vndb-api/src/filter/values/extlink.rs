use vndb_api_macros_support::filter::{IntoVndbFilterValue, VndbFilterValue, VndbFilterValueType};

use super::InvalidFilterValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtlinkSite(String);

impl ExtlinkSite {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidFilterValue> {
        let value = value.into();

        if value.is_empty() {
            Err(InvalidFilterValue::EmptyExtlinkSite)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtlinkUrl(String);

impl ExtlinkUrl {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidFilterValue> {
        let value = value.into();

        if value.is_empty() {
            Err(InvalidFilterValue::EmptyExtlinkUrl)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtlinkRemoteId {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtlinkFilterValue {
    Site(ExtlinkSite),
    Url(ExtlinkUrl),
    SiteId {
        site: ExtlinkSite,
        remote_id: ExtlinkRemoteId,
    },
}

impl ExtlinkFilterValue {
    pub fn site(site: ExtlinkSite) -> Self {
        Self::Site(site)
    }

    pub fn url(url: ExtlinkUrl) -> Self {
        Self::Url(url)
    }

    pub fn site_id(site: ExtlinkSite, remote_id: ExtlinkRemoteId) -> Self {
        Self::SiteId { site, remote_id }
    }
}

impl VndbFilterValueType for ExtlinkFilterValue {}

impl IntoVndbFilterValue<ExtlinkFilterValue> for ExtlinkFilterValue {
    fn into_vndb_filter_value(self) -> VndbFilterValue {
        match self {
            Self::Site(site) => VndbFilterValue::String(site.0),
            Self::Url(url) => VndbFilterValue::String(url.0),
            Self::SiteId { site, remote_id } => VndbFilterValue::Array(vec![
                VndbFilterValue::String(site.0),
                match remote_id {
                    ExtlinkRemoteId::Integer(value) => VndbFilterValue::Integer(value),
                    ExtlinkRemoteId::String(value) => VndbFilterValue::String(value),
                },
            ]),
        }
    }
}
