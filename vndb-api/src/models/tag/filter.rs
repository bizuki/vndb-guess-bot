use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum TagFilters {
    #[vndb_filter(value = crate::ids::TagId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::StringValue)]
    Category,
}
