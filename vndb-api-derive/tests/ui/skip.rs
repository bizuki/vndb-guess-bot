use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    #[vndb_field(skip)]
    skipped: Nested,
    str: String
}

pub struct Nested {
    field_a: String,
    field_b: String,
}

fn main() {
    let _ = StructFields::Str;
}
