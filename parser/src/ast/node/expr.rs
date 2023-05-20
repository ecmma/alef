use crate::ast::node::{dec::*, Node, NodeList};
use crate::source::loc::Range;
use crate::types::*;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum LitKind {
    String(String),

    Int(i64),

    Float(f64),

    Char(char),
}

/// Expressions refer to and do operations on variables and constants.
#[derive(Debug, Clone)]
pub enum Expr {
    /// The "..." expression.
    Ellipsis {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,
    },

    // @TODO should we remove this?
    /// The "nil" expression.
    Nil {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,
    },

    /// An expression referencing an identifier.
    Identifier {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        /// The name of the identifier.
        name: String,

        /// Where the identifier was introduced, or None.
        declared: Option<Rc<Dec>>,
    },

    Literal {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        kind: LitKind,
    },

    Paren {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        inner: Box<Expr>,
    },

    Tuple {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        exprs: NodeList,
    },

    ArrayAccess {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        index: Box<Expr>,
    },

    FuncCall {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        args: NodeList,
    },

    AdtNamecall {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        /// The name of the function being referenced.
        func_name: String,

        /// The name of the adt being referenced.
        adt_name: String,

        /// The declaration of the adt being referenced.
        adt: Option<Dec>,
    },

    TypeAccess {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        type_name: String,

        dec_ref: Option<Node>,
    },

    IndirectTypeAccess {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        type_name: String,

        dec_ref: Option<Node>,
    },

    Access {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        member_name: String,

        dec_ref: Option<Node>,
    },

    IndirectAccess {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        member_name: String,

        dec_ref: Option<Node>,
    },

    Postfix {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        arg: Box<Expr>,

        op: Box<Expr>,
    },

    PrefixArith {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        op: Box<Expr>,

        arg: Box<Expr>,
    },

    ChanRecv {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        chan: Box<Expr>,
    },

    ChanSend {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        chan: Box<Expr>,

        expr: Box<Expr>,
    },

    CanChanSend {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        chan: Box<Expr>,
    },

    CanChanRecv {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        chan: Box<Expr>,
    },

    Zerox {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        poly: Box<Expr>,
    },

    Sizeof {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        arg: Box<Expr>,
    },

    SizeofType {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        type_name: Type,
    },

    Cast {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        arg: Box<Expr>,

        new_type: Type,
    },

    Polycast {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        content: Box<Expr>,
    },

    BinaryArith {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    BooleanArith {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    Comparison {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    Shift {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    Iter {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    Assignment {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        op: Box<Expr>,

        right: Box<Expr>,
    },

    ArrayElementInit {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        obj: Rc<Dec>,

        value: Box<Expr>,

        pos: Box<Expr>,
    },

    BlockInit {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        obj: Rc<Dec>,

        exprs: NodeList,
    },

    MemberInit {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        obj: Rc<Dec>,

        value: Box<Expr>,

        name: String,
    },

    ImplicitCast {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        arg: Box<Expr>,

        new_type: Type,
    },

    ImplicitPack {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        tuple: Box<Expr>,

        complex: Type,
    },

    ImplicitUnpack {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        tuple: Box<Expr>,

        complex: Type,
    },

    ImplicitTypeAccess {
        /// Where the expression occurred.
        range: Range,

        /// The type of the expression, None before typechecking.
        atype: Option<Type>,

        left: Box<Expr>,

        type_name: String,

        dec_ref: Option<Node>,
    },
}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
