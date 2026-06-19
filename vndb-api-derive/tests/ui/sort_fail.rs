use vndb_api_derive::VndbSortEnum;

#[derive(VndbSortEnum)]
pub struct Struct {
    #[vndb_sort]
    id: String,
    title: String,
    #[vndb_sort(nested)]
    nested: Nested,
}

pub struct Nested;

fn main() {
    let _ = StructSort::Title;
    let _ = StructSort::Nested(NestedSort::FieldA);
}
