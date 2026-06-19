use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};
use vndb_api_macros_support::selector::VndbSelector;

use crate::{filter::VndbFilter, ids::UserId, query::params::QueryParams};

#[derive(Debug, Clone, Copy)]
pub enum NoSort {}

impl VndbSelector for NoSort {
    fn selector(&self) -> String {
        match *self {}
    }
}

#[derive(Debug, Clone)]
pub struct VndbQuery<Filter, Field, Sort = NoSort> {
    pub filters: Vec<VndbFilter<Filter>>,
    pub fields: Vec<Field>,
    pub sort: Option<Sort>,
    pub params: QueryParams,
}

impl<Filter, Field, Sort> VndbQuery<Filter, Field, Sort> {
    pub fn new(
        filters: impl IntoIterator<Item = VndbFilter<Filter>>,
        fields: impl IntoIterator<Item = Field>,
    ) -> Self {
        Self {
            filters: filters.into_iter().collect(),
            fields: fields.into_iter().collect(),
            sort: None,
            params: QueryParams::default(),
        }
    }

    pub fn with_sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn with_params(mut self, params: QueryParams) -> Self {
        self.params = params;
        self
    }
}

impl<Filter, Field, Sort> Serialize for VndbQuery<Filter, Field, Sort>
where
    VndbFilter<Filter>: Serialize,
    Field: VndbSelector,
    Sort: VndbSelector,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = join_selectors(&self.fields);
        let sort = self.sort.as_ref().map(VndbSelector::selector);
        let mut map = serializer.serialize_map(Some(request_field_count(&self.params, &sort)))?;

        map.serialize_entry("filters", &self.filters)?;
        map.serialize_entry("fields", &fields)?;

        if let Some(sort) = &sort {
            map.serialize_entry("sort", sort)?;
        }

        serialize_params(&mut map, &self.params)?;
        map.end()
    }
}

fn request_field_count(params: &QueryParams, sort: &Option<String>) -> usize {
    let base_fields = 8;

    base_fields + usize::from(sort.is_some()) + usize::from(params.user.is_some())
}

fn serialize_params<S>(map: &mut S, params: &QueryParams) -> Result<(), S::Error>
where
    S: SerializeMap,
{
    map.serialize_entry("reverse", &params.reverse)?;
    map.serialize_entry("results", &params.results)?;
    map.serialize_entry("page", &params.page)?;
    map.serialize_entry("count", &params.count)?;
    map.serialize_entry("compact_filters", &params.compact_filters)?;
    map.serialize_entry("normalized_filters", &params.normalized_filters)?;

    if let Some(user) = &params.user {
        serialize_user(map, user)?;
    }

    Ok(())
}

fn serialize_user<S>(map: &mut S, user: &UserId) -> Result<(), S::Error>
where
    S: SerializeMap,
{
    map.serialize_entry("user", user)
}

fn join_selectors<T>(items: &[T]) -> String
where
    T: VndbSelector,
{
    items
        .iter()
        .map(VndbSelector::selector)
        .collect::<Vec<_>>()
        .join(",")
}
