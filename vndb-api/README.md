# vndb-api

`vndb-api` is a typed Rust client and model crate for the VNDB Kana API.

It provides:

- typed VNDB response models;
- endpoint query aliases such as `VnQuery`, `ReleaseQuery`, and `CharacterQuery`;
- typed field and sort selectors;
- typed filter descriptors that only accept compatible Rust values and operators;
- a transport-independent `VndbClient` trait;
- optional `reqwest` and mock client implementations.

## API Coverage

Source: [VNDB Kana API docs](https://api.vndb.org/kana).

- ~~Token authentication through the `Authorization: Token ...` header.~~
- ~~`GET /schema` as `client.schema() -> serde_json::Value`.~~
- ~~`GET /stats` as `client.stats()`.~~
- ~~`GET /user` as `client.user(UserLookupQuery)`.~~
- ~~`GET /authinfo` as `client.authinfo()`.~~
- ~~Database query request serialization: `filters`, `fields`, `sort`, `reverse`, `results`, `page`, `user`, `count`, `compact_filters`, and `normalized_filters`.~~
- ~~Database query response deserialization: `results`, `more`, `count`, `compact_filters`, and `normalized_filters`.~~
- ~~Typed field selectors with dot nesting and brace grouping.~~
- ~~Typed flat sort selectors.~~
- ~~Typed filter descriptors with equality, ordering, nullable, integer-boolean, nested, and boolean-composition support.~~
- ~~Typed VNDB IDs and special filter values for tags, traits, labels, external links, birthdays, and resolutions.~~
- ~~`POST /vn`.~~
- ~~`POST /release`.~~
- ~~`POST /producer`.~~
- ~~`POST /character`.~~
- ~~`POST /staff`.~~
- ~~`POST /tag`.~~
- ~~`POST /trait`.~~
- ~~`POST /quote`.~~
- ~~Optional `ReqwestVndbClient`.~~
- ~~Optional `MockVndbClient`.~~
- Compact filter string authoring/parsing as a first-class Rust API.
- `POST /ulist`.
- `GET /ulist_labels`.
- `PATCH /ulist/<id>`.
- `PATCH /rlist/<id>`.
- `DELETE /ulist/<id>`.
- `DELETE /rlist/<id>`.
- Model generation from `GET /schema`.

## TODO

- Add ulist functionality.
- Add model generation from schema.

## Feature Flags

Default features enable the real HTTP client with Rustls TLS and the mock client:

```toml
vndb-api = { path = "vndb-api" }
```

Available features:

```toml
default = ["reqwest-client", "rustls-tls", "mock-client"]

reqwest-client # enables ReqwestVndbClient
rustls-tls     # enables reqwest Rustls TLS support
native-tls     # enables reqwest native TLS support
http2          # enables reqwest HTTP/2 support
system-proxy   # enables reqwest system proxy support
mock-client    # enables MockVndbClient
```

For a models/query-only dependency:

```toml
vndb-api = { path = "vndb-api", default-features = false }
```

For the HTTP client without the mock client:

```toml
vndb-api = {
    path = "vndb-api",
    default-features = false,
    features = ["reqwest-client", "rustls-tls"]
}
```

## Clients

Use `ReqwestVndbClient` for the live API:

```rust
use vndb_api::client::{ReqwestVndbClient, VndbClient};
use vndb_api::models::vn::{VnFields, VnFilters, VnQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ReqwestVndbClient::default();

    let query = VnQuery::new(
        vec![VnFilters!(search).eq("ever17")],
        VnFields!(title, image.url),
    );

    let response = client.vn(query).await?;

    for vn in response.results {
        println!("{:?}", vn.title);
    }

    Ok(())
}
```

Use `MockVndbClient` in tests:

```rust
use serde_json::json;
use vndb_api::{
    client::{MockVndbClient, VndbClient, VndbEndpoint},
    filter::VndbFilter,
    models::vn::{VnFields, VnFilters, VnQuery},
};

#[tokio::test]
async fn records_vn_request() {
    let client = MockVndbClient::new();

    client
        .push_json_response(
            VndbEndpoint::Vn,
            json!({
                "results": [{ "id": "v17", "title": "Ever17" }],
                "more": false
            }),
        )
        .unwrap();

    let query = VnQuery::new(
        Vec::<VndbFilter<VnFilters>>::new(),
        VnFields!(title),
    );

    let response = client.vn(query).await.unwrap();
    assert_eq!(response.results[0].title.as_deref(), Some("Ever17"));
}
```

## Queries

Each endpoint module exports a query alias:

```rust
use vndb_api::models::vn::VnQuery;
use vndb_api::models::release::ReleaseQuery;
use vndb_api::models::character::CharacterQuery;
```

`VndbQuery::new(filters, fields)` stores typed filters and field selectors.
Sorts and request parameters are added fluently:

```rust
use vndb_api::filter::VndbFilter;
use vndb_api::models::vn::{VnFields, VnFilters, VnQuery, VnSort};
use vndb_api::query::QueryParams;

let query = VnQuery::new(
    vec![VnFilters!(released).gte("2020-01-01")],
    VnFields!(title, released, image.{url, dims}),
)
.with_sort(VnSort::Rating)
.with_params(QueryParams::new().with_results(25).with_count(true));
```

## Fields And Sorts

Field and sort selectors can be authored with enum-specific macros:

```rust
use vndb_api::models::vn::{VnFields, VnSort};

let fields = VnFields!(title, image.{url, dims}, screenshots.release.title);
let sorts = VnSort!(released, rating, searchrank);
```

Field selectors support comma lists, dot nesting, brace groups, and root-level
groups:

```rust
let fields = VnFields!(title, image.url, image{url, dims}, {released, rating});
```

Sort selectors are flat. Nested sort paths are rejected at compile time.

Without selector macros, build the generated enums directly:

```rust
use vndb_api::models::vn::{VnFields, VnImageFields, VnSort};

let fields = vec![
    VnFields::Title,
    VnFields::Image(VnImageFields::Url),
];

let sort = VnSort::Rating;
```

## Filters

Filter macros resolve one documented VNDB filter path and return a typed field
descriptor. You then build predicates through methods on that descriptor:

```rust
use vndb_api::filter::VndbFilter;
use vndb_api::models::{release::ReleaseFilters, vn::VnFilters};

let search = VnFilters!(search).eq("nitroplus");
let long = VnFilters!(length).gt(30);
let unreleased = VnFilters!(released).is_null();
let recent_release = VnFilters!(release.released).gte("2020-01-01");

let filter = VndbFilter::and(vec![search, long, unreleased, recent_release]);
```

The available methods depend on the VNDB filter metadata:

- equality filters expose `.eq(value)` and `.ne(value)`;
- ordered filters also expose `.gt(value)`, `.gte(value)`, `.lt(value)`, and `.lte(value)`;
- nullable filters also expose `.is_null()` and `.is_not_null()`;
- integer-boolean filters expose `.is_true()` and `.is_false()`;
- nested filters expose `.matches(child_filter)` and `.not_matches(child_filter)`.

Special VNDB filter values use dedicated Rust types instead of raw JSON arrays:

```rust
use vndb_api::filter::{SpoilerLevel, TagFilterValue, TagLevel};
use vndb_api::ids::TagId;
use vndb_api::models::vn::VnFilters;

let tag = TagFilterValue::rated(
    TagId::try_from("g505").unwrap(),
    SpoilerLevel::Two,
    TagLevel::new(1.2).unwrap(),
);

let filter = VnFilters!(tag).eq(tag);
```

Without filter macros, use the generated associated constructors:

```rust
use vndb_api::filter::VndbFilter;
use vndb_api::models::{release::ReleaseFilters, vn::VnFilters};

let released = ReleaseFilters::released().gte("2020-01-01");
let nested = VnFilters::release().matches(released);

let filter = VndbFilter::and(vec![
    VnFilters::search().eq("ever17"),
    nested,
]);
```

## Users, Stats, Auth, And Schema

Simple GET endpoints are available through the same client trait:

```rust
use vndb_api::client::VndbClient;
use vndb_api::ids::UserId;
use vndb_api::models::user::{UserFields, UserLookup, UserLookupQuery};

async fn example<C: VndbClient>(client: C) -> Result<(), C::Error> {
    let stats = client.stats().await?;
    let schema = client.schema().await?;
    let auth = client.authinfo().await?;

    let users = client
        .user(UserLookupQuery::new(
            [UserLookup::id(UserId::try_from("u3").unwrap())],
            [UserFields::Username, UserFields::Lengthvotes],
        ))
        .await?;

    let _ = (stats, schema, auth, users);
    Ok(())
}
```

`client.schema()` intentionally returns `serde_json::Value`; there is no public
schema model module.

## Custom Transports

Implement `VndbClient` to use another HTTP stack or an application-specific
transport. The models, query types, filters, fields, and sorts are independent
from the bundled `reqwest` client.
