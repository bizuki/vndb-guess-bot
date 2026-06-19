use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::{
    client::{
        endpoint::VndbEndpoint,
        traits::{
            CharacterQuery, CharacterResult, ProducerQuery, ProducerResult, QuoteQuery,
            QuoteResult, ReleaseQuery, ReleaseResult, StaffQuery, StaffResult, TagQuery, TagResult,
            TraitQuery, TraitResult, VnQuery, VnResult, VndbClient,
        },
    },
    models::auth::AuthInfo,
    models::stats::VndbStats,
    models::user::{UserLookupQuery, UserLookupResponse},
    query::{VndbQuery, VndbQueryResponse},
};

pub const DEFAULT_BASE_URL: &str = "https://api.vndb.org/kana";

#[derive(Debug, Error)]
pub enum ReqwestVndbClientError {
    #[error("failed to serialize VNDB query body")]
    QueryBody(#[from] serde_json::Error),
    #[error("VNDB HTTP request failed")]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct ReqwestVndbClient {
    client: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl Default for ReqwestVndbClient {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl ReqwestVndbClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: DEFAULT_BASE_URL.to_owned(),
            token: None,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_owned();
        self
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    async fn get<Model>(&self, endpoint: VndbEndpoint) -> Result<Model, ReqwestVndbClientError>
    where
        Model: DeserializeOwned,
    {
        let response = self
            .authorized(self.client.get(self.endpoint_url(endpoint)))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    async fn get_user(
        &self,
        query: UserLookupQuery,
    ) -> Result<UserLookupResponse, ReqwestVndbClientError> {
        let mut request = self.authorized(self.client.get(self.endpoint_url(VndbEndpoint::User)));

        for query in &query.queries {
            request = request.query(&[("q", query.as_str())]);
        }

        let fields = query.selected_fields();
        if !fields.is_empty() {
            request = request.query(&[("fields", fields.as_str())]);
        }

        let response = request.send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    async fn post<Model, Filter, Field, Sort>(
        &self,
        endpoint: VndbEndpoint,
        query: VndbQuery<Filter, Field, Sort>,
    ) -> Result<VndbQueryResponse<Model>, ReqwestVndbClientError>
    where
        Model: DeserializeOwned,
        VndbQuery<Filter, Field, Sort>: Serialize,
    {
        let body = serde_json::to_value(&query)?;
        let url = format!("{}/{}", self.base_url, endpoint.path());
        let request = self.authorized(self.client.post(url).json(&body));

        let response = request.send().await?.error_for_status()?;

        Ok(response.json().await?)
    }

    fn endpoint_url(&self, endpoint: VndbEndpoint) -> String {
        self.url(endpoint.path())
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    fn authorized(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            request.header(reqwest::header::AUTHORIZATION, format!("Token {token}"))
        } else {
            request
        }
    }
}

impl VndbClient for ReqwestVndbClient {
    type Error = ReqwestVndbClientError;

    async fn schema(&self) -> Result<serde_json::Value, Self::Error> {
        self.get(VndbEndpoint::Schema).await
    }

    async fn stats(&self) -> Result<VndbStats, Self::Error> {
        self.get(VndbEndpoint::Stats).await
    }

    async fn user(&self, query: UserLookupQuery) -> Result<UserLookupResponse, Self::Error> {
        self.get_user(query).await
    }

    async fn authinfo(&self) -> Result<AuthInfo, Self::Error> {
        self.get(VndbEndpoint::AuthInfo).await
    }

    async fn vn(&self, query: VnQuery) -> Result<VnResult, Self::Error> {
        self.post(VndbEndpoint::Vn, query).await
    }

    async fn release(&self, query: ReleaseQuery) -> Result<ReleaseResult, Self::Error> {
        self.post(VndbEndpoint::Release, query).await
    }

    async fn producer(&self, query: ProducerQuery) -> Result<ProducerResult, Self::Error> {
        self.post(VndbEndpoint::Producer, query).await
    }

    async fn character(&self, query: CharacterQuery) -> Result<CharacterResult, Self::Error> {
        self.post(VndbEndpoint::Character, query).await
    }

    async fn staff(&self, query: StaffQuery) -> Result<StaffResult, Self::Error> {
        self.post(VndbEndpoint::Staff, query).await
    }

    async fn tag(&self, query: TagQuery) -> Result<TagResult, Self::Error> {
        self.post(VndbEndpoint::Tag, query).await
    }

    async fn traits(&self, query: TraitQuery) -> Result<TraitResult, Self::Error> {
        self.post(VndbEndpoint::Trait, query).await
    }

    async fn quote(&self, query: QuoteQuery) -> Result<QuoteResult, Self::Error> {
        self.post(VndbEndpoint::Quote, query).await
    }
}
