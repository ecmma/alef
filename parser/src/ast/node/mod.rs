pub mod dec;
pub mod expr;
pub mod stmt;

use std::fmt::{self, Display};
use std::vec::Vec;

/// An AST for an Alef source.
#[derive(Debug, Clone)]
pub struct Program {
    /// The list of declarations.
    pub decs: Vec<Node>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for dec in &self.decs {
            writeln!(f, "{}", dec)?;
        }

        Ok(())
    }
}

/// A node is a single semantic element created from the parsing of an Alef source.
#[derive(Clone, Hash)]
pub enum Node {
    Dec(Box<dec::Dec>),

    Expr(Box<expr::Expr>),

    Stmt(Box<stmt::Stmt>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Dec(d) => write!(f, "{}", d),
            Node::Expr(e) => write!(f, "{:?}", e),
            Node::Stmt(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Debug, Clone, Hash)]
/// A simple helper for nodes which hold lists of other nodes.
pub struct NodeList {
    nodes: Vec<Node>,
}

impl NodeList {
    /// Add a new node.
    pub fn push(&mut self, t: Node) {
        self.nodes.push(t);
    }

    /// Get all the nodes.
    pub fn get_list(&self) -> &Vec<Node> {
        &self.nodes
    }

    /// Get a mutable reference to the nodes.
    pub fn get_mut_list(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }
}
