mod filter;
mod model;

pub use filter::ProducerFilters;
pub use model::*;

pub type ProducerQuery = crate::query::VndbQuery<ProducerFilters, ProducerFields, ProducerSort>;
pub type ProducerResult = crate::query::VndbQueryResponse<Producer>;
