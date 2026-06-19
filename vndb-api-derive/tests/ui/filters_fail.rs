use vndb_api_derive::VndbFiltersEnum;

include!("../support/vndb_api_stub.rs");

#[derive(VndbFiltersEnum)]
pub struct Struct {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    id: String,
}

#[derive(VndbFiltersEnum)]
pub enum MissingValueFilters {
    #[vndb_filter]
    Id,
}

#[derive(VndbFiltersEnum)]
pub enum BadNestedFilters {
    #[vndb_filter(nested)]
    Nested,
}

#[derive(VndbFiltersEnum)]
pub enum BadPayloadFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    Payload(MissingValueFilters),
}

fn main() {}
