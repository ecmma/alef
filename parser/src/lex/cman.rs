use crate::lex::comment::Comment;
use std::vec::Vec;

/// Comment managers are instances which are used to hold every comment that
/// the scanner sees during lexical analysis.
pub trait CommentManager {
    /// Insert a new comment.
    fn push(&mut self, comment: Comment);

    /// Get all the pushed comments.
    fn get_all(&self) -> Vec<&Comment>;
}

pub struct DefaultCommentManager {
    comments: Vec<Comment>,
}

impl DefaultCommentManager {
    /// Create a new empty DefaultCommentManager.
    pub fn new() -> DefaultCommentManager {
        DefaultCommentManager { comments: vec![] }
    }
}

impl Default for DefaultCommentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CommentManager for DefaultCommentManager {
    fn push(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    fn get_all(&self) -> Vec<&Comment> {
        let mut v: Vec<&Comment> = vec![];
        for c in self.comments.iter() {
            v.push(c);
        }
        v
    }
}

#[cfg(test)]
mod test {}
