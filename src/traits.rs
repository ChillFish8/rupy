use rupy_ast::{Constant, ExprKind};

use num_traits::ToPrimitive;
use anyhow::{Result, Error};


pub trait WrappingType {
    fn to_lua(&self) -> Result<String>;
}


impl WrappingType for String {
    fn to_lua(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl WrappingType for &str {
    fn to_lua(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl WrappingType for i64 {
    fn to_lua(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl WrappingType for f64 {
    fn to_lua(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl WrappingType for bool {
    fn to_lua(&self) -> Result<String> {
        Ok(format!("{}", self))
    }
}

impl WrappingType for () {
    fn to_lua(&self) -> Result<String> {
        Ok("".to_string())
    }
}

impl<T: WrappingType> WrappingType for Option<T> {
    fn to_lua(&self) -> Result<String> {
        match self {
            Self::None => Ok("nil".to_string()),
            Self::Some(v) => v.to_lua(),
        }
    }
}

impl WrappingType for Constant {
    fn to_lua(&self) -> Result<String> {
        match self {
            Self::Str(v) => v.to_lua(),
            Constant::None => None::<()>.to_lua(),
            Constant::Bool(v) => v.to_lua(),
            Constant::Bytes(_) => unimplemented!(),
            Constant::Float(v) => v.to_lua(),
            Constant::Ellipsis => None::<()>.to_lua(),
            Constant::Int(v) => {
                if let Some(v) = v.to_i64() {
                    v.to_lua()
                } else {
                    Err(Error::msg(format!(
                        "integer is out of bounds for i64, max: {}", i64::MAX)))
                }
            }
            Constant::Tuple(_) => unimplemented!(),
            Constant::Complex { .. } => unimplemented!(),
        }
    }
}

impl WrappingType for ExprKind {
    fn to_lua(&self) -> Result<String> {
        match self {
            ExprKind::Name { id, .. } => {
                id.to_lua()
            }
            ExprKind::Constant { value, .. } => {
                value.to_lua()
            }
            ExprKind::BoolOp { .. } => unimplemented!(),
            ExprKind::NamedExpr { .. } => unimplemented!(),
            ExprKind::BinOp { .. } => unimplemented!(),
            ExprKind::UnaryOp { .. } => unimplemented!(),
            ExprKind::Lambda { .. } => unimplemented!(),
            ExprKind::IfExp { .. } => unimplemented!(),
            ExprKind::Dict { .. } => unimplemented!(),
            ExprKind::Set { .. } => unimplemented!(),
            ExprKind::ListComp { .. } => unimplemented!(),
            ExprKind::SetComp { .. } => unimplemented!(),
            ExprKind::DictComp { .. } => unimplemented!(),
            ExprKind::GeneratorExp { .. } => unimplemented!(),
            ExprKind::Await { .. } => unimplemented!(),
            ExprKind::Yield { .. } => unimplemented!(),
            ExprKind::YieldFrom { .. } => unimplemented!(),
            ExprKind::Compare { .. } => unimplemented!(),
            ExprKind::Call { .. } => unimplemented!(),
            ExprKind::FormattedValue { .. } => unimplemented!(),
            ExprKind::JoinedStr { .. } => unimplemented!(),
            ExprKind::Attribute { .. } => unimplemented!(),
            ExprKind::Subscript { .. } => unimplemented!(),
            ExprKind::Starred { .. } => unimplemented!(),
            ExprKind::List { .. } => unimplemented!(),
            ExprKind::Tuple { .. } => unimplemented!(),
            ExprKind::Slice { .. } => unimplemented!(),
        }
    }
}