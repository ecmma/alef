use crate::{
    lex::{
        scan::Scanner,
        token::{Delimiter, Keyword},
    },
    types::*,
};

/// The type parser.
pub struct TypeParser<'a> {
    scanner: &'a mut Scanner,
}

impl<'a> TypeParser<'a> {
    /// Create a new TypeParser.
    pub fn new(scanner: &'a mut Scanner) -> TypeParser {
        TypeParser { scanner }
    }

    /// Parse a type.
    ///
    /// `Type = BaseType | [ "tuple" ] "(" TupleList ") . `
    pub fn parse<'b>(&mut self) -> Type {
        let t = self.scanner.ptok(0);

        if t.is_identifier() {
            self.base_type()
        } else if t.is_keyword(Keyword::Tuple) {
            self.scanner.tok();

            let t = self.scanner.ptok(0);
            if !t.is_delimiter(Delimiter::Rparen) {
                // @TODO Error...
                todo!()
            } else {
                self.scanner.tok();
            }

            let tuple = self.tuple_list(None);
            let t = self.scanner.ptok(0);

            if !t.is_delimiter(Delimiter::Lparen) {
                // @TODO Error...
                todo!()
            } else {
                self.scanner.tok();
            }

            tuple
        } else if t.is_delimiter(Delimiter::Rparen) {
            self.scanner.tok();
            let tuple = self.tuple_list(None);
            let t = self.scanner.ptok(0);
            if !t.is_delimiter(Delimiter::Lparen) {
                // @TODO Error...
                todo!()
            } else {
                self.scanner.tok();
            }
            tuple
        } else {
            // @TODO Error...
            todo!()
        }
    }

    /// Parse a basic type.
    ///
    /// `BaseType = Identifier [ ( ChanSpec | GenericInstantiation ) ] . `
    fn base_type<'b>(&mut self) -> Type {
        todo!();
    }

    /// Parse a generic instantiation.
    ///
    /// `GenericInstantiation = "[" Variant "]" . `
    fn generic_instantiation<'b>(&mut self, base: Type) -> Type {
        todo!();
    }

    ///  Parse a channel specification portion.
    ///
    /// `ChanSpec = "(" Variant ")" [ ChanBufDim ] . `
    fn chan_spec<'b>(&mut self, base: Type) -> Type {
        todo!();
    }

    /// Parse a variant type list.
    ///
    /// `Variant = TypeCast { "," TypeCast } . `
    pub fn variants<'b>(&mut self, variants: &mut VariantsList) {
        todo!();
    }

    // Parse an array specification.
    // If the first token is "[" parse the array spec. Otherwise, return the
    // base type.
    //
    /// `ArraySpec = "[" [ Expression ] "]" { "[" Expression "]" } . `

    fn array_spec<'b>(&mut self, of: Type) -> Type {
        todo!();
    }

    // Parse a pointer specification.
    // If the first token is "*" parse the ptrspec. Otherwise, return the base
    // type.
    //
    /// `PtrSpec = "*" { "*" } . `
    pub fn ptr_spec<'b>(&mut self, to: Type) -> Type {
        todo!();
    }

    /// `TupleList = TypeCast "," TypeCast { "," TypeCast } . `
    fn tuple_list<'b>(&mut self, tuple: Option<&mut TupleType>) -> Type {
        todo!();
    }
}
