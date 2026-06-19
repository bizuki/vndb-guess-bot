use vndb_api_derive::VndbFiltersEnum;

use crate::models::{staff::StaffFilters, vn::VnFilters};

#[derive(Debug, Clone, VndbFiltersEnum)]
pub enum CharacterFilters {
    #[vndb_filter(value = crate::ids::CharacterId, ordered)]
    Id,
    #[vndb_filter(value = crate::filter::StringValue)]
    Search,
    #[vndb_filter(value = crate::filter::StringValue)]
    Role,
    #[vndb_filter(value = crate::filter::StringValue)]
    SexSpoil,
    #[vndb_filter(value = crate::filter::StringValue)]
    GenderSpoil,
    #[vndb_filter(value = crate::filter::TraitFilterValue)]
    Trait,
    #[vndb_filter(value = crate::filter::TraitFilterValue)]
    Dtrait,
    #[vndb_filter(nested)]
    Seiyuu(StaffFilters),
    #[vndb_filter(value = crate::filter::StringValue)]
    BloodType,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Height,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Weight,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Bust,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Waist,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Hips,
    #[vndb_filter(value = crate::filter::StringValue, ordered, nullable)]
    Cup,
    #[vndb_filter(value = crate::filter::IntegerValue, ordered, nullable)]
    Age,
    #[vndb_filter(value = crate::filter::Birthday, nullable)]
    Birthday,
    #[vndb_filter(value = crate::filter::StringValue)]
    Sex,
    #[vndb_filter(value = crate::filter::StringValue)]
    Gender,
    #[vndb_filter(field = "vn", nested)]
    Vns(VnFilters),
}
