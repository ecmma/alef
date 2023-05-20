use crate::diagnostic::err::{Diagnostic, LabeledSpan};
use crate::lex::token::Token;
use crate::source::loc::{Location, Range};
use std::fmt::Display;
use thiserror::Error;

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("cannot parse declaration")]
pub struct ParseDeclError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// The token causing the error..
    pub tok: Token,
}

impl Diagnostic for ParseDeclError {
    fn code<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(std::any::type_name::<Self>()))
    }

    fn loc<'a>(&self) -> Option<Box<dyn Location + 'a>> {
        Some(self.range.start.box_clone())
    }

    fn context(&self) -> Option<String> {
        Some(self.range.content.clone())
    }

    fn reason<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!(
            "{} does not begin a declaration",
            self.tok
        )))
    }

    fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }
    fn labels<'a>(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        let mut v = std::vec::Vec::new();
        let range = self.tok.get_range();
        let start = range.start.get_col();

        let end = if let Some(e) = range.end {
            e.get_col()
        } else {
            start
        };
        v.push(LabeledSpan {
            msg: Some("expected complex type definition, identifier or typedef".into()),
            start,
            end,
        });

        Some(Box::new(v.into_iter()))
    }
}
