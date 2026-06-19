use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    nested_no_field: Nested,
    enum_field: E,
}

pub struct Nested {
    field_a: String,
    field_b: String,
}

pub enum E {
    A,
    B,
    C
}

fn main() {
    let _ = StructFields::NestedNoField;
    let _ = StructFields::EnumField;
}
