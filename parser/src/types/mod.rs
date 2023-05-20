pub mod visit;
use crate::ast::node::*;
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct VariantsList {
    variants: Vec<Type>,
}

impl Hash for VariantsList {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl VariantsList {
    pub fn push(&mut self, t: Type) {
        self.variants.push(t);
    }

    pub fn get_variants(&self) -> &Vec<Type> {
        &self.variants
    }
}

#[derive(Debug, Clone)]
pub enum BasicType {
    Void,
    Byte,
    Sint,
    Usint,
    Int,
    Uint,
    Float,
    Lint,
    Ulint,
    Chan { variants: VariantsList },
    Poly { name: String },
}

impl Hash for BasicType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            BasicType::Void => "void".hash(state),
            BasicType::Byte => "byte".hash(state),
            BasicType::Sint => "sint".hash(state),
            BasicType::Usint => "usint".hash(state),
            BasicType::Int => "int".hash(state),
            BasicType::Uint => "uint".hash(state),
            BasicType::Float => "float".hash(state),
            BasicType::Lint => "lint".hash(state),
            BasicType::Ulint => "ulint".hash(state),
            BasicType::Chan { variants } => variants.hash(state),
            BasicType::Poly { name } => name.hash(state),
        };
        state.finish();
    }
}

#[derive(Debug, Clone, Hash)]
pub struct PointerType {
    pub points_to: Box<Type>,
}

#[derive(Debug, Clone, Hash)]
pub struct FunctionType {
    pub ret: Box<Type>,
    pub params: Vec<Type>,
}

#[derive(Debug, Clone, Hash)]
pub struct MethodType {
    pub ret: Box<Type>,
    pub params: Vec<Type>,
}

#[derive(Debug, Clone, Hash)]
pub struct ArrayType {
    pub of: Box<Type>,
    pub size: Box<expr::Expr>,
}

#[derive(Debug, Clone, Hash)]
pub enum DerivedType {
    Pointer(PointerType),
    Function(FunctionType),
    Method(MethodType),
    Array(ArrayType),
}

#[derive(Debug, Clone, Hash)]
pub struct EnumType {
    pub members: NodeList,
}

#[derive(Debug, Clone, Hash)]
pub struct AggrType {
    pub members: NodeList,
}

#[derive(Debug, Clone, Hash)]
pub struct AdtType {
    pub variants: VariantsList,
    pub members: NodeList,
}

#[derive(Debug, Clone, Hash)]
pub struct UnionType {
    pub members: NodeList,
}

#[derive(Debug, Clone, Hash)]
pub struct TupleType {
    pub variants: VariantsList,
}

#[derive(Debug, Clone, Hash)]
pub enum ComplexType {
    Enum(EnumType),
    Aggr(AggrType),
    Adt(AdtType),
    Union(UnionType),
    Tuple(TupleType),
}

#[derive(Debug, Clone, Hash)]
pub enum Type {
    Basic {
        declared: Option<Box<dec::Dec>>,
        kind: BasicType,
    },
    Derived {
        declared: Option<Box<dec::Dec>>,
        kind: DerivedType,
    },
    ComplexType {
        declared: Option<Box<dec::Dec>>,
        kind: ComplexType,
    },
    Unresolved {
        name: String,
    },
    Error,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Basic { declared, kind } => {
                if let Some(dec) = declared {
                    write!(f, "{} {:?}", dec, kind)
                } else {
                    write!(f, "{:?}", kind)
                }
            }
            Type::Derived { declared, kind } => {
                if let Some(dec) = declared {
                    write!(f, "{} {:?}", dec, kind)
                } else {
                    write!(f, "{:?}", kind)
                }
            }
            Type::ComplexType { declared, kind } => {
                if let Some(dec) = declared {
                    write!(f, "{} {:?}", dec, kind)
                } else {
                    write!(f, "{:?}", kind)
                }
            }
            Type::Unresolved { name } => {
                write!(f, "unresolved \"{}\"", name)
            }
            Type::Error => {
                write!(f, "error type")
            }
        }
    }
}
