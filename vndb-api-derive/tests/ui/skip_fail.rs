use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    nested: Nested,
    #[vndb_field(skip)]
    skipped: String
}

#[derive(VndbFieldsEnum)]
pub struct Nested {
    field_a: String,
    field_b: String,
    #[vndb_field(skip)]
    nested: NestedNested
}

#[derive(VndbFieldsEnum)]
pub struct NestedNested {
    test: String
}

fn main() {
    let _ = StructFields::Skipped;
    let _ = StructFields::Nested(vec![NestedFields::FieldA]);
    let _ = StructFields::Nested(vec![NestedFields::FieldB]);
    let _ = StructFields::Nested(vec![NestedFields::Nested(vec![NestedNestedFields::Test])]);
    let _ = StructFields::Nested(vec![NestedFields::FieldA, NestedFields::FieldB]);
    let _ = StructFields::Nested(vec![NestedFields::FieldA, NestedFields::Nested(vec![NestedNestedFields::Test])]);
    let _ = StructFields::Nested(vec![NestedFields::Nested(vec![NestedNestedFields::Test]), NestedFields::FieldB]);
    let _ = StructFields::Nested(vec![NestedFields::Nested(vec![NestedNestedFields::Test]), NestedFields::FieldB]);
}
