# vndb-api-derive

`vndb-api-derive` contains procedural macros for VNDB field, filter, and sort
selectors.

It provides three derives:

```rust
#[derive(VndbFieldsEnum)]
#[derive(VndbFiltersEnum)]
#[derive(VndbSortEnum)]
```

Each derive also emits an enum-specific token macro:

```rust
VnFields!(title, image.{url, dims});
VnSort!(released, rating);
CharacterFilters!(age).gt(18);
```

`VndbFieldsEnum` and `VndbSortEnum` generate enums from model structs.
`VndbFiltersEnum` is different: it is implemented for user-written filter enums,
so the VNDB filter table stays explicit rather than being spread across model
field attributes.

## Field selectors

`VndbFieldsEnum` generates `<StructName>Fields`.

Plain struct fields become leaf variants by default:

```rust
#[derive(VndbFieldsEnum)]
pub struct Vn {
    title: Option<String>,
    released: Option<String>,
}

VnFields::Title.to_string();
// "title"
```

Nested field collection is explicit. Add `#[vndb_field(nested)]` when the caller
should be able to select subfields:

```rust
#[derive(VndbFieldsEnum)]
pub struct Vn {
    #[vndb_field(nested)]
    image: Option<VnImage>,
}

#[derive(VndbFieldsEnum)]
pub struct VnImage {
    url: Option<String>,
    thumbnail: Option<String>,
}

VnFields::Image(VnImageFields::Url).to_string();
// "image{url}"
```

The generated nested field variant accepts one `ChildFields` value. Request
multiple child fields by passing multiple selectors to the generated macro; it
returns one enum value per selected leaf path.

Flattened fields delegate to a child selector without adding the parent path:

```rust
#[derive(VndbFieldsEnum)]
pub struct VnScreenshot {
    #[serde(flatten)]
    #[vndb_field(flatten)]
    image: VnImage,
}

VnScreenshotFields::Image(VnImageFields::Url).to_string();
// "url"
```

When a child enum is flattened only once in a parent enum, the derive also
generates `From<ChildFields>` for the parent enum:

```rust
let field: VnScreenshotFields = VnImageFields::Url.into();
```

The generated `VnFields!` macro parses token selectors at build time and returns
a typed vector:

```rust
let fields = VnFields!(title, image.{url, dims}, screenshots.release.title);
```

Supported selector forms are comma lists, dot nesting, brace groups with or
without a dot before the braces, and root-level groups:

```rust
VnFields!(title, image.url, image{url, dims}, image.{url, dims}, {title, released});
```

The macro normalizes grouped selectors into leaf paths, validates those paths
against `VnFields`, and returns one typed enum value per leaf path. Old
string-literal selector syntax is intentionally rejected.

## Filter selectors

`VndbFiltersEnum` is derived on a filter enum that you define yourself. The
derive implements `Display`, `Serialize`, path construction, and the hidden
typed descriptor helpers used by the enum-specific filter macro.

```rust
use vndb_api::filter::{IntegerBooleanValue, StringValue};

pub struct Vn;

#[derive(VndbFiltersEnum)]
pub enum VnFilters {
    #[vndb_filter(value = StringValue, ordered)]
    Id,
    #[vndb_filter(value = StringValue)]
    Search,
    #[vndb_filter(value = IntegerBooleanValue)]
    HasDescription,
    #[vndb_filter(nested)]
    Staff(StaffFilters),
}

VnFilters::Id.to_string();
// "id"

VnFilters::Staff(StaffFilters::Id).to_string();
// "staff{id}"
```

Every leaf filter variant must specify a `value = TypePath` marker. This is what
lets the generated constructors return descriptors whose methods only accept the
correct Rust value type. Use `ordered` for filters that accept ordering
operators and `nullable` for filters that accept `null`.

Nested filter variants accept one child filter enum, not a vector. A filter path
expands to one child field at a time. Recursive filter enum shapes can use
`Box<ChildFilters>` payloads.

Flattened filters work like flattened fields: they carry one child filter and
serialize only the child path:

```rust
#[derive(VndbFiltersEnum)]
pub enum ScreenshotFilters {
    #[vndb_filter(flatten)]
    Image(ImageFilters),
}

ScreenshotFilters::Image(ImageFilters::Url).to_string();
// "url"
```

The derive emits associated constructors and an enum-specific macro with the
same name as the filter enum. Both forms return typed field descriptors:

```rust
use vndb_api::models::character::CharacterFilters;

let name = CharacterFilters!(name).eq("name");
let adult = CharacterFilters!(age).gt(18);
let missing_birthday = CharacterFilters!(birthday).is_null();

let same_name = CharacterFilters::name().eq("name");
```

Equality fields expose `.eq(...)` and `.ne(...)`. Ordered fields additionally
expose `.gt(...)`, `.gte(...)`, `.lt(...)`, and `.lte(...)`. Nullable fields
add `.is_null()` and `.is_not_null()`. VNDB integer-boolean fields add
`.is_true()` and `.is_false()`, serializing to `= 1` and `!= 1`. Nested fields
can match a child filter:

```rust
let released = ReleaseFilters!(released).gte("2020-01-01");
let vn_filter = VnFilters!(release).matches(released);
```

Nested leaf paths are also supported:

```rust
let filter = VnFilters!(release.released).gte("2020-01-01");
```

Nested leaf macro arms delegate to the child filter enum macro, so the child
filter enum must also be in scope when using nested shorthand.

Rust keywords in VNDB field names use raw identifiers:

```rust
let filter = ProducerFilters!(r#type).eq("co");
```

The macro resolves documented VNDB filters only. Fetchable response fields that
are not filters, such as `Producer.description`, fail to compile. Full boolean
filter-expression parsing is intentionally not part of this macro; compose
predicates with `VndbFilter::and(...)` and `VndbFilter::or(...)`.

## Sort selectors

`VndbSortEnum` generates `<StructName>Sort`.

Sort fields are collected from `#[vndb_sort]` annotations and the generated enum
is always flat. A nested sort field becomes a plain sort variant with no child
payload:

```rust
#[derive(VndbSortEnum)]
#[vndb_sort(field = "search_rank")]
pub struct Vn {
    #[vndb_sort]
    id: String,
    #[vndb_sort(nested)]
    image: Option<VnImage>,
}

VnSort::Id.to_string();
// "id"

VnSort::Image.to_string();
// "image"

VnSort::SearchRank.to_string();
// "search_rank"
```

Use repeated top-level `#[vndb_sort(field = "...")]` attributes for sortable
fields that do not exist on the Rust model struct. Flattened fields do not
generate sort variants for their child fields; list those child paths with
`#[vndb_sort(field = "...")]` when they are sortable.

The generated `VnSort!` macro uses the same token comma-list parser as field
macros, but sort selectors are flat:

```rust
let sorts = VnSort!(released, rating, searchrank);
```

Dots and brace groups are rejected for sorts.

## Attributes

Field selectors use `#[vndb_field(...)]` on model fields. Filter selectors use
`#[vndb_filter(...)]` on variants of a user-defined filter enum. Sort selectors
use `#[vndb_sort(...)]` on model fields or as top-level sort extras.

Supported options:

```rust
#[vndb_field(skip)]
```

Excludes a field from the generated field enum. This is useful for values that
can be present in responses but should not be requestable through the generated
selector.

```rust
#[vndb_field(nested)]
```

Treats the field as a nested selector and unwraps the Rust type to find the
child selector enum name.

```rust
#[vndb_field(flatten)]
```

Treats the field as a flattened selector and unwraps the Rust type to find the
child selector enum name. The generated selector carries the child enum but does
not serialize the parent field path.

```rust
#[vndb_field(nested, boxed)]
#[vndb_field(flatten, boxed)]
```

Stores the generated child selector as `Box<ChildFields>`. This is useful for
recursive model graphs where `ParentFields` can eventually contain
`ParentFields` again through nested or flattened child selectors. Fields whose
Rust type already contains `Box<T>` are boxed automatically.

```rust
#[vndb_field(rename = "type")]
```

Uses a custom serialized VNDB path while keeping the Rust field name for the enum
variant.

```rust
#[vndb_filter(value = crate::filter::IntegerValue, ordered)]
```

Allows ordering operators (`>`, `>=`, `<`, and `<=`) for a filter field. Filters
without this option only accept equality and inequality.

```rust
#[vndb_filter(nullable)]
```

Allows `null` as a filter value.

```rust
#[vndb_filter(value = crate::filter::IntegerValue)]
```

Overrides the inferred filter value marker used by typed filter descriptors.
The support crate provides only primitive markers:
`StringValue`, `IntegerValue`, `NumberValue`, `BooleanValue`, and
`IntegerBooleanValue`. VNDB-specific marker/value types such as
`TagFilterValue`, `Birthday`, or `Resolution` live in `vndb-api`.

Custom filter value types can be used by implementing
`VndbFilterValueType` for the marker type and `IntoVndbFilterValue<Marker>` for
the Rust values accepted by the descriptor. Derive users outside `vndb-api`
should depend on `vndb-api-macros-support`, because generated filter descriptor
code refers to that crate.

`integer_boolean` is for VNDB filters that are authored as booleans in Rust but
serialized as `0` or `1` on the wire:

```rust
#[vndb_filter(field = "has_description", value = crate::filter::IntegerBooleanValue)]
```

```rust
let filter = VnFilters!(has_description).is_true();
// serializes as ["has_description", "=", 1]
```

```rust
#[vndb_sort(field = "search_rank")]
```

Adds a flat sort variant for a path that is not present as a model field.

Filter enum variants use `field = "..."` when the VNDB path differs from the
Rust variant name:

```rust
#[vndb_filter(field = "type", value = crate::filter::StringValue)]
Type,

#[vndb_filter(field = "developer", nested)]
Developer(ProducerFilters),
```

## Type handling

Nested selectors unwrap these containers to find the child type:

```rust
Option<T>
Vec<T>
Box<T>
[T; N]
```

For example, `Option<Vec<ExtLink>>` with `#[vndb_field(nested)]` generates a
variant that accepts `ExtLinkFields`. If the unwrapped model field contains
`Box<T>`, the generated nested selector stores `Box<ChildFields>` as well.

Non-nested fields are always leaf fields, no matter what their Rust type is.
Custom enums and custom structs therefore need no special marker unless they
should expose nested or flattened selectors.

## Generated behavior

Generated field and sort enums derive `Debug, Clone`. User-written filter enums
should derive whatever traits they need. The macros implement:

```rust
serde::Serialize
std::fmt::Display
```

Calling `.to_string()` uses the standard `Display` implementation.

## Limitations

`VndbFieldsEnum` and `VndbSortEnum` support structs with named fields only.
`VndbFiltersEnum` supports enums only.

Only `Option`, `Vec`, `Box`, and arrays are unwrapped for struct fields marked
`nested` or `flatten`. Filter enum variants do not unwrap model types; they
carry the child filter enum directly.

The macros do not read `serde` attributes. Use `#[vndb_field(rename = "...")]`
for field selectors and `#[vndb_filter(field = "...")]` for filter variants
when the VNDB selector path differs from the Rust name.
