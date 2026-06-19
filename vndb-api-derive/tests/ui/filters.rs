use vndb_api_derive::VndbFiltersEnum;

include!("../support/vndb_api_stub.rs");

#[derive(VndbFiltersEnum)]
pub enum StructFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    Id,
    #[vndb_filter(nested)]
    Nested(NestedFilters),
}

#[derive(VndbFiltersEnum)]
pub enum NestedFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    FieldA,
}

fn main() {
    let _ = StructFilters::Id;
    let _ = StructFilters::Nested(NestedFilters::FieldA);
}
