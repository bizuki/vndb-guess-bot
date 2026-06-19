use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum ProducerFilters {
    #[vndb_filter(value = crate::ids::ProducerId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::ExtlinkFilterValue)]
    Extlink,
    #[vndb_filter(value = crate::filter::StringValue)]
    Lang,
    #[vndb_filter(field = "type", value = crate::filter::StringValue)]
    Type,
}
