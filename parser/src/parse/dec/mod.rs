mod err;
use super::expect_tok;
use super::ty::TypeParser;
use crate::{
    ast::{
        node::{dec::Dec, *},
        scope::Scope,
    },
    diagnostic::diag,
    lex::{
        scan::Scanner,
        token::{Delimiter, Keyword, Operator, Token},
    },
    source::loc::Range,
    sym::ScopedTable,
    types::*,
};
use err::*;
use paste::paste;
use std::collections::hash_set::HashSet;

/// The declaration parser.
pub struct DeclParser<'a> {
    scanner: &'a mut Scanner,
}

impl<'a> DeclParser<'a> {
    pub fn new(scanner: &'a mut Scanner) -> DeclParser<'a> {
        DeclParser { scanner }
    }

    ///  `Declaration = [ Visibility ] ( SimpleDecl | ComplexDecl | TypeDefs ) . `
    pub fn declaration(&mut self) -> Option<Node> {
        if self.scanner.ptok(0).is_end() {
            return None;
        }
        let scope = self.visibility();
        let t = self.scanner.ptok(0);

        // ComplexDecl begin with one of `aggr`, `union`, `enum`, `adt`.
        let c_dec_set = HashSet::from([Keyword::Aggr, Keyword::Union, Keyword::Adt, Keyword::Enum]);
        if t.is_in_key_set(c_dec_set) {
            let cdec = self.complex_decl(scope);
        } else if t.is_identifier() {
            let sdec = self.simple_decl(scope);
        } else if t.is_keyword(Keyword::Typedef) {
            let tdef = self.type_def(scope);
        }
        diag(Box::new(ParseDeclError {
            source_name: self.scanner.src.get_name(),
            range: Range {
                start: t.get_range().start,
                end: t.get_range().end,
                content: self.scanner.get_line(),
            },
            tok: t,
        }));

        self.scanner.tok();
        self.declaration()
    }

    /// `Visibility = "intern" | "extern" . `
    fn visibility(&mut self) -> Scope {
        let t = self.scanner.ptok(0);

        if t.is_keyword(Keyword::Extern) {
            Scope::External
        } else if t.is_keyword(Keyword::Intern) {
            Scope::Internal
        } else {
            Scope::Default
        }
    }

    /// `SimpleDecl = Type [ PtrSpec ] ( FuncPtr | BaseDecl ) . `
    fn simple_decl(&mut self, scope: Scope) -> Node {
        let mut ty_parser = TypeParser::new(self.scanner);
        let base_type = ty_parser.parse();
        let ptrd_type = ty_parser.ptr_spec(base_type.clone());
        let t = self.scanner.ptok(0);

        if t.is_delimiter(Delimiter::Rparen) {
            self.func_ptr(scope, base_type, ptrd_type)
        } else if t.is_identifier() {
            self.base_decl(scope, base_type, ptrd_type)
        } else {
            // @TODO ERRSYNC
            todo!();
        }
    }

    /// `FuncPtr = "(" [ PtrSpec ] Identifier ( FuncPtrFuncDecl | FuncPtrVarDecl ) . `
    fn func_ptr(&mut self, scope: Scope, base_type: Type, ptrd_type: Type) -> Node {
        let t = self.scanner.ptok(0);

        assert!(t.is_delimiter(Delimiter::Rparen));
        self.scanner.tok();
        let t = self.scanner.ptok(0);

        // don't really create the PtrType from the ptrspec..
        let mut indirection = 0;
        while t.is_operator(Operator::Star) {
            indirection += 1;
            self.scanner.tok();
            let t = self.scanner.ptok(0);
        }

        let name = if let Token::Identifier(range, id) = t {
            self.scanner.tok();
            id
        } else {
            // @TODO ERRSYNC
            String::new()
        };

        let t = self.scanner.ptok(0);
        if t.is_delimiter(Delimiter::Rparen) {
            self.func_ptr_func_decl(name, scope, ptrd_type, indirection)
        } else if t.is_delimiter(Delimiter::Lparen) || t.is_delimiter(Delimiter::Lbrack) {
            self.func_ptr_var_decl(name, scope, base_type, ptrd_type, indirection)
        } else {
            // @TODO ERRSYNC
            todo!()
        }
    }

    /// `FuncPtrFuncDecl = "(" [ ParamList ] ")" ")" "(" [ ParamList ] ")" ( ";" | Block ) . `
    fn func_ptr_func_decl(
        &mut self,
        identifier: String,
        scope: Scope,
        return_type: Type,
        indirection: usize,
    ) -> Node {
        let t = self.scanner.ptok(0);
        assert!(t.is_delimiter(Delimiter::Rparen));
        self.scanner.tok();
        let mut inner_func_params = vec![];
        let inner_func_params = self.func_param_list(&mut inner_func_params);

        expect_tok!(self, is_delimiter, Delimiter::Lparen);
        expect_tok!(self, is_delimiter, Delimiter::Lparen);
        expect_tok!(self, is_delimiter, Delimiter::Rparen);

        let mut func_params = vec![];
        let func_params = self.func_param_list(&mut func_params);

        expect_tok!(self, is_delimiter, Delimiter::Lparen);

        let t = self.scanner.ptok(0);
        if t.is_delimiter(Delimiter::Semi) {
            self.scanner.tok();
            // @TODO
            // Something to return..
            todo!()
        } else if t.is_delimiter(Delimiter::Lbrace) {
            // @TODO
            // Statement Parser
            // parse block
            todo!()
        } else {
            // @TODO
            // Consider as missing ";"
            todo!()
        }
    }

    /// `[ ArraySpec ] ")" "(" [ ParamList ] ")" [ "=" InitExpression ] ( ";" | "," [ PtrSpec ] ( FuncPtrDeclarator FuncPtrVarDecl | Identifier VarDecl )) .`
    fn func_ptr_var_decl(
        &mut self,
        identifier: String,
        scope: Scope,
        base_type: Type,
        return_type: Type,
        indirection: usize,
    ) -> Node {
        todo!();
    }

    /// `Parse a list of parameters for a function. `
    fn func_param_list(&mut self, params: &mut Vec<Type>) {
        todo!();
    }

    /// `Param = SimpleParam | TupleParam  | "..." . `
    fn parse_func_param(&mut self, params: &mut Vec<Type>) {
        todo!();
    }

    /// `SimpleParam = BaseType [ [ PtrSpec ] ( BaseParam |  FuncPtrParam ) ] . `
    fn simple_param(&mut self) -> Node {
        todo!();
    }

    /// `BaseParam = [ Identifier ] [ ArraySpec ] . `
    fn base_param(&mut self, d: Node) -> Node {
        todo!();
    }

    /// `FuncPtrParam = "(" [ PtrSpec ] [ Identifier ] [ ArraySpec ] ")"  "(" [ ParamList ] ")" .     `
    fn func_ptr_param(&mut self, ty: Type) -> Node {
        todo!();
    }

    /// `TupleParam = "tuple" "(" TupleList ")" [ [ PtrSpec ] ( BaseParam | FuncPtrParam ) ] . `
    fn tuple_param(&mut self) -> Node {
        todo!();
    }

    /// `BaseDecl = Identifier ( FuncDecl | VarDecl | MethDecl ) . `
    fn base_decl(&mut self, scope: Scope, base_type: Type, ptrd_type: Type) -> Node {
        todo!();
    }

    /// `FuncDecl = "(" [ ParamList ] ")" ( ";" | Block ) . `
    fn func_decl(&mut self, scope: Scope, return_type: Type) -> Node {
        todo!();
    }

    /// `VarDecl = [ ArraySpec ] [ "=" InitExpression ] (";" | "," [ PtrSpec ] "(" [ PtrSpec ] Identifier FuncPtrVarDecl | Identifier VarDecl ) . `
    fn var_decl(&mut self, scope: Scope, base_type: Type, ptrd_type: Type) -> Node {
        todo!();
    }

    /// `MethDecl = "." Identifier "(" [ ParamList ] ")" Block . `
    fn method_decl(&mut self, scope: Scope, return_type: Type) -> Node {
        todo!();
    }

    /// `ComplexDecl = ( AggrDecl | UnionDecl | AdtDecl | EnumDecl ) ";" . `
    fn complex_decl(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `AggrDecl = "aggr" [ Identifier ] "{" { AggrUnionMember } "}"  `
    fn aggr_decl(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `UnionDecl = "union" [ Identifier ] "{" { AggrUnionMember } "}"  `
    fn union_decl(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `AggrUnionMember = ComplexDefs | VariableMember . `
    fn aggr_union_member(&mut self) -> Node {
        todo!();
    }

    /// ` VariableMember = BaseType [ [ PtrSpec ]  ( SimpleMember | FuncPtrMember) {"," [ PtrSpec ] ( SimpleMember | FuncPtrMember ) } ] ";" . `
    fn aggr_union_variable_member(&mut self, base_type: Type) -> Node {
        todo!();
    }

    /// `  SimpleMember =  Identifier [ ArraySpec ] . `
    fn aggr_union_simple_member(&mut self, base_type: Type) -> Node {
        todo!();
    }

    /// ` FuncPtrMember = "(" [ PtrSpec ] Identifier [ ArraySpec ] ")" "(" [ParamList] ")" . `
    fn aggr_union_func_ptr_member(&mut self, return_type: Type) -> Node {
        todo!();
    }

    /// `AdtDecl = "adt" [ Identifier ] [ "[" AdtGenSpec "]" ] "{" { AdtMember } "}" [ Identifier ] . `
    fn adt_decl(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `AdtGenSpec = Identifier { "," Identifier } . `
    fn adt_gen_spec(&mut self, adt: AdtType) -> Node {
        todo!();
    }

    /// `AdtMember = [ Visibility ] Type [ [ PtrSPec ] ( AdtFuncPtrMember | AdtBaseMember ) ] ";" . `
    fn adt_member(&mut self) -> Node {
        todo!();
    }

    /// `AdtFuncPtrMember = "(" [ PtrSpec ] Identifier ( AdtFuncPtrMethodMember | AdtFuncPtrVarMember ) . `
    fn adt_func_ptr_member(&mut self, scope: Scope, base_type: Type, return_type: Type) -> Node {
        todo!();
    }

    /// `AdtFuncPtrMethodMember = "(" [ AdtMethodRefParam [ "," ParamList ] ] | ParamList ")" ")" "(" [ ParamList ] ")" . `
    /* The first token will be an identifier. */
    fn adt_func_ptr_method_member(
        &mut self,
        scope: Scope,
        return_type: Type,
        indirection: usize,
    ) -> Node {
        todo!();
    }

    /// `AdtMethodRefParam = ( "*" | "." ) Identifier [ Identifier ] . `
    fn parse_adt_method_ref_param(&mut self) -> Node {
        todo!();
    }

    /// `AdtFuncPtrVarMember = [ ArraySpec ] ")" "(" [ ParamList ] ")"  [ "," [ PtrSpec ] ( "(" [ PtrSpec ] Identifier AdtFuncPtrVarMember | Identifier AdtVarMember ) ] .  `
    fn adt_func_ptr_var_member(
        &mut self,
        scope: Scope,
        base_type: Type,
        return_type: Type,
        indirection: usize,
    ) -> Node {
        todo!();
    }

    /// `AdtBaseMember = Identifier ( AdtMethodMember | AdtVarMember ) . `
    fn adt_base_member(&mut self, scope: Scope, base_type: Type, ptrd_type: Type) -> Node {
        todo!();
    }

    /// `AdtMethodMember = "(" [ AdtMethodRefParam [ "," ParamList ] ] | ParamList ")" .  `
    fn adt_method_member(&mut self, scope: Scope, return_type: Type) -> Node {
        todo!();
    }

    /// `AdtVarMember= [ ArraySpec ] [ "," [ PtrSpec ] ( "(" [ PtrSpec ] Identifier AdtFuncPtrVarMember | Identifier AdtVarMember ) ] .  `
    fn adt_var_member(&mut self, scope: Scope, base_type: Type, ptrd_type: Type) -> Node {
        todo!();
    }

    /// `EnumDecl = "enum" [ Identifier ] "{" { EnumMember } "}" . `
    fn enum_decl(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `EnumMember = Identifier [ "=" Expression ] `
    fn enum_member(&mut self) -> Node {
        todo!();
    }

    /// `TypeDefs = "typedef" ( PolyVarTypeDef | ForwardDef ) `
    fn type_def(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `PolyVarTypeDef = BaseType [ [PtrSpec] [ DerivedTypeDef | FuncPtrTypeDef ] ] ";" . `
    fn poly_var_type_def(&mut self, scope: Scope) -> Node {
        todo!();
    }

    /// `DerivedTypeDef = Identifier [ ArraySpec ] . `
    fn derived_type_def(&mut self, scope: Scope, base_type: Type) -> Node {
        todo!();
    }

    /// `FuncPtrTypeDef = "(" [ PtrSpec ] Identifier [ ArraySpec ] ")" "(" [ParamList] ")" .  `
    fn func_ptr_type_def(&mut self, scope: Scope, return_type: Type) -> Node {
        todo!();
    }

    /// `ForwardDef = ( "aggr" | "union" | "adt" ) Identifier ";" . `
    fn forward_type_def(&mut self, scope: Scope) -> Node {
        todo!();
    }
}
