mod filter;
mod model;

pub use filter::ReleaseFilters;
pub use model::*;

pub type ReleaseQueryBuilder<'client, Client> = crate::client::EndpointQueryBuilder<
    'client,
    Client,
    Release,
    ReleaseFilters,
    ReleaseFields,
    ReleaseSort,
>;
pub type ReleaseQuery = crate::query::VndbQuery<ReleaseFilters, ReleaseFields, ReleaseSort>;
pub type ReleaseResult = crate::query::VndbQueryResponse<Release>;
