#![allow(unused)]

use num_bigint::BigInt;
use bytes::Bytes;

/// A set of raw types that can be
#[derive(Debug, PartialEq)]
pub enum RawTypes {
    None,
    Bool(bool),

    Int32(i32),
    Int64(i64),
    Int128(i128),
    IntUnbounded(BigInt),

    Float32(f32),
    Float64(f64),

    Str(String),

    Bytes(Bytes),
}
