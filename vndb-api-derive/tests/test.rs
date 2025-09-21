use serde_json::json;
use vndb_api_derive::VndbFieldsEnum;
use trybuild;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/primitives.rs");
    t.pass("tests/ui/required_primitives.rs");
    t.pass("tests/ui/many_primitives.rs");
    t.pass("tests/ui/complex_types.rs");
    t.pass("tests/ui/skip.rs");
    t.pass("tests/ui/is_primitive.rs");
    t.compile_fail("tests/ui/unhandled_types.rs");
    t.compile_fail("tests/ui/skip_fail.rs");
}

mod to_string {
    use vndb_api_derive::VndbFieldsEnum;

    #[derive(VndbFieldsEnum)]
    #[allow(dead_code)]
    pub struct Simple {
        id: String,
        field: i32
    }

    #[test]
    fn simple() {

        assert_eq!(
            SimpleFields::Field.to_string(),
            "field"
        );

        assert_eq!(
            SimpleFields::Id.to_string(),
            "id"
        );
    }

    #[derive(VndbFieldsEnum)]
    #[allow(dead_code)]
    pub struct Nested {
        field_a: Simple,
        field_b: Box<Nested>,
    }

    #[test]
    fn nested() {
        assert_eq!(
            NestedFields::FieldA(vec![SimpleFields::Field]).to_string(),
            "field_a{field}"
        );

        assert_eq!(
            NestedFields::FieldA(vec![SimpleFields::Id]).to_string(),
            "field_a{id}"
        );

        assert_eq!(
            NestedFields::FieldA(vec![SimpleFields::Id, SimpleFields::Field]).to_string(),
            "field_a{id,field}"
        );

        assert_eq!(
            NestedFields::FieldB(
                vec![NestedFields::FieldA(vec![SimpleFields::Field])]
            ).to_string(),
            "field_b{field_a{field}}"
        );

        assert_eq!(
            NestedFields::FieldB(
                vec![NestedFields::FieldA(vec![SimpleFields::Id])]
            ).to_string(),
            "field_b{field_a{id}}"
        );

        assert_eq!(
            NestedFields::FieldB(
                vec![NestedFields::FieldA(vec![SimpleFields::Field, SimpleFields::Id])]
            ).to_string(),
            "field_b{field_a{field,id}}"
        );
    }

}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct Simple {
    id: String,
    field: i32
}

#[test]
fn simple() {
    assert_eq!(
        SimpleFields::Field.to_string(),
        "field"
    );

    assert_eq!(
        SimpleFields::Id.to_string(),
        "id"
    );
}

#[derive(VndbFieldsEnum)]
#[allow(dead_code)]
pub struct Nested {
    field_a: Simple,
    field_b: Box<Nested>,
}

#[test]
fn nested() {
    assert_eq!(
        NestedFields::FieldA(vec![SimpleFields::Field]).to_string(),
        "field_a{field}"
    );

    assert_eq!(
        NestedFields::FieldA(vec![SimpleFields::Id]).to_string(),
        "field_a{id}"
    );

    assert_eq!(
        NestedFields::FieldA(vec![SimpleFields::Id, SimpleFields::Field]).to_string(),
        "field_a{id,field}"
    );

    assert_eq!(
        NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Field])]
        ).to_string(),
        "field_b{field_a{field}}"
    );

    assert_eq!(
        NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Id])]
        ).to_string(),
        "field_b{field_a{id}}"
    );

    assert_eq!(
        NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Field, SimpleFields::Id])]
        ).to_string(),
        "field_b{field_a{field,id}}"
    );
}

#[test]
fn serialization() {
    assert_eq!(
        serde_json::to_value(&NestedFields::FieldA(vec![SimpleFields::Field])).unwrap(),
        json!("field_a{field}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldA(vec![SimpleFields::Id])).unwrap(),
        json!("field_a{id}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldA(vec![SimpleFields::Field, SimpleFields::Id])).unwrap(),
        json!("field_a{field,id}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Field])]
        )).unwrap(),
        json!("field_b{field_a{field}}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Id])]
        )).unwrap(),
        json!("field_b{field_a{id}}")
    );

    assert_eq!(
        serde_json::to_value(&NestedFields::FieldB(
            vec![NestedFields::FieldA(vec![SimpleFields::Field, SimpleFields::Id])]
        )).unwrap(),
        json!("field_b{field_a{field,id}}")
    );
}
