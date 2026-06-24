mod builder;
mod endpoint;
#[cfg(feature = "mock-client")]
mod mock;
#[cfg(feature = "reqwest-client")]
mod reqwest;
mod traits;

pub use builder::EndpointQueryBuilder;
pub use endpoint::VndbEndpoint;
#[cfg(feature = "mock-client")]
pub use mock::{MockVndbClient, MockVndbClientError, RecordedRequest};
#[cfg(feature = "reqwest-client")]
pub use reqwest::{ReqwestVndbClient, ReqwestVndbClientError, DEFAULT_BASE_URL};
pub use traits::{
    CharacterQuery, CharacterResult, ProducerQuery, ProducerResult, QuoteQuery, QuoteResult,
    ReleaseQuery, ReleaseResult, StaffQuery, StaffResult, TagQuery, TagResult, TraitQuery,
    TraitResult, VnQuery, VnResult, VndbClient,
};
