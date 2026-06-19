use vndb_api_derive::VndbSortEnum;

#[derive(VndbSortEnum)]
#[vndb_sort(field = "search_rank")]
#[vndb_sort(field = "random")]
pub struct Struct {
    #[vndb_sort]
    id: String,
    #[vndb_sort(rename = "vote_count")]
    votecount: i32,
    title: String,
    #[vndb_sort(nested)]
    nested: Nested,
}

pub struct Nested;

fn main() {
    let _ = StructSort::Id;
    let _ = StructSort::Votecount;
    let _ = StructSort::Nested;
    let _ = StructSort::SearchRank;
    let _ = StructSort::Random;
}
