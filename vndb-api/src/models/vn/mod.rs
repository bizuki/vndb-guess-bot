mod filter;
mod model;

pub use filter::VnFilters;
pub use model::*;

pub type VnQuery = crate::query::VndbQuery<VnFilters, VnFields, VnSort>;
pub type VnResult = crate::query::VndbQueryResponse<Vn>;
