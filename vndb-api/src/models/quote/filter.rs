use vndb_api_derive::VndbFiltersEnum;

use crate::models::{character::CharacterFilters, vn::VnFilters};

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum QuoteFilters {
    #[vndb_filter(value = crate::ids::QuoteId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::IntegerBooleanValue)]
    Random,
    #[vndb_filter(nested)]
    Vn(VnFilters),
    #[vndb_filter(nested)]
    Character(CharacterFilters),
}
