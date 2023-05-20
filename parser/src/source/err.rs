use crate::diagnostic::err::Diagnostic;
use crate::source::loc::Range;
use thiserror::Error;

/// Error thrown by the source when failing to read from a MemoryBuffer.
#[derive(Error, Debug)]
#[error("can't read from source")]
pub struct SourceReadError {
    /// The name of the source generating this read error.
    pub source_name: String,

    /// The position where this fault generated.
    pub range: Range,

    /// The index where this fault generated.
    pub index: usize,

    /// A message regarding the error.
    pub msg: String,
}

impl Diagnostic for SourceReadError {}

#[cfg(test)]
mod tests {}
