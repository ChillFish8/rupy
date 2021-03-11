use num_bigint::BigInt;

/// A set of raw types that can be
pub enum RawTypes {
    None,
    True,
    False,

    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntUnbounded(BigInt),

    Float32(f32),
    Float64(f64),

    String(String),

    Bytes(Bytes)
}
