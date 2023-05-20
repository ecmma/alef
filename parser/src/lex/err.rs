use crate::diagnostic::err::{Diagnostic, LabeledSpan};
use crate::source::loc::{Location, Range};
use std::fmt::Display;
use thiserror::Error;

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("stray symbol {sym:?} in source")]
pub struct StrayCharError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// The stray symbol.
    pub sym: String,

    /// A message regarding the error.
    pub msg: String,
}

impl Diagnostic for StrayCharError {
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
        Some(Box::new(format!("the symbol {:?} is not valid.", self.sym)))
    }

    fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!("remove the symbol {:?}.", self.sym)))
    }

    fn labels<'a>(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        let mut v = std::vec::Vec::new();
        let start = self.range.start.get_col();

        let end = if let Some(ref e) = self.range.end {
            e.get_col()
        } else {
            start
        };
        v.push(LabeledSpan {
            msg: Some(self.msg.clone()),
            start,
            end,
        });

        Some(Box::new(v.into_iter()))
    }
}

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("malformed literal")]
pub struct LiteralError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// A message regarding the error.
    pub msg: String,

    pub sym: Option<char>,
}

impl Diagnostic for LiteralError {
    fn code<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(std::any::type_name::<LiteralError>()))
    }

    fn loc<'a>(&self) -> Option<Box<dyn Location + 'a>> {
        Some(self.range.start.box_clone())
    }

    fn context(&self) -> Option<String> {
        Some(self.range.content.clone())
    }

    fn reason<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(self.msg.clone()))
    }

    fn labels<'a>(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        let mut v = std::vec::Vec::new();
        let start = self.range.start.get_col();

        let end = if let Some(ref e) = self.range.end {
            e.get_col()
        } else {
            start
        };
        if let Some(sym) = self.sym {
            v.push(LabeledSpan {
                msg: Some(format!("symbol '{}'", sym)),
                start,
                end,
            });
        }

        Some(Box::new(v.into_iter()))
    }

    fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }
}

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("malformed or unremoved preprocessor directive")]
pub struct PreprocessorDirectiveError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// A message regarding the error.
    pub msg: String,
}

impl Diagnostic for PreprocessorDirectiveError {
    fn code<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(std::any::type_name::<PreprocessorDirectiveError>()))
    }

    fn loc<'a>(&self) -> Option<Box<dyn Location + 'a>> {
        Some(self.range.start.box_clone())
    }

    fn context(&self) -> Option<String> {
        Some(self.range.content.clone())
    }

    fn reason<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(self.msg.clone()))
    }

    fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }
}

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("malformed comment")]
pub struct CommentError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// A message regarding the error.
    pub msg: String,
}

impl Diagnostic for CommentError {}

#[cfg(test)]
mod tests {}
