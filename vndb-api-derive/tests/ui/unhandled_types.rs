use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    str: &'static str,
    i32: &'static i32,
}

fn main() {
    let _ = StructFields::Str;
    let _ = StructFields::I32;
}
