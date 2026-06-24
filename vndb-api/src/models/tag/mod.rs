mod filter;
mod model;

pub use filter::TagFilters;
pub use model::*;

pub type TagQueryBuilder<'client, Client> =
    crate::client::EndpointQueryBuilder<'client, Client, Tag, TagFilters, TagFields, TagSort>;
pub type TagQuery = crate::query::VndbQuery<TagFilters, TagFields, TagSort>;
pub type TagResult = crate::query::VndbQueryResponse<Tag>;
