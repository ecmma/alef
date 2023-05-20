use crate::source::loc::Location;
use std::fmt::Display;

pub enum Severity {
    Fatal,
    Error,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct LabeledSpan {
    pub msg: Option<String>,
    pub start: usize,
    pub end: usize,
}

// The handling of diagnostic messages in AF takes inspiration from zkat's miette library and
// uses as a default handler zerester's ariadne.
// I do not want to limit users to a single implementation of a diagnostic backend, so this trait
// is another trait layer over miette::Diagnostic with small modifications to fit better in AF.

// Ideally we should have some procedural macro to automatically generate an impl for all these
// functions when necessary.

/// The trait every diagnostic must implement.
pub trait Diagnostic: std::error::Error {
    /// The severity of the diagnostic. The default severity level is Error.
    fn severity(&self) -> Option<Severity> {
        None
    }

    /// A unique diagnostic code such as `foo::bar::baz` or `EO123`.
    fn code<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// The position in the source generating the error in the form `<filename>:<line>:<col>`.
    fn loc<'a>(&self) -> Option<Box<dyn Location + 'a>> {
        None
    }

    /// A string extracted from the source which represents where the error originated from.
    fn context(&self) -> Option<String> {
        None
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        None
    }

    /// Additional help text explaining the cause of the fault.
    fn reason<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// Additional help text related to the diagnostic.
    fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// Relative diagnostic messages.
    fn relatives(&self) -> Option<Vec<Box<dyn Diagnostic>>> {
        None
    }
}
