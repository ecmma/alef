use crate::{
    diagnostic::diag,
    lex::cman::CommentManager,
    lex::{comment::Comment, err::*, token::*},
    source::{loc::*, MemoryBuffer, EOF_CHAR},
};
use std::i64;

pub struct Scanner {
    /// The source.
    pub src: Box<MemoryBuffer>,

    /// The comment manager.
    cman: Option<Box<dyn CommentManager>>,

    /// A collection of unconsumed tokens.
    tok_buf: std::collections::VecDeque<Token>,
}

impl Scanner {
    pub fn get_line(&self) -> String {
        self.src.get_current_line()
    }

    pub fn new(src: Box<MemoryBuffer>, cman: Option<Box<dyn CommentManager>>) -> Scanner {
        Scanner {
            tok_buf: std::collections::VecDeque::new(),
            src,
            cman,
        }
    }

    fn ch(&mut self) -> char {
        log::trace!("Scanner::ch");
        let c = self.src.next_ch();
        log::trace!("ch consumes char {:?}", c);
        if let Ok(c) = c {
            c
        } else {
            diag(Box::new(c.unwrap_err()));
            self.ch()
        }
    }

    fn pch(&mut self, l: usize) -> char {
        log::trace!("Scanner::pch");
        let c = self.src.peek(l);
        log::trace!("pch peeks char {:?}", c);
        if let Ok(c) = c {
            c
        } else {
            diag(Box::new(c.unwrap_err()));
            self.pch(l)
        }
    }

    /// Get the current location.
    fn get_loc(&mut self) -> Box<dyn Location> {
        log::trace!("Scanner::get_loc");
        self.src.get_location()
    }

    /// Consume the next token in the source file.
    pub fn tok(&mut self) -> Token {
        log::trace!("Scanner::tok");

        if self.tok_buf.is_empty() {
            self.make()
        } else {
            let t = self.tok_buf.pop_front();
            assert!(t.is_some());
            t.unwrap()
        }
    }

    /// Return and don't consume the l-th token in the source file or EOF if
    /// no such token exists.
    pub fn ptok(&mut self, l: usize) -> Token {
        log::trace!("Scanner::ptok {}", l);

        let n: usize;

        if l < self.tok_buf.len() {
            let t = self.tok_buf.get(l);
            assert!(t.is_some());
            return t.unwrap().clone();
        }

        if !self.tok_buf.is_empty() {
            let t = self.tok_buf.back();
            assert!(t.is_some());
            let t = t.unwrap();

            if t.is_end() {
                return t.clone();
            }
        }

        if self.tok_buf.is_empty() {
            n = l + 1;
        } else {
            n = l - self.tok_buf.len() + 1;
        }

        for _i in 0..n {
            let t = self.make();

            if t.is_end() {
                self.tok_buf.push_back(t);
                let t = self.tok_buf.back();
                assert!(t.is_some());
                return t.unwrap().clone();
            }

            self.tok_buf.push_back(t);
        }

        let t = self.tok_buf.get(l);
        assert!(t.is_some());
        t.unwrap().clone()
    }

    /// Create the next token.
    fn make(&mut self) -> Token {
        log::trace!("Scanner::make");
        let lstart = self.get_loc();
        let mut p = self.pch(0);
        match p {
            EOF_CHAR => {
                let range = self.src.get_range(lstart.as_ref(), None);
                Token::End(range)
            }
            '#' => {
                self.preproc();
                self.make()
            }
            ' ' | '\t' | '\r' | '\n' => {
                self.ch();
                p = self.pch(0);
                while p == ' ' || p == '\t' || p == '\r' || p == '\n' {
                    self.ch();
                    p = self.pch(0);
                }
                self.make()
            }

            // Operators.
            // ..., .FLOAT, .
            '.' => {
                let p1 = self.pch(1);
                if p1.is_ascii_digit() {
                    self.number(true)
                } else if p1 == '.' {
                    let p2 = self.pch(2);
                    if p2 == '.' {
                        self.ch();
                        self.ch();
                        let range = self.src.get_range(lstart.as_ref(), None);
                        self.ch();
                        Token::Identifier(range, "...".to_string())
                    } else {
                        self.ch();
                        self.ch();
                        self.make_stray_err(lstart, "unterminated \"...\"", "..");
                        self.make()
                    }
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Operator(range, Operator::Dot)
                }
            }
            '-' => {
                if self.pch(1) == '>' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Operator(range, Operator::Indir)
                } else if self.pch(1) == '-' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Operator(range, Operator::Dec)
                } else if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Operator(range, Operator::Subeq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Operator(range, Operator::Sub)
                }
            }
            '!' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Neq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Not)
                }
            }
            '~' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Operator(range, Operator::BNot)
            }
            '+' => {
                if self.pch(1) == '+' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Inc)
                } else if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Addeq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Add)
                }
            }
            '<' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Leq)
                } else if self.pch(1) == '<' {
                    self.ch();
                    let mut range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    if self.pch(0) == '=' {
                        range = self.src.get_range(lstart.as_ref(), None);
                        self.ch();

                        Token::Operator(range, Operator::Shleq)
                    } else {
                        Token::Operator(range, Operator::Shl)
                    }
                } else if self.pch(1) == '-' {
                    self.ch();
                    let mut range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    if self.pch(0) == '=' {
                        range = self.src.get_range(lstart.as_ref(), None);
                        self.ch();

                        Token::Operator(range, Operator::Snd)
                    } else {
                        Token::Operator(range, Operator::Recv)
                    }
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Lss)
                }
            }
            '?' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Operator(range, Operator::Avail)
            }
            '*' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Muleq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Star)
                }
            }
            '/' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Diveq)
                } else if self.pch(1) == '*' || self.pch(1) == '/' {
                    self.comment();
                    self.make()
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Div)
                }
            }
            '%' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Modeq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Mod)
                }
            }
            ':' => {
                if self.pch(1) == ':' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Iter)
                } else if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Pasgn)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();
                    Token::Delimiter(range, Delimiter::Colon)
                }
            }
            '>' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Geq)
                } else if self.pch(1) == '>' {
                    self.ch();
                    let mut range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    if self.pch(0) == '=' {
                        range = self.src.get_range(lstart.as_ref(), None);
                        self.ch();

                        Token::Operator(range, Operator::Shreq)
                    } else {
                        Token::Operator(range, Operator::Shr)
                    }
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Grt)
                }
            }
            '=' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Eqeq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Asgn)
                }
            }
            '&' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Andeq)
                } else if self.pch(1) == '&' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::LAnd)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Amp)
                }
            }
            '|' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Oreq)
                } else if self.pch(1) == '|' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::LOr)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Or)
                }
            }
            '^' => {
                if self.pch(1) == '=' {
                    self.ch();
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Xoreq)
                } else {
                    let range = self.src.get_range(lstart.as_ref(), None);
                    self.ch();

                    Token::Operator(range, Operator::Xor)
                }
            }

            // Delimiters.
            ';' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Semi)
            }
            ',' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Comma)
            }

            '{' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Lbrace)
            }

            '}' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Rbrace)
            }

            '(' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Lparen)
            }

            ')' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Rparen)
            }

            '[' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Lbrack)
            }

            ']' => {
                let range = self.src.get_range(lstart.as_ref(), None);
                self.ch();

                Token::Delimiter(range, Delimiter::Rbrack)
            }

            // Literals.
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(false),
            '\'' => self.character(),
            '\"' => self.string(),
            '$' => self.runestring(),
            _ => {
                let mut name: String = "".to_string();

                let mut range = self.src.get_range(lstart.as_ref(), None);
                while p.is_alphanumeric() || p == '_' {
                    name.push(p);
                    self.ch();
                    range = self.src.get_range(lstart.as_ref(), None);
                    p = self.pch(0);
                }

                let str = name.as_str();
                if KEYS_MAP.contains_key(str) {
                    Token::Keyword(range, *KEYS_MAP.get(str).unwrap())
                } else {
                    Token::Identifier(range, name)
                }
            }
        }
    }

    /// Read a number literal from the source.
    fn number(&mut self, seen_period: bool) -> Token {
        log::trace!("Scanner::number");
        let mut p = self.pch(0);

        // The literal may be an Integer or a Float.
        // An Integer literal can be expressed in a
        // decimal, octal or hexadecimal base.
        //
        // A Float is composed by an integer part, a dot, a fractional
        // part and an exponent part. If there's the dot, the
        // exponent part may be left out. If there's the exponent part,
        // the dot may be left out.
        let mut base = 10;
        let mut number = String::new();
        let mut ok = true;
        let lstart = self.get_loc();
        if !seen_period {
            if p == '0' {
                match self.pch(1) {
                    'x' | 'X' => {
                        base = 16;
                        self.ch();
                        self.ch();
                    }

                    '.' => {
                        self.ch();
                    }
                    '1'..='9' => {
                        base = 8;
                        self.ch();
                    }
                    _ => {
                        self.ch();
                        number.push('0');
                    }
                }
            }

            p = self.pch(0);

            if base <= 10 {
                let max = '0' as u32 + base;

                while p.is_ascii_digit() {
                    self.ch();
                    if p as u32 >= max && ok {
                        self.make_literal_err(
                            lstart.box_clone(),
                            format!("invalid digit '{}' in base {} literal", p, base).as_str(),
                            None,
                        );
                        ok = false;
                    } else {
                        number.push(p);
                    }
                    p = self.pch(0);
                }
            } else {
                while p.is_ascii_hexdigit() {
                    self.ch();
                    number.push(p);
                    p = self.pch(0);
                }
            }
        }

        if base != 10 && number.is_empty() {
            number.push('0');
            if ok {
                self.make_literal_err(
                    lstart.box_clone(),
                    format!("invalid empty base {} literal", base).as_str(),
                    None,
                );
                ok = false;
            }
        }

        let mut p = self.pch(0);
        if p == '.' {
            if base != 10 && ok {
                self.make_literal_err(
                    lstart.box_clone(),
                    format!("invalid radix point in base {} literal", base).as_str(),
                    None,
                );
                ok = false;
            } else if base == 10 && number.is_empty() {
                number.push('0');
            }

            number.push(p);
            self.ch();
            p = self.pch(0);
        } else {
            let parsed_num = i64::from_str_radix(&number, base);
            if let Ok(num) = parsed_num {
                let range = self.src.get_range(lstart.as_ref(), None);
                return Token::Integer(range, ok, num);
            } else {
                let err = parsed_num.err().unwrap();
                let range = self.src.get_range(lstart.as_ref(), None);

                if ok {
                    self.make_literal_err(
                        lstart.box_clone(),
                        format!("error parsing integer literal: {}", err).as_str(),
                        None,
                    );
                    ok = false;
                }

                return Token::Integer(range, ok, 0);
            }
        }

        while p.is_ascii_digit() {
            number.push(p);
            self.ch();
            p = self.pch(0);
        }

        if p == 'e' || p == 'E' {
            if base != 10 && ok {
                self.make_literal_err(
                    lstart.box_clone(),
                    format!("invalid exponent notation in base {} literal", base).as_str(),
                    None,
                );

                ok = false;
            }

            number.push(p);
            self.ch();
            p = self.pch(0);

            if p == '+' || p == '-' {
                number.push(p);
                self.ch();
                p = self.pch(0);
            }
            let mut count = 0;
            while p.is_ascii_digit() {
                number.push(p);
                self.ch();
                p = self.pch(0);
                count += 1;
            }

            if count == 0 && ok {
                self.make_literal_err(lstart.box_clone(), "float exponent has no digits", None);
                ok = false;
            }
        }

        let parsed_num = number.parse::<f64>();
        if let Ok(num) = parsed_num {
            let range = self.src.get_range(lstart.as_ref(), None);
            Token::Float(range, ok, num)
        } else {
            let err = parsed_num.err().unwrap();

            if ok {
                self.make_literal_err(
                    lstart.box_clone(),
                    format!("error parsing float literal: {}", err).as_str(),
                    None,
                );
                ok = false;
            }
            let range = self.src.get_range(lstart.as_ref(), None);

            Token::Float(range, ok, 0.0)
        }
    }

    /// Read a character literal from the source.
    fn character(&mut self) -> Token {
        log::trace!("Scanner::character");

        // Consume the starting '''.
        let mut p = self.ch();
        assert!(p == '\'');
        let mut c = ' ';
        let mut ok = true;
        let lstart = self.get_loc();

        p = self.pch(0);
        if p == '\\' {
            match self.pch(1) {
                '0' => c = '\0',
                'n' => c = '\n',
                'r' => c = '\r',
                't' => c = '\t',
                '\\' => c = '\\',
                '\"' => c = '\"',
                '\'' => c = '\'',
                _ => {
                    self.ch();

                    self.make_literal_err(
                        lstart.box_clone(),
                        format!("invalid escaped character '{}'", c).as_str(),
                        Some(c),
                    );

                    ok = false;
                }
            }
            self.ch();
        } else {
            c = p;
        }

        self.ch();
        p = self.pch(0);
        if p != '\'' {
            self.make_literal_err(
                lstart.box_clone(),
                "missing '\'' after character literal",
                None,
            );
            ok = false;
        } else {
            self.ch();
        }

        let range = self.src.get_range(lstart.as_ref(), None);
        Token::Character(range, ok, c)
    }

    /// Read a string literal from the source.
    fn string(&mut self) -> Token {
        log::trace!("Scanner::string");
        // Consume the starting '"'.
        let mut p = self.ch();
        assert!(p == '"');

        let mut value = "".to_owned();
        let mut ok = true;

        let lstart = self.get_loc();

        p = self.pch(0);
        while p != '"' && p != '\n' {
            if p == '\\' {
                match self.pch(1) {
                    '0' => value.push('\0'),
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '\"' => value.push('\"'),
                    '\'' => value.push('\''),
                    _ => {
                        self.ch();

                        let c = self.pch(0);
                        self.make_literal_err(
                            lstart.box_clone(),
                            format!("invalid escaped character '{}'", c).as_str(),
                            Some(c),
                        );
                        ok = false;
                    }
                }
                self.ch();
            } else if p.is_ascii() {
                value.push(p);
            } else {
                self.make_literal_err(
                    lstart.box_clone(),
                    format!("invalid non-ASCII character '{}' in string", p).as_str(),
                    Some(p),
                );
                ok = false;
            }

            self.ch();
            p = self.pch(0);
        }

        if p != '"' {
            self.make_literal_err(
                lstart.box_clone(),
                "missing '\"' after string literal",
                None,
            );
            ok = false;
        } else {
            self.ch();
        }

        let range = self.src.get_range(lstart.as_ref(), None);
        Token::String(range, ok, value)
    }

    /// Read a runestring literal from the source.
    fn runestring(&mut self) -> Token {
        log::trace!("Scanner::runestring");

        // Consume the starting '$"'.
        let mut p = self.ch();
        assert!(p == '$');

        p = self.ch();
        assert!(p == '"');

        let lstart = self.get_loc();

        let mut value = "".to_owned();
        let mut ok = true;

        p = self.pch(0);
        while p != '"' && p != '\n' {
            if p == '\\' {
                match self.pch(1) {
                    '0' => value.push('\0'),
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '\"' => value.push('\"'),
                    '\'' => value.push('\''),
                    _ => {
                        self.ch();
                        let c = self.pch(0);
                        self.make_literal_err(
                            lstart.box_clone(),
                            format!("invalid escaped character '{}'", c).as_str(),
                            Some(c),
                        );
                        ok = false;
                    }
                }
                self.ch();
            } else {
                value.push(p);
            }

            self.ch();
            p = self.pch(0);
        }

        if p != '"' {
            self.make_literal_err(
                lstart.box_clone(),
                "missing '\"' after runestring literal",
                None,
            );
            ok = false;
        } else {
            self.ch();
        }

        let range = self.src.get_range(lstart.as_ref(), None);
        Token::Runestring(range, ok, value)
    }

    /// Consume a preprocessor output line, starting with '#'.
    fn preproc(&mut self) {
        log::trace!("Scanner::preproc");
        // Source file name and line number information is conveyed by lines of the
        // form # linenum filename flags - Directly consume '# '.
        let lstart = self.get_loc();

        let mut p = self.ch();
        assert!(p == '#');

        let mut lineno = "".to_owned();
        let mut filename = "".to_owned();
        p = self.pch(0);
        if p.is_ascii_digit() {
            // Read the line number.
            while p.is_ascii_digit() {
                lineno.push(p);
                self.ch();
                p = self.pch(0);
            }

            if p != ' ' {
                //emit::error!(&range, "wrong preprocessor directive, expected a digit");
            } else {
                self.ch();
            }

            p = self.pch(0);

            // Read the filename.
            while p != ' ' {
                filename.push(p);
                self.ch();
                p = self.pch(0);

                // The filename must be on a single line.
                if p == '\n' {
                    //                    let range = self.src.get_range(lstart.as_ref(), None);
                    //emit::error!(&range, "wrong preprocessor directive, expected a filename");
                    //@TODO: preprocessor errors and warnings??
                    break;
                }
            }

            self.ch();
            p = self.pch(0);

            if p == '1' {
                // Start a new file.
                while p != '\n' && p != EOF_CHAR {
                    self.ch();
                    p = self.pch(0);
                }

                // Should the lman be a _source manager_?
                // So, instead of passing MemoryBuffers to the lman we should
                // just pass file names or strings and let the lman keep the
                // map of filenames and actual memory buffers..?
                // Otherwise, who owns this MemBuffer?

                // @TODO: manage inclusion of files...
                // let mbuf = MemoryBuffer::from_file(filename);
                // if let Ok(mbuf) = mbuf {
                //     self.lman.new_file(&mbuf, lineno.parse::<u32>().unwrap());
                // }
            }
        } else {
            p = self.pch(0);

            while p != '\n' && p != EOF_CHAR {
                self.ch();
                p = self.pch(0);
            }

            self.make_preproc_warn(lstart, "unremoved preprocessor directive");
        }
    }

    /// Read and publish a comment from the source.
    fn comment(&mut self) {
        log::trace!("Scanner::comment");
        // Source file name and line number information is conveyed by lines of the
        let lstart = self.get_loc();

        let mut p = self.ch();
        assert!(p == '/');

        let mut comment = p.to_string();

        p = self.pch(0);

        // This is a line comment.
        if p == '/' {
            comment.push(p);
            self.ch();

            p = self.pch(0);
            while p != '\n' {
                comment.push(p);
                p = self.ch();
            }
        } else if p == '*' {
            // Multi line comment.
            comment.push(p);
            self.ch();

            let mut p = self.pch(0);
            loop {
                if p == EOF_CHAR || (p == '*' && self.pch(1) == '/') {
                    break;
                } else {
                    comment.push(p);
                    self.ch();
                }
                p = self.pch(0);
            }

            if !(self.pch(0) == '*' && self.pch(1) == '/') {
                self.make_comment_err(lstart.box_clone(), "unterminated comment");
            } else {
                comment.push_str("*/");
                self.ch();
                self.ch();
            }
        } else {
            self.make_stray_err(lstart, "incomplete comment start", "/");
            return;
        }

        let range = self.src.get_range(lstart.as_ref(), None);
        if let Some(ref mut cman) = self.cman {
            cman.push(Comment {
                range,
                cont: comment,
            });
        }
    }

    fn make_stray_err(&self, lstart: Box<dyn Location>, msg: &str, sym: &str) {
        let range = self.src.get_range(lstart.as_ref(), None);
        let err = Box::new(StrayCharError {
            source_name: self.src.get_name(),
            range: Range {
                start: range.start,
                end: range.end,
                content: self.src.get_current_line(),
            },
            sym: sym.to_string(),
            msg: msg.to_string(),
        });
        log::trace!("making stray err {:?}", err);
        diag(err);
    }

    fn make_literal_err(&self, lstart: Box<dyn Location>, msg: &str, sym: Option<char>) {
        let range = self.src.get_range(lstart.as_ref(), None);
        let err = Box::new(LiteralError {
            source_name: self.src.get_name(),
            range: Range {
                start: range.start,
                end: range.end,
                content: self.src.get_current_line(),
            },
            msg: msg.to_string(),
            sym,
        });
        diag(err);
    }

    fn make_preproc_warn(&self, lstart: Box<dyn Location>, msg: &str) {
        let range = self.src.get_range(lstart.as_ref(), None);
        let err = Box::new(PreprocessorDirectiveError {
            source_name: self.src.get_name(),
            range: Range {
                start: range.start,
                end: range.end,
                content: self.src.get_current_line(),
            },
            msg: msg.to_string(),
        });
        diag(err);
    }

    fn make_comment_err(&self, lstart: Box<dyn Location>, msg: &str) {
        let range = self.src.get_range(lstart.as_ref(), None);
        let err = Box::new(CommentError {
            source_name: self.src.get_name(),
            range: Range {
                start: range.start,
                end: range.end,
                content: self.src.get_current_line(),
            },
            msg: msg.to_string(),
        });
        diag(err);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::source::MemoryBuffer;

    #[test]
    fn smoke_scan() {
        let src = "int a; [}";
        let mb = MemoryBuffer::from_str(src, "SmokeTest".to_owned());
        let mut scanner = Scanner::new(Box::new(mb), None);

        let t = scanner.tok();
        assert_eq!(t.is_identifier(), true);

        let t = scanner.tok();
        assert_eq!(t.is_identifier(), true);

        let t = scanner.tok();
        assert_eq!(t.is_delimiter(Delimiter::Semi), true);

        let t = scanner.tok();
        assert_eq!(t.is_delimiter(Delimiter::Lbrack), true);

        let t = scanner.tok();
        assert_eq!(t.is_delimiter(Delimiter::Rbrace), true);
    }

    #[test]
    fn scan_keys() {
        let mut src = String::new();
        let mut keyvec: std::vec::Vec<Keyword> = std::vec!();

        for (str, key) in KEYS_MAP.entries() {
            src.push_str(str);
            src.push(' ');
            keyvec.push(*key);
        }

        let mb = MemoryBuffer::from_str(src.as_str(), "keys".to_string());
        let mut scanner = Scanner::new(Box::new(mb), None);

        for key in keyvec {
            let tok = scanner.tok();
            assert_eq!(tok.is_keyword(key), true);
        }
    }

    #[test]
    fn scan_ops() {
        let src = ". -> ! ~ ++ -- <- ? * / % + - << >> :: <= < > >= == != & | ^ && || <-= = := += -= *= /= %= &= |= ^= <<= >>=";
        let ops = std::vec!(
            Operator::Dot,
            Operator::Indir,
            Operator::Not,
            Operator::BNot,
            Operator::Inc,
            Operator::Dec,
            Operator::Recv,
            Operator::Avail,
            Operator::Star,
            Operator::Div,
            Operator::Mod,
            Operator::Add,
            Operator::Sub,
            Operator::Shl,
            Operator::Shr,
            Operator::Iter,
            Operator::Leq,
            Operator::Lss,
            Operator::Grt,
            Operator::Geq,
            Operator::Eqeq,
            Operator::Neq,
            Operator::Amp,
            Operator::Or,
            Operator::Xor,
            Operator::LAnd,
            Operator::LOr,
            Operator::Snd,
            Operator::Asgn,
            Operator::Pasgn,
            Operator::Addeq,
            Operator::Subeq,
            Operator::Muleq,
            Operator::Diveq,
            Operator::Modeq,
            Operator::Andeq,
            Operator::Oreq,
            Operator::Xoreq,
            Operator::Shleq,
            Operator::Shreq
        );

        let mb = MemoryBuffer::from_str(src, "Operators".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);
        for op in ops {
            let tok = scanner.tok();
            assert_eq!(tok.is_operator(op), true);
        }
    }

    #[test]
    fn scan_dels() {
        let src = "{ } [ ] ( ) ; , :";
        let dels = std::vec!(
            Delimiter::Lbrace,
            Delimiter::Rbrace,
            Delimiter::Lbrack,
            Delimiter::Rbrack,
            Delimiter::Lparen,
            Delimiter::Rparen,
            Delimiter::Semi,
            Delimiter::Comma,
            Delimiter::Colon
        );
        let mb = MemoryBuffer::from_str(src, "Delimiters".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);
        for want in dels {
            let tok = scanner.tok();
            assert_eq!(tok.is_delimiter(want), true);
        }
    }

    #[test]
    fn scan_ints() {
        let nums: std::vec::Vec<i64> = std::vec!(1, 10, 100, 12);

        let mut str = String::new();

        for num in nums {
            str.push_str(num.to_string().as_str());
            str.push(' ');
        }

        let nums: std::vec::Vec<i64> = std::vec!(1, 10, 100, 12);

        str.push_str("0x10 ");
        str.push_str("010");

        let mb = MemoryBuffer::from_str(str.as_str(), "Integers".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);
        for want in nums {
            let tok = scanner.tok();
            assert_eq!(tok.is_literal(), true);
            if let Token::Integer(.., got) = tok {
                assert_eq!(want, got);
            }
        }

        let tok = scanner.tok();
        let want = 16;
        assert_eq!(tok.is_literal(), true);
        if let Token::Integer(.., got) = tok {
            assert_eq!(want, got);
        }

        let tok = scanner.tok();
        let want = 8;
        assert_eq!(tok.is_literal(), true);
        if let Token::Integer(.., got) = tok {
            assert_eq!(want, got);
        }
    }

    #[test]
    fn scan_floats() {
        let nums: std::vec::Vec<f64> = std::vec!(1.0, 10.0, 3.14);

        let mut str = String::new();

        for num in nums {
            str.push_str(num.to_string().as_str());
            str.push(' ');
        }

        let nums: std::vec::Vec<f64> = std::vec!(1.0, 10.0, 3.14);

        str.push_str(".123123 ");
        str.push_str(".123e2 ");

        let mb = MemoryBuffer::from_str(str.as_str(), "Integers".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);

        for want in nums {
            let tok = scanner.tok();
            assert_eq!(tok.is_literal(), true);
            if let Token::Float(.., got) = tok {
                assert_eq!(want, got);
            }
        }

        let tok = scanner.tok();
        let want = 0.123123;
        assert_eq!(tok.is_literal(), true);
        if let Token::Float(.., got) = tok {
            assert_eq!(want, got);
        }

        let tok = scanner.tok();
        let want = 0.123e2;
        assert_eq!(tok.is_literal(), true);
        if let Token::Float(.., got) = tok {
            assert_eq!(want, got);
        }
    }

    #[test]
    fn scan_chars() {
        let str = "'\\r' '\\t' '\\n' '\\\'' '\\\\'";

        let mb = MemoryBuffer::from_str(str, "Chars".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);

        let chars: std::vec::Vec<char> = std::vec!('\r', '\t', '\n', '\'', '\\');
        for want in chars {
            let tok = scanner.tok();
            assert_eq!(tok.is_literal(), true);
            if let Token::Character(.., got) = tok {
                assert_eq!(want, got);
            }
        }
    }

    #[test]
    fn scan_strings() {
        let strs: std::vec::Vec<&str> = std::vec!("this ", "is ", "a ", " string", " hehe");
        let mut str = String::new();

        for s in strs {
            str.push('\"');
            str.push_str(s);
            str.push('\"');
            str.push(' ');
        }

        let mb = MemoryBuffer::from_str(str.as_str(), "Chars".to_string());

        let mut scanner = Scanner::new(Box::new(mb), None);

        let strs: std::vec::Vec<&str> = std::vec!("this ", "is ", "a ", " string", " hehe");
        for want in strs {
            let tok = scanner.tok();
            assert_eq!(tok.is_literal(), true);
            if let Token::String(.., got) = tok {
                assert_eq!(want, got);
            }
        }
    }

    #[test]
    fn scan_runestrings() {}

    #[test]
    fn scan_locs() {}

    #[test]
    fn scan_entire_source() {}
}
