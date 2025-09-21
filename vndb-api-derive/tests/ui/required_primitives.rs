use vndb_api_derive::VndbFieldsEnum;

#[derive(VndbFieldsEnum)]
pub struct Struct {
    // Integers
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    int128: i128,

    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint128: u128,

    // Floating point
    float32: f32,
    float64: f64,

    // Other primitives
    boolean: bool,
    character: char,

    // Strings
    string: String,
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
