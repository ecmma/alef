#![allow(unused_assignments, dead_code, unused_imports, unused_variables)]
pub mod dec;
pub mod expr;
pub mod stmt;
pub mod ty;

use crate::{
    ast::node::{Node, Program},
    lex::{cman::CommentManager, scan::Scanner},
    source::MemoryBuffer,
};
use dec::DeclParser;

macro_rules! expect_tok {
    ($self:ident,$is:ident, $tok:expr) => {
        let t = $self.scanner.ptok(0);
        if !t.$is($tok) {
            // @TODO ERRSYNC
            todo!()
        } else {
            $self.scanner.tok();
        }
    };
}
pub(crate) use expect_tok;

/// The parser creates a new AST representing a source file.
pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    /// Create a new parser.
    pub fn new(src: Box<MemoryBuffer>, cman: Option<Box<dyn CommentManager>>) -> Parser {
        Parser {
            scanner: Scanner::new(src, cman),
        }
    }

    /// Parse an Alef program and return a node::Program.
    pub fn parse(&mut self) -> Program {
        let mut dec_parser = DeclParser::new(&mut self.scanner);
        let mut decs = std::vec! {};

        while let Some(d) = dec_parser.declaration() {
            decs.push(d);
        }

        Program { decs }
    }
}
