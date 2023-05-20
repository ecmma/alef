use crate::ast::node::{dec::Dec, expr::Expr, stmt::Stmt, Node, Program};
use paste::paste;

macro_rules! gen_visit {
    ($node: ident, $param_name: ident, $type: ident, $($mutability:ident)?) => {
        paste! {
            fn [<visit_ $node _$param_name>](&mut self, $param_name: &$($mutability)? $type) -> Option<T> {
                let _ = $param_name;
                None
            }
        }
    };
}

macro_rules! gen_match {
    ($self: ident, $type:ident, $ltype:ident, $($node: ident, $lnode:ident),+) => {

        paste! {
        match $ltype {
            $(
                $type::$node { .. } => $self.[<visit_ $lnode _ $ltype>]($ltype),
            )+
        }
        }
    };
}

macro_rules! gen_visitor {
    ($visitor_trait_name:ident, $($mutability:ident)? ) => {
        trait $visitor_trait_name<T> {
            fn visit_program(&mut self, prog: &$($mutability)? Program) -> Option<T> {
                for dec in &$($mutability)? prog.decs {
                    self.visit_node(dec);
                }
                None
            }

            fn visit_node(&mut self, node: &$($mutability)? Node) -> Option<T> {
                match node {
                    Node::Dec(d) => self.visit_dec(d),
                    Node::Expr(e) => self.visit_expr(e),
                    Node::Stmt(s) => self.visit_stmt(s),
                }
            }

            fn visit_dec(&mut self, dec: &$($mutability)? Dec) -> Option<T> {
                gen_match!(
                    self, Dec, dec, Var, var, Function, function, Prototype, prototype, Method,
                    method, Adt, adt, Aggr, aggr, Union, union, Enum, enum, Typedef, typedef,
                    TypeParam, type_param, Forward, forward, Polydef, polydef, Basetype, basetype
                );
                None
            }

            gen_visit!(var, dec, Dec,$($mutability)?);
            gen_visit!(function, dec, Dec,$($mutability)?);
            gen_visit!(prototype, dec, Dec,$($mutability)?);
            gen_visit!(method, dec, Dec,$($mutability)?);
            gen_visit!(adt, dec, Dec,$($mutability)?);
            gen_visit!(aggr, dec, Dec,$($mutability)?);
            gen_visit!(union, dec, Dec,$($mutability)?);
            gen_visit!(enum, dec, Dec,$($mutability)?);
            gen_visit!(typedef, dec, Dec,$($mutability)?);
            gen_visit!(type_param, dec, Dec,$($mutability)?);
            gen_visit!(forward, dec, Dec,$($mutability)?);
            gen_visit!(polydef, dec, Dec,$($mutability)?);
            gen_visit!(basetype, dec, Dec,$($mutability)?);

            fn visit_expr(&mut self, expr: &$($mutability)? Expr) -> Option<T> {
                #[rustfmt::skip]
                gen_match!(self, Expr, expr,
                    Ellipsis, ellipsis,
                    Nil, nil,
                    Identifier, identifier,
                    Literal, literal,
                    Paren, paren,
                    Tuple, tuple,
                    ArrayAccess, array_access,
                    FuncCall, func_call,
                    AdtNamecall, adt_name_call,
                    TypeAccess, type_access,
                    IndirectTypeAccess, indirect_type_access,
                    Access, access,
                    IndirectAccess, indirect_access,
                    Postfix, postfix,
                    PrefixArith, prefix_arith,
                    ChanRecv, chan_recv,
                    ChanSend, chan_send,
                    CanChanSend, can_chan_send,
                    CanChanRecv, can_chan_recv,
                    Zerox, zerox,
                    Sizeof, sizeof,
                    SizeofType, sizeof_type,
                    Cast, cast,
                    Polycast, polycast,
                    BinaryArith, binary_arith,
                    BooleanArith, boolean_arith,
                    Comparison, comparison,
                    Shift, shift,
                    Iter, iter,
                    Assignment, assignment,
                    ArrayElementInit, array_element_init,
                    BlockInit, block_init,
                    MemberInit, member_init,
                    ImplicitCast, implicit_cast,
                    ImplicitPack, implicit_pack,
                    ImplicitUnpack, implicit_unpack,
                    ImplicitTypeAccess, implicit_type_access);
                None
            }

            gen_visit!(ellipsis, expr, Expr,$($mutability)?);
            gen_visit!(nil, expr, Expr,$($mutability)?);
            gen_visit!(identifier, expr, Expr,$($mutability)?);
            gen_visit!(literal, expr, Expr,$($mutability)?);
            gen_visit!(paren, expr, Expr,$($mutability)?);
            gen_visit!(tuple, expr, Expr,$($mutability)?);
            gen_visit!(array_access, expr, Expr,$($mutability)?);
            gen_visit!(func_call, expr, Expr,$($mutability)?);
            gen_visit!(adt_name_call, expr, Expr,$($mutability)?);
            gen_visit!(type_access, expr, Expr,$($mutability)?);
            gen_visit!(indirect_type_access, expr, Expr,$($mutability)?);
            gen_visit!(access, expr, Expr,$($mutability)?);
            gen_visit!(indirect_access, expr, Expr,$($mutability)?);
            gen_visit!(postfix, expr, Expr,$($mutability)?);
            gen_visit!(prefix_arith, expr, Expr,$($mutability)?);
            gen_visit!(chan_recv, expr, Expr,$($mutability)?);
            gen_visit!(chan_send, expr, Expr,$($mutability)?);
            gen_visit!(can_chan_send, expr, Expr,$($mutability)?);
            gen_visit!(can_chan_recv, expr, Expr,$($mutability)?);
            gen_visit!(zerox, expr, Expr,$($mutability)?);
            gen_visit!(sizeof, expr, Expr,$($mutability)?);
            gen_visit!(sizeof_type, expr, Expr,$($mutability)?);
            gen_visit!(cast, expr, Expr,$($mutability)?);
            gen_visit!(polycast, expr, Expr,$($mutability)?);
            gen_visit!(binary_arith, expr, Expr,$($mutability)?);
            gen_visit!(boolean_arith, expr, Expr,$($mutability)?);
            gen_visit!(comparison, expr, Expr,$($mutability)?);
            gen_visit!(shift, expr, Expr,$($mutability)?);
            gen_visit!(iter, expr, Expr,$($mutability)?);
            gen_visit!(assignment, expr, Expr,$($mutability)?);
            gen_visit!(array_element_init, expr, Expr,$($mutability)?);
            gen_visit!(block_init, expr, Expr,$($mutability)?);
            gen_visit!(member_init, expr, Expr,$($mutability)?);
            gen_visit!(implicit_cast, expr, Expr,$($mutability)?);
            gen_visit!(implicit_pack, expr, Expr,$($mutability)?);
            gen_visit!(implicit_unpack, expr, Expr,$($mutability)?);
            gen_visit!(implicit_type_access, expr, Expr,$($mutability)?);

            fn visit_stmt(&mut self, stmt: &$($mutability)? Stmt) -> Option<T> {
                #[rustfmt::skip]
                gen_match!(self, Stmt, stmt,
                            Empty, empty,
                            Expression, expression,
                            Label, label,
                            Block, block,
                            If, if,
                            Switch, switch,
                            SwitchCase, switch_case,
                            DefaultSwitchCase, default_switch_case,
                            Typeof, typeof,
                            TypeofCase, typeof_case,
                            DefaultTypeofCase, default_typeof_case,
                            Alt, alt,
                            AltCase, alt_case,
                            While, while,
                            Do, do,
                            For, for,
                            Goto, goto,
                            Continue, continue,
                            Break, break,
                            Return, return,
                            Become, become,
                            Raise, raise,
                            Rescue, rescue,
                            Check, check,
                            Proc, proc,
                            Task, task,
                            Par, par,
                            Alloc, alloc,
                            Unalloc, unalloc
                    );
                None
            }
            gen_visit!(empty, stmt, Stmt,$($mutability)?);
            gen_visit!(expression, stmt, Stmt,$($mutability)?);
            gen_visit!(label, stmt, Stmt,$($mutability)?);
            gen_visit!(block, stmt, Stmt,$($mutability)?);
            gen_visit!(if, stmt, Stmt,$($mutability)?);
            gen_visit!(switch, stmt, Stmt,$($mutability)?);
            gen_visit!(switch_case, stmt, Stmt,$($mutability)?);
            gen_visit!(default_switch_case, stmt, Stmt,$($mutability)?);
            gen_visit!(typeof, stmt, Stmt,$($mutability)?);
            gen_visit!(typeof_case, stmt, Stmt,$($mutability)?);
            gen_visit!(default_typeof_case, stmt, Stmt,$($mutability)?);
            gen_visit!(alt, stmt, Stmt,$($mutability)?);
            gen_visit!(alt_case, stmt, Stmt,$($mutability)?);
            gen_visit!(while, stmt, Stmt,$($mutability)?);
            gen_visit!(do, stmt, Stmt,$($mutability)?);
            gen_visit!(for, stmt, Stmt,$($mutability)?);
            gen_visit!(goto, stmt, Stmt,$($mutability)?);
            gen_visit!(continue, stmt, Stmt,$($mutability)?);
            gen_visit!(break, stmt, Stmt,$($mutability)?);
            gen_visit!(return, stmt, Stmt,$($mutability)?);
            gen_visit!(become, stmt, Stmt,$($mutability)?);
            gen_visit!(raise, stmt, Stmt,$($mutability)?);
            gen_visit!(rescue, stmt, Stmt,$($mutability)?);
            gen_visit!(check, stmt, Stmt,$($mutability)?);
            gen_visit!(proc, stmt, Stmt,$($mutability)?);
            gen_visit!(task, stmt, Stmt,$($mutability)?);
            gen_visit!(par, stmt, Stmt,$($mutability)?);
            gen_visit!(alloc, stmt, Stmt,$($mutability)?);
            gen_visit!(unalloc, stmt, Stmt,$($mutability)?);
        }
    };
}

gen_visitor!(ImmutableAstVisitor,);
gen_visitor!(MutableAstVisitor, mut);
