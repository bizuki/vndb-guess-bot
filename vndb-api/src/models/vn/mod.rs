mod filter;
mod model;

pub use filter::VnFilters;
pub use model::*;

pub type VnQueryBuilder<'client, Client> =
    crate::client::EndpointQueryBuilder<'client, Client, Vn, VnFilters, VnFields, VnSort>;
pub type VnQuery = crate::query::VndbQuery<VnFilters, VnFields, VnSort>;
pub type VnResult = crate::query::VndbQueryResponse<Vn>;
