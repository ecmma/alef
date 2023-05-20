use crate::source::loc::Range;

/// A comment seen in a source file.
pub struct Comment {
    /// The range of the comment.
    pub range: Range,

    /// The content of the comment.
    pub cont: String,
}

#[cfg(test)]
mod test {}
