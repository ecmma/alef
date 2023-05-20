#![feature(lazy_cell)]

/// Representation and operations on Alef ASTs.
pub mod ast {
    /// The AST representation itself.
    pub mod node;

    /// Visibility enums.
    pub mod scope;

    /// Visitor trait.
    pub mod visit;
}
/// Diagnostic tooling.
pub mod diagnostic;

/// Symbol tables. 
pub mod sym; 

/// Lexical analysis.
pub mod lex {
    /// A comment manager trait.
    pub mod cman;

    /// The representation of comments in the source code, published to a CommentManager or
    /// ignored and discarded.
    pub mod comment;

    /// The scanner: fetches chars from the MemoryBuffer and creates tokens.
    pub mod scan;

    /// Tokens.
    pub mod token;

    /// Diagnostics regarding the lexical analysis phase.
    pub mod err;
}

/// Operations and representation of Alef source files.
pub mod source;

/// Syntactic analysis, companion of the lexical analysis.
pub mod parse;

/// Internal representation of Alef's type system.
pub mod types;
