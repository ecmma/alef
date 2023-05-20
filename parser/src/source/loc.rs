//! Locations are simple elements that contain a reference to a precise column-line-index position
//! in a determinate MemoryBuffer.

use std::fmt::{Debug, Display, Formatter, Result};

/// This trait is the abstract representation of a location in a source file.
pub trait Location: Display {
    /// Get the line of the location in the source.
    fn get_line(&self) -> usize {
        0
    }

    /// Get the column of the location in the source.
    fn get_col(&self) -> usize {
        0
    }

    /// Get the index for the location in the MemoryBuffer.
    fn get_mbuf_index(&self) -> usize {
        0
    }

    /// Return true if the location is the special predeclared location.
    fn is_predeclared(&self) -> bool {
        false
    }

    /// Return true if the location is the special unknown location.
    fn is_unknown(&self) -> bool {
        false
    }

    // Get a boxed clone of the location.
    fn box_clone(&self) -> Box<dyn Location>;
}

impl Debug for dyn Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

/// The default implementation of predeclared locations.
#[derive(Debug)]
pub struct DefaultPredeclaredLocation {}

impl Display for DefaultPredeclaredLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<predeclared>")
    }
}

impl Location for DefaultPredeclaredLocation {
    fn is_predeclared(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn Location> {
        Box::new(DefaultPredeclaredLocation {})
    }
}

pub fn generate_predeclared() -> Box<dyn Location> {
    Box::new(DefaultPredeclaredLocation {})
}

/// The default implementation of predeclared locations.
#[derive(Debug)]
pub struct DefaultUnknownLocation {}

impl Display for DefaultUnknownLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<unknown>")
    }
}

impl Location for DefaultUnknownLocation {
    fn is_unknown(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn Location> {
        Box::new(DefaultUnknownLocation {})
    }
}

pub fn generate_unknown() -> Box<dyn Location> {
    Box::new(DefaultPredeclaredLocation {})
}

pub fn generate_predeclared_range() -> Range {
    Range {
        start: generate_predeclared(),
        end: None,
        content: String::new(),
    }
}

pub fn generate_unknown_range() -> Range {
    Range {
        start: generate_unknown(),
        end: None,
        content: String::new(),
    }
}

/// The default implementation of locations.
#[derive(Debug)]
pub struct DefaultLocation {
    pub line: usize,
    pub col: usize,
    pub index: usize,
    pub source_name: String,
}

impl Location for DefaultLocation {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_col(&self) -> usize {
        self.col
    }

    fn get_mbuf_index(&self) -> usize {
        self.index
    }
    fn is_predeclared(&self) -> bool {
        false
    }

    fn is_unknown(&self) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn Location> {
        Box::new(DefaultLocation {
            line: self.line,
            col: self.col,
            source_name: self.source_name.clone(),
            index: self.index,
        })
    }
}

impl Display for DefaultLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{}:{}", self.source_name, self.line, self.col)
    }
}

/// Ranges are simple tuples of locations referring to a certain MemoryBuffer.
pub struct Range {
    pub start: Box<dyn Location>,
    pub end: Option<Box<dyn Location>>,
    pub content: String,
}

impl Clone for Range {
    fn clone(&self) -> Self {
        let mut r = Range {
            start: self.start.box_clone(),
            end: None,
            content: self.content.clone(),
        };

        if let Some(ref end) = self.end {
            r.end = Some(end.box_clone());
        }
        r
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(ref end) = self.end {
            write!(f, "{} - {}", self.start, end)
        } else {
            write!(f, "{}", self.start)
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(ref end) = self.end {
            write!(f, "{} - {}: {}", self.start, end, self.content)
        } else {
            write!(f, "{}: {}", self.start, self.content)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::source::err::SourceReadError;
    use crate::source::MemoryBuffer;
    #[test]
    fn default_loc() -> Result<(), SourceReadError> {
        let src = "abcdefghijklmno\nasdasdasdasdasdasd";
        let mut mb = MemoryBuffer::from_str(src, "name".to_owned());

        let mut col = 1;
        let mut line = 1;

        let loc = mb.get_location();
        assert_eq!(loc.is_predeclared(), false);
        assert_eq!(loc.is_unknown(), false);

        for c in src.chars() {
            let loc = mb.get_location();

            assert_eq!(loc.get_col(), col);
            assert_eq!(loc.get_line(), line);

            if c == '\n' {
                col = 1;
                line += 1;
            } else {
                col += 1;
            }

            mb.next_ch()?;
        }
        Ok(())
    }
}
