mod filter;
mod model;

pub use filter::ProducerFilters;
pub use model::*;

pub type ProducerQueryBuilder<'client, Client> = crate::client::EndpointQueryBuilder<
    'client,
    Client,
    Producer,
    ProducerFilters,
    ProducerFields,
    ProducerSort,
>;
pub type ProducerQuery = crate::query::VndbQuery<ProducerFilters, ProducerFields, ProducerSort>;
pub type ProducerResult = crate::query::VndbQueryResponse<Producer>;
