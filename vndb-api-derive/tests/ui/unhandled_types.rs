use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    #[vndb_field(nested)]
    str: &'static str,
    #[vndb_field(nested)]
    i32: &'static i32,
}

fn main() {
}
