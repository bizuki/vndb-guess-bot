use std::{collections::VecDeque, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::{
    client::{endpoint::VndbEndpoint, traits::VndbClient},
    models::auth::AuthInfo,
    models::stats::VndbStats,
    models::user::{UserLookupQuery, UserLookupResponse},
    query::{VndbQuery, VndbQueryResponse},
};

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedRequest {
    pub endpoint: VndbEndpoint,
    pub path: String,
    pub body: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum MockVndbClientError {
    #[error("failed to serialize VNDB query body")]
    QueryBody(#[from] serde_json::Error),
    #[error("no mock response queued for {endpoint:?}")]
    NoResponse { endpoint: VndbEndpoint },
    #[error("mock response endpoint mismatch: expected {expected:?}, got {actual:?}")]
    EndpointMismatch {
        expected: VndbEndpoint,
        actual: VndbEndpoint,
    },
    #[error("failed to deserialize mock response for {endpoint:?}")]
    Response {
        endpoint: VndbEndpoint,
        #[source]
        source: serde_json::Error,
    },
    #[error("mock client lock was poisoned")]
    Poisoned,
}

#[derive(Debug, Clone)]
pub struct MockVndbClient {
    state: Arc<std::sync::Mutex<MockState>>,
}

impl Default for MockVndbClient {
    fn default() -> Self {
        Self {
            state: Arc::new(std::sync::Mutex::new(MockState::default())),
        }
    }
}

impl MockVndbClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_json_response(
        &self,
        endpoint: VndbEndpoint,
        response: serde_json::Value,
    ) -> Result<(), MockVndbClientError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| MockVndbClientError::Poisoned)?;
        state
            .responses
            .push_back(MockResponse { endpoint, response });
        Ok(())
    }

    pub fn requests(&self) -> Result<Vec<RecordedRequest>, MockVndbClientError> {
        let state = self
            .state
            .lock()
            .map_err(|_| MockVndbClientError::Poisoned)?;
        Ok(state.requests.clone())
    }

    pub fn clear_requests(&self) -> Result<(), MockVndbClientError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| MockVndbClientError::Poisoned)?;
        state.requests.clear();
        Ok(())
    }

    async fn respond_json<Model>(
        &self,
        endpoint: VndbEndpoint,
        path: String,
        body: serde_json::Value,
    ) -> Result<Model, MockVndbClientError>
    where
        Model: DeserializeOwned,
    {
        let response = self.pop_response(endpoint, path, body)?;

        serde_json::from_value(response)
            .map_err(|source| MockVndbClientError::Response { endpoint, source })
    }

    fn pop_response(
        &self,
        endpoint: VndbEndpoint,
        path: String,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, MockVndbClientError> {
        let response = {
            let mut state = self
                .state
                .lock()
                .map_err(|_| MockVndbClientError::Poisoned)?;
            state.requests.push(RecordedRequest {
                endpoint,
                path,
                body,
            });
            state
                .responses
                .pop_front()
                .ok_or(MockVndbClientError::NoResponse { endpoint })?
        };

        if response.endpoint != endpoint {
            return Err(MockVndbClientError::EndpointMismatch {
                expected: endpoint,
                actual: response.endpoint,
            });
        }

        Ok(response.response)
    }
}

impl VndbClient for MockVndbClient {
    type Error = MockVndbClientError;

    async fn schema(&self) -> Result<serde_json::Value, Self::Error> {
        self.respond_json(
            VndbEndpoint::Schema,
            VndbEndpoint::Schema.path().to_owned(),
            serde_json::Value::Null,
        )
        .await
    }

    async fn stats(&self) -> Result<VndbStats, Self::Error> {
        self.respond_json(
            VndbEndpoint::Stats,
            VndbEndpoint::Stats.path().to_owned(),
            serde_json::Value::Null,
        )
        .await
    }

    async fn user(&self, query: UserLookupQuery) -> Result<UserLookupResponse, Self::Error> {
        let queries = query
            .queries
            .iter()
            .map(|query| query.as_str())
            .collect::<Vec<_>>();
        let fields = query.selected_fields();

        self.respond_json(
            VndbEndpoint::User,
            VndbEndpoint::User.path().to_owned(),
            serde_json::json!({
                "q": queries,
                "fields": fields,
            }),
        )
        .await
    }

    async fn authinfo(&self) -> Result<AuthInfo, Self::Error> {
        self.respond_json(
            VndbEndpoint::AuthInfo,
            VndbEndpoint::AuthInfo.path().to_owned(),
            serde_json::Value::Null,
        )
        .await
    }

    async fn execute_query<Model, Filter, Field, Sort>(
        &self,
        endpoint: VndbEndpoint,
        query: VndbQuery<Filter, Field, Sort>,
    ) -> Result<VndbQueryResponse<Model>, Self::Error>
    where
        Model: DeserializeOwned,
        VndbQuery<Filter, Field, Sort>: Serialize,
    {
        let body = serde_json::to_value(&query)?;
        self.respond_json(endpoint, endpoint.path().to_owned(), body)
            .await
    }
}

#[derive(Debug, Default)]
struct MockState {
    requests: Vec<RecordedRequest>,
    responses: VecDeque<MockResponse>,
}

#[derive(Debug)]
struct MockResponse {
    endpoint: VndbEndpoint,
    response: serde_json::Value,
}
