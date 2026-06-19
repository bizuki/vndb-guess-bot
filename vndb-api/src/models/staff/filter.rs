use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum StaffFilters {
    #[vndb_filter(value = crate::ids::StaffId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::StringValue)]
    Role,
    #[vndb_filter(value = crate::filter::ExtlinkFilterValue)]
    Extlink,
    #[vndb_filter(value = crate::filter::IntegerValue)]
    Aid,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Ismain,
    #[vndb_filter(value = crate::filter::StringValue)]
    Lang,
    #[vndb_filter(value = crate::filter::StringValue)]
    Gender,
}
