use serde_json::json;
use vndb_api::filter::VndbFilter;
use vndb_api::{
    client::{MockVndbClient, MockVndbClientError, VndbClient, VndbEndpoint},
    ids::{UserId, VnId},
    models::{
        user::{UserFields, UserLookup, UserLookupQuery},
        vn::{VnFields, VnFilters, VnSort},
    },
    query::{QueryParams, VndbQuery},
};

#[test]
fn query_serializes_typed_filters_fields_sort_and_params() {
    let id = VnId::try_from("v17").unwrap();
    let query = VndbQuery::new(vec![VnFilters!(id).eq(id)], vec![VnFields::Title])
        .with_sort(VnSort::Rating)
        .with_params(QueryParams::new().with_results(5).with_count(true));

    assert_eq!(
        serde_json::to_value(&query).unwrap(),
        json!({
            "filters": [["id", "=", "v17"]],
            "fields": "title",
            "sort": "rating",
            "reverse": false,
            "results": 5,
            "page": 1,
            "count": true,
            "compact_filters": false,
            "normalized_filters": false
        })
    );
}

#[tokio::test]
async fn mock_client_returns_queued_response_and_records_request() {
    let client = MockVndbClient::new();

    client
        .push_json_response(
            VndbEndpoint::Vn,
            json!({
                "results": [
                    {
                        "id": "v17",
                        "title": "Ever17"
                    }
                ],
                "more": false,
                "count": 1
            }),
        )
        .unwrap();

    let result = client
        .vn(VndbQuery::new(
            Vec::<VndbFilter<VnFilters>>::new(),
            vec![VnFields::Title],
        ))
        .await
        .unwrap();

    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].id.as_ref(), "v17");
    assert_eq!(result.results[0].title.as_deref(), Some("Ever17"));
    assert_eq!(result.more, false);
    assert_eq!(result.count, Some(1));

    let requests = client.requests().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].endpoint, VndbEndpoint::Vn);
    assert_eq!(requests[0].body["fields"], json!("title"));
    assert_eq!(requests[0].body["filters"], json!([]));
}

#[tokio::test]
async fn mock_client_rejects_endpoint_mismatches() {
    let client = MockVndbClient::new();

    client
        .push_json_response(
            VndbEndpoint::Release,
            json!({
                "results": [],
                "more": false
            }),
        )
        .unwrap();

    let error = client
        .vn(VndbQuery::new(
            Vec::<VndbFilter<VnFilters>>::new(),
            vec![VnFields::Title],
        ))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        MockVndbClientError::EndpointMismatch {
            expected: VndbEndpoint::Vn,
            actual: VndbEndpoint::Release,
        }
    ));
}

#[tokio::test]
async fn mock_client_handles_simple_get_endpoints() {
    let client = MockVndbClient::new();

    client
        .push_json_response(
            VndbEndpoint::Stats,
            json!({
                "chars": 1,
                "producers": 2,
                "releases": 3,
                "staff": 4,
                "tags": 5,
                "traits": 6,
                "vn": 7
            }),
        )
        .unwrap();

    let stats = client.stats().await.unwrap();

    assert_eq!(stats.vn, 7);
    let requests = client.requests().unwrap();
    assert_eq!(requests[0].endpoint, VndbEndpoint::Stats);
    assert_eq!(requests[0].path, "stats");
    assert_eq!(requests[0].body, json!(null));
}

#[tokio::test]
async fn mock_client_records_user_lookup_query() {
    let client = MockVndbClient::new();

    client
        .push_json_response(
            VndbEndpoint::User,
            json!({
                "u3": {
                    "id": "u3",
                    "username": "ayo",
                    "lengthvotes": 2
                }
            }),
        )
        .unwrap();

    let users = client
        .user(UserLookupQuery::new(
            [UserLookup::id(UserId::try_from("u3").unwrap())],
            [UserFields::Lengthvotes],
        ))
        .await
        .unwrap();

    assert_eq!(users["u3"].as_ref().unwrap().username, "ayo");
    let requests = client.requests().unwrap();
    assert_eq!(requests[0].endpoint, VndbEndpoint::User);
    assert_eq!(
        requests[0].body,
        json!({
            "q": ["u3"],
            "fields": "lengthvotes"
        })
    );
}
