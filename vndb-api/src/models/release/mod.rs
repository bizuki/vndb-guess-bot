mod filter;
mod model;

pub use filter::ReleaseFilters;
pub use model::*;

pub type ReleaseQuery = crate::query::VndbQuery<ReleaseFilters, ReleaseFields, ReleaseSort>;
pub type ReleaseResult = crate::query::VndbQueryResponse<Release>;
