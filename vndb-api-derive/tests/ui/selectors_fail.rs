use vndb_api_derive::{VndbFieldsEnum, VndbSortEnum};

#[derive(VndbFieldsEnum)]
pub struct Root {
    title: String,
    #[vndb_field(nested)]
    child: Child,
}

#[derive(VndbFieldsEnum)]
pub struct Child {
    leaf: String,
}

#[derive(VndbSortEnum)]
pub struct Sortable {
    #[vndb_sort]
    id: String,
}

fn main() {
    let _ = RootFields!(missing);
    let _ = RootFields!(child);
    let _ = RootFields!(title,);
    let selector = "title";
    let _ = RootFields!(selector);
    let _ = SortableSort!(id.leaf);
    let _ = RootFields!("title");
    let _ = SortableSort!("id");
}
