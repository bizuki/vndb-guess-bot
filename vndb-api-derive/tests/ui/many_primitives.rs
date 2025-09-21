use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    // Integers
    int8: Vec<i8>,
    int16: Vec<i16>,
    int32: Vec<i32>,
    int64: Vec<i64>,
    int128: Vec<i128>,

    uint8: Vec<u8>,
    uint16: Vec<u16>,
    uint32: Vec<u32>,
    uint64: Vec<u64>,
    uint128: Vec<u128>,

    // Floating point
    float32: Vec<f32>,
    float64: Vec<f64>,

    // Other primitives
    boolean: Vec<bool>,
    character: Vec<char>,

    // Strings
    string: Vec<String>,
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
