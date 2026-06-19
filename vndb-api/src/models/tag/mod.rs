mod filter;
mod model;

pub use filter::TagFilters;
pub use model::*;

pub type TagQuery = crate::query::VndbQuery<TagFilters, TagFields, TagSort>;
pub type TagResult = crate::query::VndbQueryResponse<Tag>;
