use super::expect_tok;
use crate::{
    ast::node::{expr::Expr, expr::LitKind, Node, NodeList},
    lex::{
        scan::Scanner,
        token::{Operator, Token},
    },
    source::loc::Range,
    types::*,
};

/// The expression parser.
pub struct ExprParser<'a> {
    scanner: &'a mut Scanner,
}

impl<'a> ExprParser<'a> {
    pub fn new(scanner: &'a mut Scanner) -> ExprParser {
        ExprParser { scanner }
    }
    /// `InitExpression = Expression | ArrayElementInit | MemberInit | BlockInit .  `
    pub fn init_expr(obj: Node) -> Node {
        todo!();
    }

    /// `ArrayElementInit   = "[" Expression "]"  ( Expression | BlockInit ) . `
    pub fn array_element_init_expr(obj: Node) -> Node {
        todo!();
    }

    /// `BlockInit = "{" [ InitExpression { ","  InitExpression } ] "}" . `
    pub fn block_init_expr(obj: Node) -> Node {
        todo!();
    }

    /// `MemberInit = "." Identifier Expression . `
    pub fn member_init_expr(obj: Node) -> Node {
        todo!();
    }

    /// Parse a primary expression.
    ///
    /// `PrimaryExpression = Identifier | Literal | "nil" | [ "tuple" ] "(" ExpressionList ")" .`
    pub fn primary_expr(&mut self) -> Node {
        let t = self.scanner.ptok(0);
        let make_lit = |range, kind| {
            Node::Expr(Box::new(Expr::Literal {
                range,
                atype: None,
                kind,
            }))
        };

        match t {
            Token::Identifier(range, id) => Node::Expr(Box::new(Expr::Identifier {
                range,
                atype: None,
                name: id,
                declared: None,
            })),
            Token::Float(range, mal, float) => make_lit(range, LitKind::Float(float)),
            Token::Integer(range, mal, int) => make_lit(range, LitKind::Int(int)),
            Token::Character(range, mal, char) => make_lit(range, LitKind::Char(char)),
            Token::String(range, mal, string) => make_lit(range, LitKind::String(string)),
            Token::Runestring(range, mal, runestring) => {
                make_lit(range, LitKind::String(runestring))
            }
            _ => {
                // @TODO ERRSYNC
                todo!()
            }
        }
    }

    /// Parse an ADT namecall expression.
    ///
    /// `AdtNameCall = "." Identifier "." Identifier "(" [ ExpressionList ] ") . `
    pub fn adt_name_call_expr(&mut self) -> Node {
        let t = self.scanner.ptok(0);
        let start_r = t.get_range().start.box_clone();
        assert!(t.is_operator(Operator::Dot));
        self.scanner.tok();

        let t = self.scanner.ptok(0);

        let adt_name = if let Token::Identifier(_, id) = t {
            id
        } else {
            // @TODO ERRSYNC
            todo!()
        };

        expect_tok!(self, is_operator, Operator::Dot);

        let t = self.scanner.ptok(0);
        let func_name = if let Token::Identifier(_, id) = t {
            id
        } else {
            // @TODO ERRSYNC
            todo!()
        };

        let range = self.scanner.src.get_range(start_r.as_ref(), None);
        let left = Node::Expr(Box::new(Expr::AdtNamecall {
            range,
            atype: None,
            func_name,
            adt_name,
            adt: None,
        }));
        self.func_call_expr(left)
    }

    /// Parse a postfix operand expression.
    ///
    /// `PostfixOperand = ArrayAccess | FuncCall | MemberAccess | IndirectAccess | UnaryPostfix .`
    pub fn parse_postfix_operand_expr(&mut self, left: Node) -> Node {
        todo!();
    }

    ///  Parse a postfix expression.
    ///
    ///  `PostfixExpression = ( PrimaryExpression | AdtNameCall ) { PostfixOperand } .`
    pub fn postfix_expr(&mut self) -> Node {
        todo!();
    }

    /// Parse an array access expression.
    ///
    /// `ArrayAccess = "[" Expression "]" .`
    pub fn array_access_expr(_left: Node) -> Node {
        todo!();
    }

    /// Parse a function call expression.
    ///
    /// `FuncCall = "(" [ ExpressionList ] ")" .`
    pub fn func_call_expr(&mut self, left: Node) -> Node {
        todo!();
    }

    /// Parse a member access expression.
    ///
    /// `MemberAccess = "." Identifier .`
    pub fn member_access_expr(_left: Node) -> Node {
        todo!();
    }

    /// Parse an indirect access expression.
    ///
    /// `IndirectAccess = "->" Identifier . `
    pub fn indirect_access_expr(_left: Node) -> Node {
        todo!();
    }

    /// Parse a unary postfix expression.
    ///
    /// `UnaryPostfix = "++" | "--" | "?" .`
    pub fn unary_postfix_expr(_left: Node) -> Node {
        todo!();
    }

    /// Parse an expression list expression.
    ///
    /// `ExpressionList = Expression { "," Expression } .`
    pub fn expression_list_expr(_node: NodeList) -> Node {
        todo!()
    }

    /// Parse a unary expression.
    ///
    /// `UnaryExpression = PostfixExpression | UnaryPrefix | CastPrefix .`
    pub fn unary_expr() -> Node {
        todo!();
    }

    /// Parse a unary prefix expression.
    ///
    /// `UnaryPrefix = ( "<-" | "++" | "--" | "zerox" ) UnaryExpression .`
    pub fn unary_prefix_expr() -> Node {
        todo!();
    }

    /// Parse a prefix cast expression.
    ///
    /// `CastPrefix = UnaryOperator Term .`
    /// `UnaryOperator = ( "?" | "*" | "!" | "+" | "-" | "~" | "sizeof" ) .`
    pub fn cast_prefix_expr() -> Node {
        todo!();
    }

    /// Parse a term expression.
    ///
    /// `Term = UnaryExpression | CastExpression | AllocExpression .`
    pub fn term_expr() -> Node {
        todo!();
    }

    ///  Parse a cast expression.
    ///
    /// `CastExpression = "(" TypeCast ")" Term .`
    pub fn cast_expr() -> Node {
        todo!();
    }

    /// Parse an alloc expression.
    ///
    /// `AllocExpression = "(" "alloc" Identifier ")" Term .`
    pub fn alloc_expr() -> Node {
        todo!();
    }

    /// Parse a type cast expression.
    ///
    /// `TypeCast = ( BaseType [ PtrSpec ]  [ FuncCast ] ) | "tuple" "(" TupleList ")" .`
    pub fn type_cast_expr() -> Type {
        todo!();
    }

    /// Parse a function cast expression.
    ///
    /// `FuncCast = [ "(" [ PtrSpec ] ")" "(" [ ParamList ] ")" ]`
    pub fn func_cast_expr(_return: Type) -> Node {
        todo!();
    }

    /// Parse an expression.
    ///
    /// `Expression = Term | Expression BinaryOp Expression .`
    pub fn expression<'b>(&'b mut self, precedence: i32, left: Option<Node>) -> Node {
        todo!();
    }

    /// Get an expression node for the given operator and operands.
    fn get_bin_expression_node(_loc: Range, _op: Operator, _left: Node, _right: Node) -> Node {
        todo!();
    }
}
