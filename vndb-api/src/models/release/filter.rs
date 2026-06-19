use vndb_api_derive::VndbFiltersEnum;

use crate::models::{producer::ProducerFilters, vn::VnFilters};

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum ReleaseFilters {
    #[vndb_filter(value = crate::ids::ReleaseId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::StringValue, nullable)]
    Medium,
    #[vndb_filter(value = crate::filter::Resolution, ordered)]
    ResolutionAspect,
    #[vndb_filter(value = crate::filter::StringValue)]
    Rtype,
    #[vndb_filter(value = crate::filter::ExtlinkFilterValue)]
    Extlink,
    #[vndb_filter(value = crate::filter::StringValue)]
    Drm,
    #[vndb_filter(value = crate::filter::StringValue, nullable)]
    Image,
    #[vndb_filter(field = "lang", value = crate::filter::StringValue)]
    Lang,
    #[vndb_filter(field = "platform", value = crate::filter::StringValue)]
    Platform,
    #[vndb_filter(field = "vn", nested)]
    Vns(VnFilters),
    #[vndb_filter(field = "producer", nested)]
    Producers(ProducerFilters),
    #[vndb_filter(value = crate::filter::StringValue, ordered)]
    Released,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Minage,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Patch,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Freeware,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Uncensored,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Official,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    HasEro,
    #[vndb_filter(value = crate::filter::Resolution, ordered)]
    Resolution,
    #[vndb_filter(value = crate::filter::StringValue, nullable)]
    Engine,
    #[vndb_filter(value = crate::filter::IntegerValue, nullable)]
    Voiced,
}
