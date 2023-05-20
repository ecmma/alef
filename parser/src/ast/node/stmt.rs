use crate::{
    ast::node::{expr::Expr, NodeList},
    source::loc::Range,
};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty {
        range: Range,
    },

    Expression {
        range: Range,
        expr: Expr,
    },

    Label {
        range: Range,

        name: String,

        to: Box<Stmt>,
    },

    Block {
        range: Range,

        decs: NodeList,

        stms: NodeList,
    },

    If {
        range: Range,

        guard: Box<Expr>,

        then: Box<Stmt>,

        else_: Option<Box<Stmt>>,
    },

    Switch {
        range: Range,

        guard: bool,

        on: Box<Expr>,

        default: Option<Box<Stmt>>,

        cases: NodeList,
    },

    SwitchCase {
        range: Range,

        value: Box<Expr>,

        body: Box<Stmt>,
    },

    DefaultSwitchCase {
        range: Range,

        body: Box<Stmt>,
    },

    Typeof {
        range: Range,

        guard: bool,

        on: Box<Expr>,

        default: Option<Box<Stmt>>,

        cases: NodeList,
    },

    TypeofCase {
        range: Range,

        value: Box<Expr>,

        body: Box<Stmt>,
    },

    DefaultTypeofCase {
        range: Range,

        body: Box<Stmt>,
    },

    Alt {
        range: Range,

        guard: bool,

        cases: NodeList,
    },

    AltCase {
        range: Range,

        value: Box<Expr>,

        body: Box<Stmt>,
    },

    While {
        range: Range,

        condition: Box<Expr>,

        body: Box<Stmt>,
    },

    Do {
        range: Range,

        condition: Box<Expr>,

        body: Box<Stmt>,
    },

    For {
        range: Range,

        init: NodeList,

        cond: NodeList,

        incr: NodeList,

        body: Box<Stmt>,
    },

    Goto {
        range: Range,

        to: Box<Stmt>,
    },

    Continue {
        range: Range,

        depth: Box<Expr>,
    },

    Break {
        range: Range,

        depth: Box<Expr>,
    },

    Return {
        range: Range,

        ret: Box<Expr>,
    },

    Become {
        range: Range,

        be: Box<Expr>,
    },

    Raise {
        range: Range,

        label: Option<String>,

        res_rescue: Option<Box<Expr>>,
    },

    Rescue {
        range: Range,

        label: Option<String>,

        body: Box<Stmt>,
    },

    Check {
        range: Range,

        check: Box<Expr>,

        msg: Option<String>,
    },

    Proc {
        range: Range,

        calls: NodeList,
    },

    Task {
        range: Range,

        calls: NodeList,
    },

    Par {
        range: Range,

        body: Box<Stmt>,
    },

    Alloc {
        range: Range,

        to_alloc: NodeList,
    },

    Unalloc {
        range: Range,

        to_unalloc: NodeList,
    },
}

impl Hash for Stmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
