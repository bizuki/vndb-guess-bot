use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::VndbEndpoint,
    models::auth::AuthInfo,
    models::stats::VndbStats,
    models::user::{UserLookupQuery, UserLookupResponse},
    query::{VndbQuery, VndbQueryResponse},
};

pub use crate::{
    models::character::{CharacterQuery, CharacterQueryBuilder, CharacterResult},
    models::producer::{ProducerQuery, ProducerQueryBuilder, ProducerResult},
    models::quote::{QuoteQuery, QuoteQueryBuilder, QuoteResult},
    models::release::{ReleaseQuery, ReleaseQueryBuilder, ReleaseResult},
    models::staff::{StaffQuery, StaffQueryBuilder, StaffResult},
    models::tag::{TagQuery, TagQueryBuilder, TagResult},
    models::traits::{TraitQuery, TraitQueryBuilder, TraitResult},
    models::vn::{VnQuery, VnQueryBuilder, VnResult},
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

    fn vn(&self) -> VnQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        VnQueryBuilder::new(self, VndbEndpoint::Vn)
    }

    fn release(&self) -> ReleaseQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        ReleaseQueryBuilder::new(self, VndbEndpoint::Release)
    }

    fn producer(&self) -> ProducerQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        ProducerQueryBuilder::new(self, VndbEndpoint::Producer)
    }

    fn character(&self) -> CharacterQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        CharacterQueryBuilder::new(self, VndbEndpoint::Character)
    }

    fn staff(&self) -> StaffQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        StaffQueryBuilder::new(self, VndbEndpoint::Staff)
    }

    fn tag(&self) -> TagQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        TagQueryBuilder::new(self, VndbEndpoint::Tag)
    }

    fn traits(&self) -> TraitQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        TraitQueryBuilder::new(self, VndbEndpoint::Trait)
    }

    fn quote(&self) -> QuoteQueryBuilder<'_, Self>
    where
        Self: Sized,
    {
        QuoteQueryBuilder::new(self, VndbEndpoint::Quote)
    }
}
