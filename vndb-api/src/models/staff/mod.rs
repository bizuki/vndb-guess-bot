mod filter;
mod model;

pub use filter::StaffFilters;
pub use model::*;

pub type StaffQuery = crate::query::VndbQuery<StaffFilters, StaffFields, StaffSort>;
pub type StaffResult = crate::query::VndbQueryResponse<Staff>;
