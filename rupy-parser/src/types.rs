use num_bigint::BigInt;
use bytes::Bytes;
use rustpython_parser::ast::{StringGroup, Keyword};


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

    Str(String),

    Bytes(Bytes),
}

#[derive(Debug)]
pub enum ParsingTypes {
    None,
    True,
    False,
    Int(BigInt),
    Float(f64),
    Complex((f64, f64)),
    Str(StringGroup),
    Bytes(Vec<u8>),
    Var(String),
    KeyVar(Option<String>),
}