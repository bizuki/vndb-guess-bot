use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::{EndpointQueryBuilder, VndbEndpoint},
    models::auth::AuthInfo,
    models::character::{Character, CharacterFields, CharacterFilters, CharacterSort},
    models::producer::{Producer, ProducerFields, ProducerFilters, ProducerSort},
    models::quote::{Quote, QuoteFields, QuoteFilters, QuoteSort},
    models::release::{Release, ReleaseFields, ReleaseFilters, ReleaseSort},
    models::staff::{Staff, StaffFields, StaffFilters, StaffSort},
    models::stats::VndbStats,
    models::tag::{Tag, TagFields, TagFilters, TagSort},
    models::traits::{Trait, TraitFields, TraitFilters, TraitSort},
    models::user::{UserLookupQuery, UserLookupResponse},
    models::vn::{Vn, VnFields, VnFilters, VnSort},
    query::{VndbQuery, VndbQueryResponse},
};

pub use crate::{
    models::character::{CharacterQuery, CharacterResult},
    models::producer::{ProducerQuery, ProducerResult},
    models::quote::{QuoteQuery, QuoteResult},
    models::release::{ReleaseQuery, ReleaseResult},
    models::staff::{StaffQuery, StaffResult},
    models::tag::{TagQuery, TagResult},
    models::traits::{TraitQuery, TraitResult},
    models::vn::{VnQuery, VnResult},
};

#[allow(async_fn_in_trait)]
pub trait VndbClient {
    type Error;

    async fn schema(&self) -> Result<serde_json::Value, Self::Error>;

    async fn stats(&self) -> Result<VndbStats, Self::Error>;

    async fn user(&self, query: UserLookupQuery) -> Result<UserLookupResponse, Self::Error>;

    async fn authinfo(&self) -> Result<AuthInfo, Self::Error>;

    async fn execute_query<Model, Filter, Field, Sort>(
        &self,
        endpoint: VndbEndpoint,
        query: VndbQuery<Filter, Field, Sort>,
    ) -> Result<VndbQueryResponse<Model>, Self::Error>
    where
        Model: DeserializeOwned,
        VndbQuery<Filter, Field, Sort>: Serialize;

    fn vn(&self) -> EndpointQueryBuilder<'_, Self, Vn, VnFilters, VnFields, VnSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Vn)
    }

    fn release(
        &self,
    ) -> EndpointQueryBuilder<'_, Self, Release, ReleaseFilters, ReleaseFields, ReleaseSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Release)
    }

    fn producer(
        &self,
    ) -> EndpointQueryBuilder<'_, Self, Producer, ProducerFilters, ProducerFields, ProducerSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Producer)
    }

    fn character(
        &self,
    ) -> EndpointQueryBuilder<'_, Self, Character, CharacterFilters, CharacterFields, CharacterSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Character)
    }

    fn staff(&self) -> EndpointQueryBuilder<'_, Self, Staff, StaffFilters, StaffFields, StaffSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Staff)
    }

    fn tag(&self) -> EndpointQueryBuilder<'_, Self, Tag, TagFilters, TagFields, TagSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Tag)
    }

    fn traits(&self) -> EndpointQueryBuilder<'_, Self, Trait, TraitFilters, TraitFields, TraitSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Trait)
    }

    fn quote(&self) -> EndpointQueryBuilder<'_, Self, Quote, QuoteFilters, QuoteFields, QuoteSort>
    where
        Self: Sized,
    {
        EndpointQueryBuilder::new(self, VndbEndpoint::Quote)
    }
}
