use num_bigint::BigInt;
use bytes::Bytes;
use rustpython_parser::ast::{StringGroup, Keyword};




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