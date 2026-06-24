use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::{VndbClient, VndbEndpoint},
    filter::VndbFilter,
    ids::UserId,
    query::{
        request::{combine_filters, FilterCombination},
        QueryParams, VndbQuery, VndbQueryResponse,
    },
};

pub struct EndpointQueryBuilder<'client, Client, Model, Filter, Field, Sort>
where
    Client: VndbClient + ?Sized,
{
    client: &'client Client,
    endpoint: VndbEndpoint,
    filters: Option<VndbFilter<Filter>>,
    fields: Vec<Field>,
    sort: Option<Sort>,
    params: QueryParams,
    model: PhantomData<Model>,
}

impl<'client, Client, Model, Filter, Field, Sort>
    EndpointQueryBuilder<'client, Client, Model, Filter, Field, Sort>
where
    Client: VndbClient + ?Sized,
{
    pub(crate) fn new(client: &'client Client, endpoint: VndbEndpoint) -> Self {
        Self {
            client,
            endpoint,
            filters: None,
            fields: Vec::new(),
            sort: None,
            params: QueryParams::default(),
            model: PhantomData,
        }
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn fields(mut self, fields: impl IntoIterator<Item = Field>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn filter(mut self, filter: VndbFilter<Filter>) -> Self {
        self.filters = combine_existing_filter(self.filters, filter, FilterCombination::And);
        self
    }

    pub fn filters(mut self, filters: impl IntoIterator<Item = VndbFilter<Filter>>) -> Self {
        self.filters = combine_many_filters(self.filters, filters, FilterCombination::And);
        self
    }

    pub fn or_filter(mut self, filter: VndbFilter<Filter>) -> Self {
        self.filters = combine_existing_filter(self.filters, filter, FilterCombination::Or);
        self
    }

    pub fn or_filters(mut self, filters: impl IntoIterator<Item = VndbFilter<Filter>>) -> Self {
        self.filters = combine_many_filters(self.filters, filters, FilterCombination::Or);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn params(mut self, params: QueryParams) -> Self {
        self.params = params;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.params.reverse = true;
        self
    }

    pub fn results(mut self, results: usize) -> Self {
        self.params.results = results;
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        self.params.page = page;
        self
    }

    pub fn user(mut self, user: UserId) -> Self {
        self.params.user = Some(user);
        self
    }

    pub fn count(mut self) -> Self {
        self.params.count = true;
        self
    }

    pub fn compact_filters(mut self) -> Self {
        self.params.compact_filters = true;
        self
    }

    pub fn normalized_filters(mut self) -> Self {
        self.params.normalized_filters = true;
        self
    }

    pub fn build(self) -> VndbQuery<Filter, Field, Sort> {
        VndbQuery {
            filters: self.filters,
            fields: self.fields,
            sort: self.sort,
            params: self.params,
        }
    }

    pub async fn send(self) -> Result<VndbQueryResponse<Model>, Client::Error>
    where
        Model: DeserializeOwned,
        VndbQuery<Filter, Field, Sort>: Serialize,
    {
        let client = self.client;
        let endpoint = self.endpoint;
        let query = self.build();
        client.execute_query(endpoint, query).await
    }
}

fn combine_existing_filter<Filter>(
    existing: Option<VndbFilter<Filter>>,
    filter: VndbFilter<Filter>,
    combination: FilterCombination,
) -> Option<VndbFilter<Filter>> {
    combine_many_filters(existing, [filter], combination)
}

fn combine_many_filters<Filter>(
    existing: Option<VndbFilter<Filter>>,
    filters: impl IntoIterator<Item = VndbFilter<Filter>>,
    combination: FilterCombination,
) -> Option<VndbFilter<Filter>> {
    let incoming = combine_filters(filters, combination)?;

    Some(match existing {
        Some(existing) => match combination {
            FilterCombination::And => VndbFilter::and(vec![existing, incoming]),
            FilterCombination::Or => VndbFilter::or(vec![existing, incoming]),
        },
        None => incoming,
    })
}
