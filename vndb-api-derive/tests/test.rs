use serde_json::json;
use trybuild;
use vndb_api_derive::{VndbFieldsEnum, VndbFiltersEnum, VndbSortEnum};

include!("support/vndb_api_stub.rs");

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/primitives.rs");
    t.pass("tests/ui/required_primitives.rs");
    t.pass("tests/ui/many_primitives.rs");
    t.pass("tests/ui/complex_types.rs");
    t.pass("tests/ui/skip.rs");
    t.pass("tests/ui/custom_leaf_types.rs");
    t.pass("tests/ui/filters.rs");
    t.pass("tests/ui/flatten.rs");
    t.pass("tests/ui/sort.rs");
    t.pass("tests/ui/selectors.rs");
    t.compile_fail("tests/ui/unhandled_types.rs");
    t.compile_fail("tests/ui/skip_fail.rs");
    t.compile_fail("tests/ui/filters_fail.rs");
    t.compile_fail("tests/ui/flatten_fail.rs");
    t.compile_fail("tests/ui/sort_fail.rs");
    t.compile_fail("tests/ui/selectors_fail.rs");
}

mod to_string {
    use vndb_api_derive::VndbFieldsEnum;

    #[derive(VndbFieldsEnum)]
    #[allow(dead_code)]
    pub struct Simple {
        id: String,
        field: i32,
    }

    #[test]
    fn simple() {
        assert_eq!(SimpleFields::Field.to_string(), "field");

        assert_eq!(SimpleFields::Id.to_string(), "id");
    }

    #[derive(VndbFieldsEnum)]
    #[allow(dead_code)]
    pub struct Nested {
        #[vndb_field(nested)]
        field_a: Simple,
        #[vndb_field(nested)]
        field_b: Box<Nested>,
    }

    #[test]
    fn nested() {
        assert_eq!(
            NestedFields::FieldA(SimpleFields::Field).to_string(),
            "field_a{field}"
        );

        assert_eq!(
            NestedFields::FieldA(SimpleFields::Id).to_string(),
            "field_a{id}"
        );

        assert_eq!(
            NestedFields::FieldB(Box::new(NestedFields::FieldA(SimpleFields::Field))).to_string(),
            "field_b{field_a{field}}"
        );

        assert_eq!(
            NestedFields::FieldB(Box::new(NestedFields::FieldA(SimpleFields::Id))).to_string(),
            "field_b{field_a{id}}"
        );
    }
}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct Simple {
    id: String,
    field: i32,
}

#[test]
fn simple() {
    assert_eq!(SimpleFields::Field.to_string(), "field");

    assert_eq!(SimpleFields::Id.to_string(), "id");
}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct Nested {
    #[vndb_field(nested)]
    field_a: Simple,
    #[vndb_field(nested)]
    field_b: Box<Nested>,
}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct Flattened {
    #[vndb_field(flatten)]
    simple: Simple,
    #[vndb_field(nested)]
    nested: Simple,
}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct ExplicitBoxed {
    #[vndb_field(nested, boxed)]
    nested: Simple,
    #[vndb_field(flatten, boxed)]
    flattened: Simple,
}

#[test]
fn nested() {
    assert_eq!(
        NestedFields::FieldA(SimpleFields::Field).to_string(),
        "field_a{field}"
    );

    assert_eq!(
        NestedFields::FieldA(SimpleFields::Id).to_string(),
        "field_a{id}"
    );

    assert_eq!(
        NestedFields::FieldB(Box::new(NestedFields::FieldA(SimpleFields::Field))).to_string(),
        "field_b{field_a{field}}"
    );

    assert_eq!(
        NestedFields::FieldB(Box::new(NestedFields::FieldA(SimpleFields::Id))).to_string(),
        "field_b{field_a{id}}"
    );
}

#[test]
fn field_selector_macro() {
    let fields = NestedFields!(field_a.{field, id}, field_b.field_a.{id, field});
    let fields = fields.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        fields,
        vec![
            "field_a{field}",
            "field_a{id}",
            "field_b{field_a{id}}",
            "field_b{field_a{field}}"
        ]
    );
}

#[test]
fn field_selector_macro_supports_root_group_and_flattening() {
    let fields = FlattenedFields!({id, field, nested.{id, field}});
    let fields = fields.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(fields, vec!["id", "field", "nested{id}", "nested{field}"]);
}

#[test]
fn field_selector_macro_keeps_ungrouped_nested_paths_separate() {
    let fields = NestedFields!(field_a.field, field_a.id);
    let fields = fields.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(fields, vec!["field_a{field}", "field_a{id}"]);
}

#[test]
fn flatten_fields() {
    assert_eq!(
        FlattenedFields::Simple(SimpleFields::Field).to_string(),
        "field"
    );

    assert_eq!(
        FlattenedFields::Nested(SimpleFields::Field).to_string(),
        "nested{field}"
    );

    let field: FlattenedFields = SimpleFields::Id.into();

    assert_eq!(field.to_string(), "id");
}

#[test]
fn explicit_boxed_fields() {
    assert_eq!(
        ExplicitBoxedFields::Nested(Box::new(SimpleFields::Field)).to_string(),
        "nested{field}"
    );

    assert_eq!(
        ExplicitBoxedFields::Flattened(Box::new(SimpleFields::Id)).to_string(),
        "id"
    );

    let field: ExplicitBoxedFields = SimpleFields::Id.into();
    assert_eq!(field.to_string(), "id");

    let fields = ExplicitBoxedFields!(nested.id, field);
    let fields = fields.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(fields, vec!["nested{id}", "field"]);
}

#[allow(dead_code)]
pub struct FilterSimple;

#[derive(VndbFiltersEnum)]
pub enum FilterSimpleFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    Id,
}

#[derive(VndbFiltersEnum)]
pub enum FilterNestedFilters {
    #[vndb_filter(nested)]
    Simple(FilterSimpleFilters),
}

#[derive(VndbFiltersEnum)]
pub enum FilterBoxedNestedFilters {
    #[vndb_filter(nested)]
    Simple(Box<FilterSimpleFilters>),
}

#[derive(VndbFiltersEnum)]
pub enum FilterFlattenedFilters {
    #[vndb_filter(flatten)]
    Simple(FilterSimpleFilters),
    #[vndb_filter(nested)]
    Nested(FilterSimpleFilters),
}

#[derive(VndbFiltersEnum)]
pub enum FilterRenamedNestedFilters {
    #[vndb_filter(field = "vn", nested)]
    Vns(FilterRenamedLinkFilters),
}

#[derive(VndbFiltersEnum)]
pub enum FilterRenamedLinkFilters {
    #[vndb_filter(flatten)]
    Simple(FilterSimpleFilters),
}

#[derive(VndbFiltersEnum)]
pub enum FilterWithExtrasFilters {
    #[vndb_filter(value = vndb_api_macros_support::filter::StringValue)]
    Search,
    #[vndb_filter(nested)]
    Simple(FilterSimpleFilters),
}

#[test]
fn filters() {
    assert_eq!(FilterSimpleFilters::Id.to_string(), "id");

    assert_eq!(
        FilterNestedFilters::Simple(FilterSimpleFilters::Id).to_string(),
        "simple{id}"
    );

    assert_eq!(
        serde_json::to_value(&FilterNestedFilters::Simple(FilterSimpleFilters::Id)).unwrap(),
        json!("simple{id}")
    );

    assert_eq!(
        FilterBoxedNestedFilters::Simple(Box::new(FilterSimpleFilters::Id)).to_string(),
        "simple{id}"
    );

    let boxed_filter = FilterBoxedNestedFilters!(simple.id).eq("x");
    assert_eq!(
        serde_json::to_value(boxed_filter).unwrap(),
        json!(["simple", "=", ["id", "=", "x"]])
    );

    assert_eq!(FilterWithExtrasFilters::Search.to_string(), "search");

    assert_eq!(
        FilterWithExtrasFilters::Simple(FilterSimpleFilters::Id).to_string(),
        "simple{id}"
    );
}

#[test]
fn flatten_filters() {
    assert_eq!(
        FilterFlattenedFilters::Simple(FilterSimpleFilters::Id).to_string(),
        "id"
    );

    assert_eq!(
        FilterFlattenedFilters::Nested(FilterSimpleFilters::Id).to_string(),
        "nested{id}"
    );

    let filter: FilterFlattenedFilters = FilterSimpleFilters::Id.into();

    assert_eq!(filter.to_string(), "id");
}

#[test]
fn renamed_nested_filter_keeps_child_filter_type() {
    assert_eq!(
        FilterRenamedNestedFilters::Vns(FilterRenamedLinkFilters::Simple(FilterSimpleFilters::Id))
            .to_string(),
        "vn{id}"
    );
}

#[derive(VndbSortEnum)]
#[vndb_sort(field = "search_rank")]
#[allow(dead_code)]
pub struct Sortable {
    #[vndb_sort]
    id: String,
    #[vndb_sort(rename = "vote_count")]
    votecount: i32,
    #[vndb_sort(nested)]
    nested: FilterSimple,
    #[vndb_sort(flatten)]
    flattened: FilterSimple,
    ignored: String,
}

#[test]
fn sort() {
    assert_eq!(SortableSort::Id.to_string(), "id");
    assert_eq!(SortableSort::Votecount.to_string(), "vote_count");
    assert_eq!(SortableSort::Nested.to_string(), "nested");
    assert_eq!(SortableSort::SearchRank.to_string(), "search_rank");

    assert_eq!(
        serde_json::to_value(&SortableSort::SearchRank).unwrap(),
        json!("search_rank")
    );
}

#[test]
fn sort_selector_macro() {
    let fields = SortableSort!(id, vote_count, search_rank);
    let fields = fields.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(fields, vec!["id", "vote_count", "search_rank"]);
}

#[test]
fn serialization() {
    assert_eq!(
        serde_json::to_value(&NestedFields::FieldA(SimpleFields::Field)).unwrap(),
        json!("field_a{field}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldA(SimpleFields::Id)).unwrap(),
        json!("field_a{id}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldB(Box::new(NestedFields::FieldA(
            SimpleFields::Field
        ))))
        .unwrap(),
        json!("field_b{field_a{field}}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldB(Box::new(NestedFields::FieldA(
            SimpleFields::Id
        ))))
        .unwrap(),
        json!("field_b{field_a{id}}")
    );
}
