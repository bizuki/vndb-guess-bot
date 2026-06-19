use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    #[vndb_field(nested)]
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
    let _ = StructFields::Nested(NestedFields::FieldA);
    let _ = StructFields::Nested(NestedFields::FieldB);
    let _ = StructFields::Nested(NestedFields::Nested(NestedNestedFields::Test));
}
