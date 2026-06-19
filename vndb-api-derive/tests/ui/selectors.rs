use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

#[derive(VndbFieldsEnum)]
pub struct Root {
    title: String,
    #[vndb_field(nested)]
    child: Child,
    #[vndb_field(flatten)]
    flat: Flat,
}

#[derive(VndbFieldsEnum)]
pub struct Child {
    leaf_a: String,
    leaf_b: String,
}

#[derive(VndbFieldsEnum)]
pub struct Flat {
    flat_leaf: String,
}

#[derive(VndbSortEnum)]
#[vndb_sort(field = "searchrank")]
pub struct Sortable {
    #[vndb_sort]
    id: String,
    #[vndb_sort(rename = "vote_count")]
    votecount: i32,
}

fn main() {
    let _ = RootFields!(title, child.{leaf_a, leaf_b});
    let _ = RootFields!(title, child{leaf_a, leaf_b});
    let _ = RootFields!({title, flat_leaf});
    let _ = SortableSort!(id, vote_count, searchrank);
}
