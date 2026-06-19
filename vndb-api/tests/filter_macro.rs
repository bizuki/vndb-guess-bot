use serde_json::json;
use vndb_api::filter::{
    Birthday, ExtlinkFilterValue, ExtlinkRemoteId, ExtlinkSite, Resolution, SpoilerLevel,
    TagFilterValue, TagId, TagLevel, VndbFilter,
};
use vndb_api::ids::{InvalidId, VnId};
use vndb_api_derive::VndbFiltersEnum;

#[derive(Debug, Clone, VndbFiltersEnum)]
#[allow(dead_code)]
enum TestCharacterFilters {
    #[vndb_filter(value = vndb_api::filter::StringValue)]
    Name,
    #[vndb_filter(value = vndb_api::filter::IntegerValue, ordered)]
    Age,
    #[vndb_filter(value = vndb_api::filter::Birthday, nullable)]
    Birthday,
    #[vndb_filter(value = vndb_api::filter::IntegerBooleanValue)]
    HasDescription,
    #[vndb_filter(nested)]
    Vn(TestVnFilters),
}

#[derive(Debug, Clone, VndbFiltersEnum)]
#[allow(dead_code)]
enum TestVnFilters {
    #[vndb_filter(value = vndb_api::filter::StringValue, ordered)]
    Released,
}

#[derive(Debug, Clone, VndbFiltersEnum)]
#[allow(dead_code)]
enum TestProducerFilters {
    #[vndb_filter(field = "type", value = vndb_api::filter::StringValue)]
    Type,
}

#[test]
fn typed_filter_fields_build_equality_and_ordered_predicates() {
    let filter = VndbFilter::and(vec![
        TestCharacterFilters!(name).eq("name"),
        TestCharacterFilters!(age).gt(18),
    ]);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["and", ["name", "=", "name"], ["age", ">", 18]])
    );
}

#[test]
fn typed_filter_fields_build_nested_leaf_predicates() {
    let filter = TestCharacterFilters!(vn.released).gte("2020");

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["vn", "=", ["released", ">=", "2020"]])
    );
}

#[test]
fn typed_filter_fields_build_nested_field_predicates() {
    let child = TestVnFilters!(released).gte("2020");
    let filter = TestCharacterFilters!(vn).matches(child);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["vn", "=", ["released", ">=", "2020"]])
    );
}

#[test]
fn typed_filter_fields_support_nullable_and_struct_values() {
    let filter = VndbFilter::or(vec![
        TestCharacterFilters!(birthday).eq(Birthday::new(1, 23).unwrap()),
        TestCharacterFilters!(birthday).is_null(),
    ]);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["or", ["birthday", "=", [1, 23]], ["birthday", "=", null]])
    );
}

#[test]
fn typed_filter_fields_support_raw_identifiers() {
    let filter = TestProducerFilters!(r#type).eq("co");

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["type", "=", "co"])
    );
}

#[test]
fn typed_filter_fields_serialize_integer_booleans() {
    let filter = VndbFilter::or(vec![
        TestCharacterFilters!(has_description).is_true(),
        TestCharacterFilters!(has_description).is_false(),
    ]);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!([
            "or",
            ["has_description", "=", 1],
            ["has_description", "!=", 1]
        ])
    );
}

#[test]
fn ids_validate_deref_and_filter_as_strings() {
    use vndb_api::filter::VnFilters;

    let id = VnId::try_from("v17").unwrap();

    assert_eq!(&*id, "v17");
    assert_eq!(id.as_ref(), "v17");
    assert_eq!(id.to_string(), "v17");
    assert_eq!(serde_json::to_string(&id).unwrap(), "\"v17\"");
    assert_eq!(VnId::try_from("x17").unwrap_err(), InvalidId::WrongPrefix);
    assert_eq!(VnId::try_from("v").unwrap_err(), InvalidId::MissingDigits);

    let filter = VnFilters!(id).eq(id);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["id", "=", "v17"])
    );
}

#[test]
fn real_character_sex_and_gender_filters_are_strings() {
    use vndb_api::models::character::CharacterFilters;

    let filter = VndbFilter::and(vec![
        CharacterFilters!(sex).eq("m"),
        CharacterFilters!(gender).ne("a"),
    ]);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!(["and", ["sex", "=", "m"], ["gender", "!=", "a"]])
    );
}

#[test]
fn real_vn_recursive_nested_filters_are_boxed() {
    use vndb_api::models::{character::CharacterFilters, release::ReleaseFilters, vn::VnFilters};

    let filter = VndbFilter::and(vec![
        VnFilters!(release.released).gte("2020-01-01"),
        VnFilters!(character.age).gt(18),
    ]);

    assert_eq!(
        serde_json::to_value(filter).unwrap(),
        json!([
            "and",
            ["release", "=", ["released", ">=", "2020-01-01"]],
            ["character", "=", ["age", ">", 18]]
        ])
    );
}

#[test]
fn typed_special_filter_values_serialize_to_vndb_shapes() {
    let tag = TagFilterValue::rated(
        TagId::try_from("g505").unwrap(),
        SpoilerLevel::Two,
        TagLevel::new(1.2).unwrap(),
    );

    use vndb_api::models::release::ReleaseFilters;
    use vndb_api::models::vn::VnFilters;

    let tag_filter = VnFilters!(tag).eq(tag);

    assert_eq!(
        serde_json::to_value(tag_filter).unwrap(),
        json!(["tag", "=", ["g505", 2, 1.2]])
    );

    let extlink = ExtlinkFilterValue::site_id(
        ExtlinkSite::new("steam").unwrap(),
        ExtlinkRemoteId::Integer(702050),
    );

    let extlink_filter = ReleaseFilters!(extlink).eq(extlink);

    assert_eq!(
        serde_json::to_value(extlink_filter).unwrap(),
        json!(["extlink", "=", ["steam", 702050]])
    );

    let resolution_filter = ReleaseFilters!(resolution).gte(Resolution::new(1920, 1080).unwrap());

    assert_eq!(
        serde_json::to_value(resolution_filter).unwrap(),
        json!(["resolution", ">=", [1920, 1080]])
    );
}

#[test]
fn filter_macro_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/filter_macro_fail.rs");
}
