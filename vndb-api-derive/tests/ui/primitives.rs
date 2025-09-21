use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    int8: Option<i8>,
    int16: Option<i16>,
    int32: Option<i32>,
    int64: Option<i64>,
    int128: Option<i128>,

    uint8: Option<u8>,
    uint16: Option<u16>,
    uint32: Option<u32>,
    uint64: Option<u64>,
    uint128: Option<u128>,

    float32: Option<f32>,
    float64: Option<f64>,

    boolean: Option<bool>,
    character: Option<char>,

    string: Option<String>,
}

fn main() {
    let _ = StructFields::Int8;
    let _ = StructFields::Int16;
    let _ = StructFields::Int32;
    let _ = StructFields::Int64;
    let _ = StructFields::Int128;

    let _ = StructFields::Uint8;
    let _ = StructFields::Uint16;
    let _ = StructFields::Uint32;
    let _ = StructFields::Uint64;
    let _ = StructFields::Uint128;

    let _ = StructFields::Float32;
    let _ = StructFields::Float64;

    let _ = StructFields::Boolean;
    let _ = StructFields::Character;
    let _ = StructFields::String;
}
