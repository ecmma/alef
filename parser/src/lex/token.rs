use crate::source::loc::Range;
use phf::phf_map;
use std::fmt;
use std::matches;

pub static KEYS_MAP: phf::Map<&'static str, Keyword> = phf_map! {
"adt" => Keyword::Adt,
    "aggr" => Keyword::Aggr,
    "alloc" => Keyword::Alloc,
    "alt" => Keyword::Alt,
    "become" => Keyword::Become,
    "break" => Keyword::Break,
    "case" => Keyword::Case,
    "check" => Keyword::Check,
    "continue" => Keyword::Continue,
    "default" => Keyword::Default,
    "do" => Keyword::Do,
    "else" => Keyword::Else,
    "enum" => Keyword::Enum,
    "extern" => Keyword::Extern,
    "for" => Keyword::For,
    "goto" => Keyword::Goto,
    "if" => Keyword::If,
    "intern" => Keyword::Intern,
    "nil" => Keyword::Nil,
    "par" => Keyword::Par,
    "proc" => Keyword::Proc,
    "private" => Keyword::Private,
    "raise" => Keyword::Raise,
    "rescue" => Keyword::Rescue,
    "return" => Keyword::Return,
    "sizeof" => Keyword::Sizeof,
    "switch" => Keyword::Switch,
    "task" => Keyword::Task,
    "tuple" => Keyword::Tuple,
    "typedef" => Keyword::Typedef,
    "typeof" => Keyword::Typeof,
    "unalloc" => Keyword::Unalloc,
    "union" => Keyword::Union,
    "while" => Keyword::While,
    "zerox" => Keyword::Zerox,
};

/// Enum used to represent Alef operators.
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Operator {
    /* Highest precedence first. */
    /// Dot operator: ".".
    Dot,
    /// Indirection operator: "->".
    Indir,

    /* Unary operator precedence. */
    /// Not operator: "!".
    Not,
    /// Binary not operator: "~".
    BNot,
    /// Increment operator: "++".
    Inc,
    /// Decrement operator: "--".
    Dec,
    /// Receive operator:  "<-".
    Recv,
    /// Avail-check operator: "?".
    Avail,

    /* Multiplication precedence. */
    /// Star operator: "*".
    Star,
    /// Division operator: "/".
    Div,
    /// Module operator: "%".
    Mod,

    /* Addition precedence.*/
    /// Addition operator: "+".
    Add,
    /// Subtraction operator: "-".
    Sub,

    /* Shift precedence. */
    /// Shift left operator: "<<".
    Shl,
    /// Shift right operator: ">>".
    Shr,

    /* Iterator precedence. */
    /// Iteration operator: "::".
    Iter,

    /* Comparison precedence. */
    /// Less or equal than operator: "<=".
    Leq,
    /// Less than operator: "<".
    Lss,
    /// Greater than operator: ">".
    Grt,
    /// Greater or equal than operator: ">=".
    Geq,

    /* Equation precedence. */
    /// Equality comparison operator: "==".
    Eqeq,
    /// Not-equality comparison operator: "!=".
    Neq,

    /* Bitwise and/yield addr precedence. */
    /// Ampersand operator: "&".
    Amp,

    /* Bitwise or precedence. */
    /// Bitwise or operator: "|".
    Or,

    /* Bitwise xor precedence. */
    /// Xor operator: "^".
    Xor,

    /* Logical and precedence. */
    /// Logical and operator: "&&".
    LAnd,

    /* Logical or precedence.*/
    /// Logical or operator: "||".
    LOr,

    /* Assignment precedence. */
    /// Send on channel operator: "<-=".
    Snd,
    /// Simple assignment operator: "=".
    Asgn,
    /// Polymorph copy-erase assignment operator: ":=".
    Pasgn,
    /// Add-assign operator: "+=".
    Addeq,
    /// Sub-assign operator: "-=".
    Subeq,
    /// Mul-assign operator: "*=".
    Muleq,
    /// Div-assign operator: "/=".
    Diveq,
    /// Mod-assign operator: "%=".
    Modeq,
    /// Binary-and-assign operator: "&=".
    Andeq,
    /// Binary-or-assign operator: "|=".
    Oreq,
    /// Xor-assign operator: "^=".
    Xoreq,
    /// Shift-left-assign operator: "<<=".
    Shleq,
    /// Shift-right-assign operator: ">>=".
    Shreq,
}

impl fmt::Display for Operator {
    /// Format an operator object.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        match *self {
            Operator::Dot => s = String::from("."),
            Operator::Indir => s = String::from("->"),
            Operator::Not => s = String::from("!"),
            Operator::BNot => s = String::from("~"),
            Operator::Inc => s = String::from("++"),
            Operator::Dec => s = String::from("--"),
            Operator::Recv => s = String::from("<-"),
            Operator::Avail => s = String::from("?"),
            Operator::Star => s = String::from("*"),
            Operator::Div => s = String::from("/"),
            Operator::Mod => s = String::from("%"),
            Operator::Add => s = String::from("+"),
            Operator::Sub => s = String::from("-"),
            Operator::Shl => s = String::from("<<"),
            Operator::Shr => s = String::from(">>"),
            Operator::Iter => s = String::from("::"),
            Operator::Leq => s = String::from("<="),
            Operator::Lss => s = String::from("<"),
            Operator::Grt => s = String::from(">"),
            Operator::Geq => s = String::from(">="),
            Operator::Eqeq => s = String::from("=="),
            Operator::Neq => s = String::from("!="),
            Operator::Amp => s = String::from("&"),
            Operator::Or => s = String::from("|"),
            Operator::Xor => s = String::from("^"),
            Operator::LAnd => s = String::from("&&"),
            Operator::LOr => s = String::from("||"),
            Operator::Snd => s = String::from("<-="),
            Operator::Asgn => s = String::from("="),
            Operator::Pasgn => s = String::from(":="),
            Operator::Addeq => s = String::from("+="),
            Operator::Subeq => s = String::from("-="),
            Operator::Muleq => s = String::from("*="),
            Operator::Diveq => s = String::from("/="),
            Operator::Modeq => s = String::from("%="),
            Operator::Andeq => s = String::from("&="),
            Operator::Oreq => s = String::from("|="),
            Operator::Xoreq => s = String::from("^="),
            Operator::Shleq => s = String::from("<<="),
            Operator::Shreq => s = String::from(">>="),
        }
        write!(f, "{}", s)
    }
}

impl Operator {
    /// Return the precedence of the operator.
    pub fn get_precedence(&self) -> u16 {
        match self {
            Operator::Dot => 14,
            Operator::Indir => 14,
            Operator::Not => 13,
            Operator::BNot => 13,
            Operator::Inc => 13,
            Operator::Dec => 13,
            Operator::Recv => 13,
            Operator::Avail => 13,
            Operator::Star => 12,
            Operator::Div => 12,
            Operator::Mod => 12,
            Operator::Add => 11,
            Operator::Sub => 11,
            Operator::Shl => 10,
            Operator::Shr => 10,
            Operator::Iter => 9,
            Operator::Leq => 8,
            Operator::Lss => 8,
            Operator::Grt => 8,
            Operator::Geq => 8,
            Operator::Eqeq => 7,
            Operator::Neq => 7,
            Operator::Amp => 6,
            Operator::Or => 5,
            Operator::Xor => 4,
            Operator::LAnd => 3,
            Operator::LOr => 2,
            Operator::Snd => 1,
            Operator::Asgn => 1,
            Operator::Pasgn => 1,
            Operator::Addeq => 1,
            Operator::Subeq => 1,
            Operator::Muleq => 1,
            Operator::Diveq => 1,
            Operator::Modeq => 1,
            Operator::Andeq => 1,
            Operator::Oreq => 1,
            Operator::Xoreq => 1,
            Operator::Shleq => 1,
            Operator::Shreq => 1,
        }
    }
}

/// Enum used to represent Alef delimiters.
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Delimiter {
    /// Left brace delimiter: "{".
    Lbrace,
    /// Right brace delimiter: "}".
    Rbrace,
    /// Left bracket delimiter: "[".
    Lbrack,
    /// Right bracket delimiter: "]".
    Rbrack,
    /// Left parenthesis delimiter: ")".
    Lparen,
    /// Right parenthesis delimiter: "(".
    Rparen,
    /// Semicolon delimiter: ";".
    Semi,
    /// Comma delimiter: ",".
    Comma,
    /// Colon delimiter: ":".
    Colon,
}

impl fmt::Display for Delimiter {
    /// Format a delimiter object.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        match *self {
            Delimiter::Lbrace => s = String::from("{"),
            Delimiter::Rbrace => s = String::from("}"),
            Delimiter::Lbrack => s = String::from("["),
            Delimiter::Rbrack => s = String::from("]"),
            Delimiter::Lparen => s = String::from("("),
            Delimiter::Rparen => s = String::from(")"),
            Delimiter::Semi => s = String::from(";"),
            Delimiter::Comma => s = String::from(","),
            Delimiter::Colon => s = String::from(":"),
        }
        write!(f, "{}", s)
    }
}

/// Enum used to represent Alef keywords.
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Keyword {
    /// Adt keyword: "adt".
    Adt,
    /// Aggr keyword: "aggr".
    Aggr,
    /// Alloc keyword: "alloc".
    Alloc,
    /// Alt keyword: "alt".
    Alt,
    /// Become keyword: "become".
    Become,
    /// Break keyword: "break".
    Break,
    /// Case keyword: "case".
    Case,
    /// Check keyword: "check".
    Check,
    /// Continue keyword: "continue".
    Continue,
    /// Default keyword: "default".
    Default,
    /// Do keyword: "do".
    Do,
    /// Else keyword: "else".
    Else,
    /// Enum keyword: "enum".
    Enum,
    /// Extern keyword: "extern".
    Extern,
    /// For keyword: "for".
    For,
    /// Goto keyword: "goto".
    Goto,
    /// If keyword: "if".
    If,
    /// Intern keyword: "intern".
    Intern,
    /// Nil keyword: "nil".
    Nil,
    /// Par keyword: "par".
    Par,
    /// Proc keyword: "proc".
    Proc,
    /// Private keyword: "private".
    Private,
    /// Raise keyword: "raise".
    Raise,
    /// Rescue keyword: "rescue".
    Rescue,
    /// Return keyword: "return".
    Return,
    /// Sizeof keyword: "sizeof".
    Sizeof,
    /// Switch keyword: "switch".
    Switch,
    /// Task keyword: "task".
    Task,
    /// Tuple keyword: "tuple".
    Tuple,
    /// Typedef keyword: "typedef".
    Typedef,
    /// Typeof keyword: "typeof".
    Typeof,
    /// Unalloc keyword: "unalloc".
    Unalloc,
    /// Union keyword: "union".
    Union,
    /// While keyword: "while".
    While,
    /// Zerox keyword: "zerox".
    Zerox,
}

impl fmt::Display for Keyword {
    /// Format a keyword object.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        match *self {
            Keyword::Adt => s = String::from("adt"),
            Keyword::Aggr => s = String::from("aggr"),
            Keyword::Alloc => s = String::from("alloc"),
            Keyword::Alt => s = String::from("alt"),
            Keyword::Become => s = String::from("become"),
            Keyword::Break => s = String::from("Break"),
            Keyword::Case => s = String::from("case"),
            Keyword::Check => s = String::from("check"),
            Keyword::Continue => s = String::from("continue"),
            Keyword::Default => s = String::from("default"),
            Keyword::Do => s = String::from("do"),
            Keyword::Else => s = String::from("else"),
            Keyword::Enum => s = String::from("enum"),
            Keyword::Extern => s = String::from("extern"),
            Keyword::For => s = String::from("for"),
            Keyword::Goto => s = String::from("goto"),
            Keyword::If => s = String::from("if"),
            Keyword::Intern => s = String::from("intern"),
            Keyword::Nil => s = String::from("nil"),
            Keyword::Par => s = String::from("par"),
            Keyword::Proc => s = String::from("proc"),
            Keyword::Private => s = String::from("private"),
            Keyword::Raise => s = String::from("raise"),
            Keyword::Rescue => s = String::from("rescue"),
            Keyword::Return => s = String::from("return"),
            Keyword::Sizeof => s = String::from("sizeof"),
            Keyword::Switch => s = String::from("switch"),
            Keyword::Task => s = String::from("task"),
            Keyword::Tuple => s = String::from("tuple"),
            Keyword::Typedef => s = String::from("typedef"),
            Keyword::Typeof => s = String::from("typeof"),
            Keyword::Unalloc => s = String::from("unalloc"),
            Keyword::Union => s = String::from("union"),
            Keyword::While => s = String::from("while"),
            Keyword::Zerox => s = String::from("zerox"),
        }
        write!(f, "{}", s)
    }
}

/// Enum used to represent Alef tokens.
#[derive(Debug, Clone)]
pub enum Token {
    /// The token is not valid.
    Invalid(Range),

    /// The token indicates the end of the input.
    End(Range),

    /// The token is a keyword.
    Keyword(Range, Keyword),

    /// The token is an operator.
    Operator(Range, Operator),

    /// The token is delimiter.
    Delimiter(Range, Delimiter),

    /// The token is an identifier.
    Identifier(Range, String),

    /// The token is an integer constant.
    Integer(Range, bool, i64),

    /// The token is a floating point constant.
    Float(Range, bool, f64),

    /// The token is a character constant.
    Character(Range, bool, char),

    /// The token is an ASCII string.
    String(Range, bool, String),

    /// The token is an arbitrary string.
    Runestring(Range, bool, String),
}

impl fmt::Display for Token {
    /// Format a keyword object.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        match self {
            Token::Invalid(_) => s = String::from("invalid"),
            Token::End(_) => s = String::from("eof"),
            Token::Keyword(_, ref k) => {
                let str = k.to_string();
                s = format!("keyword \"{}\"", str)
            }
            Token::Operator(_, ref o) => {
                let str = o.to_string();
                s = format!("operator \"{}\"", str)
            }
            Token::Delimiter(_, ref d) => {
                let str = d.to_string();
                s = format!("delimiter \"{}\"", str)
            }
            Token::Identifier(_, ref n) => s = format!("identifier \"{}\"", n.clone()),
            Token::Integer(_, _, ref i) => s = format!("integer {}", i),
            Token::Float(_, _, ref f) => s = format!("float {}", f),

            Token::Character(_, _, ref c) => s = format!("char \'{}\'", c.escape_default()),
            Token::String(_, _, ref t) => s = format!("string \"{}\"", t.escape_default()),
            Token::Runestring(_, _, ref r) => s = format!("string \"{}\"", r.escape_default()),
        }
        write!(f, "{}", s)
    }
}

impl Token {
    pub fn is_end(&self) -> bool {
        if let Token::End(..) = self {
            return true;
        }
        false
    }

    pub fn is_invalid(&self) -> bool {
        if let Token::Invalid(_) = self {
            return true;
        }
        false
    }

    pub fn is_any_delimiter(&self) -> bool {
        if let Token::Delimiter(..) = self {
            return true;
        }
        false
    }

    pub fn is_delimiter(&self, del: Delimiter) -> bool {
        if let Token::Delimiter(.., tdel) = self {
            return del == *tdel;
        }
        false
    }

    pub fn is_in_deli_set(&self, set: std::collections::hash_set::HashSet<Delimiter>) -> bool {
        if let Token::Delimiter(.., tdel) = self {
            return set.contains(tdel);
        }
        false
    }

    pub fn is_any_keyword(&self) -> bool {
        if let Token::Keyword(..) = self {
            return true;
        }
        false
    }

    pub fn is_keyword(&self, key: Keyword) -> bool {
        if let Token::Keyword(.., tkey) = self {
            return key == *tkey;
        }
        false
    }

    pub fn is_in_key_set(&self, set: std::collections::hash_set::HashSet<Keyword>) -> bool {
        if let Token::Keyword(.., tkey) = self {
            return set.contains(tkey);
        }
        false
    }

    pub fn is_any_operator(&self) -> bool {
        if let Token::Operator(..) = self {
            return true;
        }
        false
    }

    pub fn is_operator(&self, op: Operator) -> bool {
        if let Token::Operator(.., top) = self {
            return op == *top;
        }
        false
    }

    pub fn is_in_op_set(&self, set: std::collections::hash_set::HashSet<Operator>) -> bool {
        if let Token::Operator(.., top) = self {
            return set.contains(top);
        }
        false
    }

    pub fn is_identifier(&self) -> bool {
        if let Token::Identifier(..) = self {
            return true;
        }
        false
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Integer(_, _, _)
                | Token::Float(_, _, _)
                | Token::Character(_, _, _)
                | Token::String(_, _, _)
                | Token::Runestring(_, _, _)
        )
    }
    pub fn get_range(&self) -> Range {
        match self {
            Token::Invalid(r) => r.clone(),
            Token::End(r) => r.clone(),
            Token::Keyword(r, _) => r.clone(),
            Token::Operator(r, _) => r.clone(),
            Token::Delimiter(r, _) => r.clone(),
            Token::Identifier(r, _) => r.clone(),
            Token::Integer(r, _, _) => r.clone(),
            Token::Float(r, _, _) => r.clone(),
            Token::Character(r, _, _) => r.clone(),
            Token::String(r, _, _) => r.clone(),
            Token::Runestring(r, _, _) => r.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::source::loc::generate_unknown_range;
    use std::collections::HashSet;
    #[test]
    fn op_tok() {
        let range = generate_unknown_range();
        let tok = Token::Operator(range, Operator::Add);

        let dset = HashSet::from([Delimiter::Colon]);
        let oset = HashSet::from([Operator::Add]);
        let kset = HashSet::from([Keyword::Do]);

        assert_eq!(tok.is_any_delimiter(), false);
        assert_eq!(tok.is_delimiter(Delimiter::Colon), false);
        assert_eq!(tok.is_any_keyword(), false);
        assert_eq!(tok.is_keyword(Keyword::Adt), false);
        assert_eq!(tok.is_any_operator(), true);
        assert_eq!(tok.is_operator(Operator::Addeq), false);
        assert_eq!(tok.is_operator(Operator::Add), true);
        assert_eq!(tok.is_in_deli_set(dset), false);
        assert_eq!(tok.is_in_key_set(kset), false);
        assert_eq!(tok.is_in_op_set(oset), true);
        assert_eq!(tok.is_end(), false);
        assert_eq!(tok.is_invalid(), false);
        assert_eq!(tok.is_identifier(), false);
        assert_eq!(tok.is_literal(), false);
    }

    #[test]
    fn key_tok() {
        let range = generate_unknown_range();
        let tok = Token::Keyword(range, Keyword::Adt);

        let dset = HashSet::from([Delimiter::Colon]);
        let oset = HashSet::from([Operator::Add]);
        let kset = HashSet::from([Keyword::Adt]);

        assert_eq!(tok.is_in_deli_set(dset), false);
        assert_eq!(tok.is_in_key_set(kset), true);
        assert_eq!(tok.is_in_op_set(oset), false);

        assert_eq!(tok.is_any_delimiter(), false);
        assert_eq!(tok.is_delimiter(Delimiter::Colon), false);
        assert_eq!(tok.is_any_operator(), false);
        assert_eq!(tok.is_operator(Operator::Addeq), false);
        assert_eq!(tok.is_any_keyword(), true);
        assert_eq!(tok.is_keyword(Keyword::Aggr), false);
        assert_eq!(tok.is_keyword(Keyword::Adt), true);
        assert_eq!(tok.is_end(), false);
        assert_eq!(tok.is_invalid(), false);
        assert_eq!(tok.is_identifier(), false);
        assert_eq!(tok.is_literal(), false);
    }

    #[test]
    fn del_tok() {
        let range = generate_unknown_range();
        let tok = Token::Delimiter(range, Delimiter::Colon);

        let dset = HashSet::from([Delimiter::Colon]);
        let oset = HashSet::from([Operator::Add]);
        let kset = HashSet::from([Keyword::Do]);

        assert_eq!(tok.is_in_deli_set(dset), true);
        assert_eq!(tok.is_in_key_set(kset), false);
        assert_eq!(tok.is_in_op_set(oset), false);

        assert_eq!(tok.is_any_keyword(), false);
        assert_eq!(tok.is_any_operator(), false);
        assert_eq!(tok.is_operator(Operator::Addeq), false);
        assert_eq!(tok.is_keyword(Keyword::Aggr), false);
        assert_eq!(tok.is_any_delimiter(), true);
        assert_eq!(tok.is_delimiter(Delimiter::Lbrace), false);
        assert_eq!(tok.is_delimiter(Delimiter::Colon), true);
        assert_eq!(tok.is_end(), false);
        assert_eq!(tok.is_invalid(), false);
        assert_eq!(tok.is_identifier(), false);
        assert_eq!(tok.is_literal(), false);
    }

    #[test]
    fn op() {
        let op = Operator::Add;

        assert_eq!(op.to_string(), "+");
        assert_eq!(op.get_precedence(), 11);
    }

    #[test]
    fn del() {
        let del = Delimiter::Semi;
        assert_eq!(del.to_string(), ";");
    }

    #[test]
    fn key() {
        let key = Keyword::Adt;
        assert_eq!(key.to_string(), "adt");
    }
}
