use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    #[vndb_field(nested, flatten)]
    child: Child,
}

#[derive(VndbFieldsEnum)]
pub struct Child {
    field: String,
}

fn main() {}
