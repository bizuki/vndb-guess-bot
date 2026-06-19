use crate::{
    models::auth::AuthInfo,
    models::stats::VndbStats,
    models::user::{UserLookupQuery, UserLookupResponse},
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

    async fn vn(&self, query: VnQuery) -> Result<VnResult, Self::Error>;

    async fn release(&self, query: ReleaseQuery) -> Result<ReleaseResult, Self::Error>;

    async fn producer(&self, query: ProducerQuery) -> Result<ProducerResult, Self::Error>;

    async fn character(&self, query: CharacterQuery) -> Result<CharacterResult, Self::Error>;

    async fn staff(&self, query: StaffQuery) -> Result<StaffResult, Self::Error>;

    async fn tag(&self, query: TagQuery) -> Result<TagResult, Self::Error>;

    async fn traits(&self, query: TraitQuery) -> Result<TraitResult, Self::Error>;

    async fn quote(&self, query: QuoteQuery) -> Result<QuoteResult, Self::Error>;
}
