use crate::{ast::node::*, source::loc::Range, types::*};
use owo_colors::OwoColorize;
use std::hash::Hash;

/// Declarations introduce new named objects and types.
#[derive(Debug, Clone)]
pub enum Dec {
    /// Declaration of a new variable.
    Var {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Var declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a new function with a body.
    Function {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Function declaration.
        atype: Type,

        /// The body of the function.
        body: Box<Node>,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a new function without a body.
    Prototype {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Prototype declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a method relative to some ADT.
    Method {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Method declaration.
        atype: Type,

        /// The name of the adt.
        adt: String,

        /// The body of the function.
        body: Box<Node>,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a new ADT type.
    Adt {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Adt declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: Option<String>,
    },

    /// Declaration of an aggregate type.
    Aggr {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Aggr declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: Option<String>,
    },

    /// Declaration of an union.
    Union {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Union declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: Option<String>,
    },

    /// Declaration of an enumerator.
    Enum {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Enum declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: Option<String>,
    },

    /// Declaration of a new type alias.
    Typedef {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Typedef declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a type parameter relative to an ADT.
    TypeParam {
        /// Where the declaration occurred.
        range: Range,

        /// The name bound to the declaration.
        name: String,
    },

    /// Forward declaration of a complex type.
    Forward {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Forward declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of a new polymorphic type.
    Polydef {
        /// Where the declaration occurred.
        range: Range,

        /// The name bound to the declaration.
        name: String,
    },

    /// Declaration of an intrinsic type.
    Basetype {
        /// Where the declaration occurred.
        range: Range,

        /// The type bound to the Basetype declaration.
        atype: Type,

        /// The name bound to the declaration.
        name: String,
    },
}

impl Display for Dec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dec::Var { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
            Dec::Function { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
            Dec::Prototype { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
            Dec::Method {
                atype, adt, name, ..
            } => write!(f, "{}.{} {}", adt, name, atype),
            Dec::Adt { atype, name, .. } => {
                if let Some(name) = name {
                    write!(f, "{} {}", name.bold(), atype)
                } else {
                    write!(f, "{} {}", "unnamed".bold(), atype)
                }
            }
            Dec::Aggr { atype, name, .. } => {
                if let Some(name) = name {
                    write!(f, "{} {}", name.bold(), atype)
                } else {
                    write!(f, "{} {}", "unnamed".bold(), atype)
                }
            }
            Dec::Union { atype, name, .. } => {
                if let Some(name) = name {
                    write!(f, "{} {}", name.bold(), atype)
                } else {
                    write!(f, "{} {}", "unnamed".bold(), atype)
                }
            }
            Dec::Enum { atype, name, .. } => {
                if let Some(name) = name {
                    write!(f, "{} {}", name.bold(), atype)
                } else {
                    write!(f, "{} {}", "unnamed".bold(), atype)
                }
            }
            Dec::Typedef { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
            Dec::TypeParam { name, .. } => write!(f, "{} {}", "typeparam".bold(), name),
            Dec::Forward { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
            Dec::Polydef { name, .. } => write!(f, "{} {}", "poly".bold(), name),
            Dec::Basetype { atype, name, .. } => write!(f, "{} {}", name.bold(), atype),
        }
    }
}

impl Hash for Dec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
