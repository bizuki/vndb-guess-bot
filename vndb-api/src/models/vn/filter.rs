use vndb_api_derive::VndbFiltersEnum;

use crate::models::{
    character::CharacterFilters, producer::ProducerFilters, release::ReleaseFilters,
    staff::StaffFilters,
};

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum VnFilters {
    #[vndb_filter(value = crate::ids::VnId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    HasDescription,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    HasAnime,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    HasScreenshot,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    HasReview,
    #[vndb_filter(value = crate::filter::TagFilterValue)]
    Tag,
    #[vndb_filter(value = crate::filter::TagFilterValue)]
    Dtag,
    #[vndb_filter(value = crate::filter::LabelFilterValue)]
    Label,
    #[vndb_filter(nested)]
    Release(Box<ReleaseFilters>),
    #[vndb_filter(nested)]
    Character(Box<CharacterFilters>),
    #[vndb_filter(value = crate::filter::IntegerValue)]
    AnimeId,
    #[vndb_filter(value = crate::filter::StringValue)]
    Olang,
    #[vndb_filter(value = crate::filter::IntegerValue)]
    Devstatus,
    #[vndb_filter(value = crate::filter::StringValue, ordered, nullable)]
    Released,
    #[vndb_filter(field = "lang", value = crate::filter::StringValue)]
    Lang,
    #[vndb_filter(field = "platform", value = crate::filter::StringValue)]
    Platform,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered)]
    Length,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered)]
    Rating,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered)]
    Votecount,
    #[vndb_filter(field = "developer", nested)]
    Developer(ProducerFilters),
    #[vndb_filter(nested)]
    Staff(StaffFilters),
}
