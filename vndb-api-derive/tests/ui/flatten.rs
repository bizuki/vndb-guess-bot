use vndb_api_derive::{VndbFieldsEnum, VndbFiltersEnum, VndbSortEnum};

include!("../support/vndb_api_stub.rs");

#[derive(VndbFieldsEnum)]
pub struct Child {
    field_a: String,
    field_b: String,
}

#[derive(VndbFiltersEnum)]
pub enum ChildFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    FieldA,
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    FieldB,
}

#[derive(VndbFieldsEnum, VndbSortEnum)]
#[vndb_sort(field = "field_a")]
pub struct Struct {
    #[vndb_field(flatten)]
    child: Child,
    #[vndb_field(nested)]
    #[vndb_sort(nested)]
    nested: Child,
}

#[derive(VndbFiltersEnum)]
pub enum StructFilters {
    #[vndb_filter(flatten)]
    Child(ChildFilters),
    #[vndb_filter(nested)]
    Nested(ChildFilters),
}

fn main() {
    let _ = StructFields::Child(ChildFields::FieldA);
    let _: StructFields = ChildFields::FieldB.into();
    let _ = StructFields::Nested(ChildFields::FieldA);

    let _ = StructFilters::Child(ChildFilters::FieldA);
    let _: StructFilters = ChildFilters::FieldB.into();
    let _ = StructFilters::Nested(ChildFilters::FieldA);

    let _ = StructSort::Nested;
    let _ = StructSort::FieldA;
}
