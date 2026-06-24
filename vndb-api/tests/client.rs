use serde_json::json;
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
            "filters": ["id", "=", "v17"],
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

#[test]
fn query_builder_supports_sort_and_query_params() {
    let client = MockVndbClient::new();
    let id = VnId::try_from("v17").unwrap();
    let query = client
        .vn()
        .filter(VnFilters!(id).eq(id))
        .field(VnFields::Title)
        .sort(VnSort::Rating)
        .results(5)
        .page(2)
        .reverse()
        .user(UserId::try_from("u3").unwrap())
        .count()
        .compact_filters()
        .normalized_filters()
        .build();

    assert_eq!(
        serde_json::to_value(&query).unwrap(),
        json!({
            "filters": ["id", "=", "v17"],
            "fields": "title",
            "sort": "rating",
            "reverse": true,
            "results": 5,
            "page": 2,
            "count": true,
            "compact_filters": true,
            "normalized_filters": true,
            "user": "u3"
        })
    );
}

#[test]
fn query_builder_clears_internal_state() {
    let client = MockVndbClient::new();
    let query = client
        .vn()
        .filter(VnFilters!(search).eq("ever17"))
        .field(VnFields::Title)
        .sort(VnSort::Rating)
        .params(
            QueryParams::new()
                .with_results(5)
                .with_page(2)
                .with_count(true),
        )
        .clear_filters()
        .clear_fields()
        .clear_sort()
        .clear_params()
        .build();

    assert_eq!(
        serde_json::to_value(&query).unwrap(),
        json!({
            "filters": [],
            "fields": "",
            "reverse": false,
            "results": 10,
            "page": 1,
            "count": false,
            "compact_filters": false,
            "normalized_filters": false
        })
    );
}

#[test]
fn query_builder_clears_individual_query_params() {
    let client = MockVndbClient::new();
    let query = client
        .vn()
        .field(VnFields::Title)
        .reverse()
        .results(5)
        .page(2)
        .user(UserId::try_from("u3").unwrap())
        .count()
        .compact_filters()
        .normalized_filters()
        .clear_reverse()
        .clear_results()
        .clear_page()
        .clear_user()
        .clear_count()
        .clear_compact_filters()
        .clear_normalized_filters()
        .build();

    assert_eq!(
        serde_json::to_value(&query).unwrap(),
        json!({
            "filters": [],
            "fields": "title",
            "reverse": false,
            "results": 10,
            "page": 1,
            "count": false,
            "compact_filters": false,
            "normalized_filters": false
        })
    );
}

#[test]
fn query_builder_clear_resets_every_mutable_field() {
    let client = MockVndbClient::new();
    let query = client
        .vn()
        .filter(VnFilters!(search).eq("ever17"))
        .field(VnFields::Title)
        .sort(VnSort::Rating)
        .reverse()
        .results(5)
        .page(2)
        .user(UserId::try_from("u3").unwrap())
        .count()
        .compact_filters()
        .normalized_filters()
        .clear()
        .build();

    assert_eq!(
        serde_json::to_value(&query).unwrap(),
        json!({
            "filters": [],
            "fields": "",
            "reverse": false,
            "results": 10,
            "page": 1,
            "count": false,
            "compact_filters": false,
            "normalized_filters": false
        })
    );
}

#[test]
fn query_builder_wraps_repeated_filters_with_and_or() {
    let client = MockVndbClient::new();
    let and_query = client
        .vn()
        .filter(VnFilters!(search).eq("ever17"))
        .filter(VnFilters!(released).gte("2000-01-01"))
        .field(VnFields::Title)
        .build();

    assert_eq!(
        serde_json::to_value(&and_query).unwrap()["filters"],
        json!([
            "and",
            ["search", "=", "ever17"],
            ["released", ">=", "2000-01-01"]
        ])
    );

    let or_query = client
        .vn()
        .or_filters([
            VnFilters!(search).eq("ever17"),
            VnFilters!(search).eq("remember11"),
        ])
        .field(VnFields::Title)
        .build();

    assert_eq!(
        serde_json::to_value(&or_query).unwrap()["filters"],
        json!([
            "or",
            ["search", "=", "ever17"],
            ["search", "=", "remember11"]
        ])
    );
}

#[test]
fn real_recursive_field_all_helpers_do_not_loop() {
    let fields = VnFields::all()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert!(fields.contains(&"title".to_owned()));
    assert!(fields.contains(&"image{url}".to_owned()));
    assert!(fields.contains(&"va{character{name}}".to_owned()));

    assert!(!fields.iter().any(|field| field.contains("vns{va")));
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

    let result = client.vn().field(VnFields::Title).send().await.unwrap();

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

    let error = client.vn().field(VnFields::Title).send().await.unwrap_err();

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
