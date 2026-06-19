mod filter;
mod model;

pub use filter::CharacterFilters;
pub use model::*;

pub type CharacterQuery = crate::query::VndbQuery<CharacterFilters, CharacterFields, CharacterSort>;
pub type CharacterResult = crate::query::VndbQueryResponse<Character>;
