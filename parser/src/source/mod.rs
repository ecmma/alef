//! We use a single internal in-memory representation of files: the underlying source - a string, a file,
//! a stream, any resource that can be read into  a string - is read in its entirety and then consumed
//! by the lexical and syntactic analysis phase. Using an in-memory representation is useful
//! because it prevents diagnostic and general faults when a user updates the source during the
//! compilation and because it may be faster than other alternatives (e.g. reading every time from
//! a file.)

pub mod err;
pub mod loc;

use self::loc::{DefaultLocation, Location, Range};
use err::SourceReadError;
use log::trace;
use std::io;

/// The in-memory representation of a source. The underlying content representation is a simple
/// primitive `str`.
#[derive(Debug)]
pub struct MemoryBuffer {
    /// The content of the MemoryBuffer.
    content: String,

    /// The length of the content.
    len: usize,

    /// The name of the source.
    name: String,

    /// The index of the byte read up to.
    nindex: usize,

    /// The index of the last '\n' seen.
    nlindex: usize,

    /// The current line.
    line: usize,

    /// The current column.
    col: usize,

    /// (line,col) -> index map.
    map: std::collections::HashMap<(usize, usize), usize>,
}

/// The EOF char is returned when reading from a MemoryBuffer after all the chars in the source
/// were read.
pub const EOF_CHAR: char = '\u{0}';

impl std::fmt::Display for MemoryBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:[{}]", self.name.clone(), self.len)
    }
}

impl MemoryBuffer {
    /// Create a new MemoryBuffer from a file.
    pub fn from_file(filename: String) -> anyhow::Result<MemoryBuffer> {
        trace!("create mbuf from file");
        let mut file = std::fs::File::open(filename.clone())?;
        let content = io::read_to_string(&mut file)?;
        log::trace!("created mbuf with content {:?}", content);
        let len = content.len();
        Ok(MemoryBuffer {
            content,
            len,
            name: filename,
            nindex: 0,
            nlindex: 0,
            line: 1,
            col: 1,
            map: std::collections::HashMap::new(),
        })
    }

    /// Create a new MemoryBuffer from a string.
    pub fn from_str(content: &str, name: String) -> MemoryBuffer {
        trace!("create mbuf from string");
        let l = content.len();
        MemoryBuffer {
            len: l,
            name,
            content: content.to_string(),
            nindex: 0,
            nlindex: 0,
            line: 1,
            col: 1,
            map: std::collections::HashMap::new(),
        }
    }

    /// Fetch and consume the next unread char in the source or return EOF_CHAR if all are read.
    pub fn next_ch(&mut self) -> Result<char, SourceReadError> {
        trace!("MemoryBuffer::next_ch");
        if (self.nindex + 1) > self.content.bytes().len() {
            return Ok(EOF_CHAR);
        }

        let next = self.content.get(self.nindex..);

        if let Some(s) = next {
            let c = s.chars().next();
            if let Some(c) = c {
                self.nindex += c.len_utf8();
                if c == '\n' {
                    self.col = 1;
                    self.line += 1;
                } else {
                    self.col += 1;
                }
                self.map.insert((self.line, self.col), self.nindex);
                if c == '\n' {
                    self.nlindex = self.nindex;
                }
                return Ok(c);
            }
        }

        let lstart = self.get_location();
        let range = self.get_range(lstart.as_ref(), None);
        Err(SourceReadError {
            source_name: self.name.clone(),
            range,
            index: self.nindex,
            msg: "cannot read next character from source!".to_string(),
        })
    }

    /// Fetch but don't consume the l-th unread char in the source or return EOF_CHAR if there is
    /// no l-th char.
    pub fn peek(&mut self, l: usize) -> Result<char, SourceReadError> {
        trace!("MemoryBuffer::peek({})", l);
        if self.nindex == self.content.bytes().len() {
            return Ok(EOF_CHAR);
        }

        if (self.nindex + l) >= self.content.bytes().len() {
            return Ok(EOF_CHAR);
        }

        let next = self.content.get(self.nindex..);
        if let Some(s) = next {
            return Ok(s.chars().nth(l).unwrap());
        }

        let lstart = self.get_location();
        let range = self.get_range(lstart.as_ref(), None);
        Err(SourceReadError {
            source_name: self.name.clone(),
            range,
            index: self.nindex,
            msg: format!("cannot peek {}-th character from source!", l),
        })
    }

    /// Create a location for the current column-line-index position in the source.
    pub fn get_location(&mut self) -> Box<dyn Location> {
        trace!("MemoryBuffer::get_location");
        Box::new(DefaultLocation {
            line: self.line,
            col: self.col,
            source_name: self.name.clone(),
            index: self.nindex,
        })
    }

    pub fn get_current_line(&self) -> String {
        let start = self.nlindex;
        let content = self.content.get(start..);
        if let Some(content) = content {
            if let Some(content) = content.split_once("\n") {
                content.0.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    pub fn get_range(&self, start: &dyn Location, end: Option<&dyn Location>) -> Range {
        trace!(
            "MemoryBuffer::get_range({}, None: {})",
            start,
            end.is_none()
        );
        let start_index = if let Some(index) = self.map.get(&(start.get_line(), start.get_col())) {
            *index
        } else {
            0
        };

        let end: Box<dyn Location> = if let Some(end) = end {
            end.box_clone()
        } else {
            Box::new(DefaultLocation {
                line: self.line,
                col: self.col,
                source_name: self.name.clone(),
                index: self.nindex,
            })
        };

        let end_index = if let Some(index) = self.map.get(&(end.get_line(), end.get_col())) {
            *index
        } else {
            0
        };

        let mut content = self.content[start_index..end_index].to_string();

        if let Some(s) = self.content.get(end_index..) {
            if let Some(c) = s.chars().next() {
                content.push(c);
            }
        }

        let range = Range {
            start: start.box_clone(),
            end: Some(end),
            content,
        };

        trace!("result of get_range: {}", range);
        range
    }

    /// Dump the entire content of the source.
    pub fn get_content(&mut self) -> String {
        self.content.to_string()
    }

    /// Get the name of the source.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mbuf() -> Result<(), SourceReadError> {
        let src = "abcdefghijklmno\npqrstuvzxyz";
        let mut mb = MemoryBuffer::from_str(src, "this is the name!".to_owned());
        assert_eq!(mb.get_name(), "this is the name!");
        assert_eq!(mb.get_content(), src);

        let start = mb.get_location();
        println!("{}", start);

        let mut chars = 1;
        for want in src.chars() {
            let range = mb.get_range(start.as_ref(), None);
            let got = mb.next_ch()?;
            assert_eq!(want, got);

            let content: String = src.chars().take(chars).collect();
            assert_eq!(range.content, content);

            chars += 1;
        }

        let range = mb.get_range(start.as_ref(), None);
        assert_eq!(range.content, src.to_string());

        let end = mb.get_location();
        let range = mb.get_range(start.as_ref(), Some(end.as_ref()));
        assert_eq!(range.content, src.to_string());

        Ok(())
    }

    #[test]
    fn mbuf_next() -> Result<(), SourceReadError> {
        let src = "abcdefghijklmno\npqrstuvxywz";
        let mut mb = MemoryBuffer::from_str(src, "this is the name!".to_owned());

        for want in src.chars() {
            let got = mb.next_ch()?;
            assert_eq!(want, got);
        }

        let want = EOF_CHAR;
        for _i in 1..5 {
            let got = mb.next_ch()?;
            assert_eq!(want, got);
        }

        Ok(())
    }

    #[test]
    fn mbuf_peek() -> Result<(), SourceReadError> {
        let src = "abcdefghijklmno\npqrstuvxywz";
        let mut mb = MemoryBuffer::from_str(src, "this is the name!".to_owned());

        for i in 0..src.len() {
            let got = mb.peek(i)?;
            let want = src.chars().nth(i).unwrap();
            assert_eq!(want, got);
        }

        for i in 0..src.len() {
            let got = mb.peek(i)?;
            let want = src.chars().nth(i).unwrap();
            assert_eq!(want, got);
        }

        for want in src.chars() {
            let got = mb.next_ch()?;
            assert_eq!(want, got);
        }

        let want = EOF_CHAR;
        let got = mb.peek(src.len())?;
        assert_eq!(want, got);

        Ok(())
    }
}
