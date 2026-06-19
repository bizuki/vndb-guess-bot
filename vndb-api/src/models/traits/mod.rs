mod filter;
mod model;

pub use filter::TraitFilters;
pub use model::*;

pub type TraitQuery = crate::query::VndbQuery<TraitFilters, TraitFields, TraitSort>;
pub type TraitResult = crate::query::VndbQueryResponse<Trait>;
