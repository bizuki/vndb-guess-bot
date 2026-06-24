mod filter;
mod model;

pub use filter::TraitFilters;
pub use model::*;

pub type TraitQueryBuilder<'client, Client> = crate::client::EndpointQueryBuilder<
    'client,
    Client,
    Trait,
    TraitFilters,
    TraitFields,
    TraitSort,
>;
pub type TraitQuery = crate::query::VndbQuery<TraitFilters, TraitFields, TraitSort>;
pub type TraitResult = crate::query::VndbQueryResponse<Trait>;
