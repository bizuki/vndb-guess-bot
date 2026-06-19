use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum TraitFilters {
    #[vndb_filter(value = crate::ids::TraitId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
}
