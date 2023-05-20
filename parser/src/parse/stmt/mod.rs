use crate::ast::node::{stmt::Stmt, Node};

pub struct StmtParser {}
impl StmtParser {
    /// `Statement = [ Expression ] ";" | LabelStmt | Block | SelectionStmn | LoopStmt | JumpStmn | ExceptionStmn | ProcessStmn | AllocationStmn .  `
    pub fn statement() -> Node {
        todo!();
    }

    /// `ExprStmt = [ Expression ] ";" . `
    fn expr_stmt() -> Node {
        todo!();
    }

    /// `LabelStmt = Identifier ":" Statement . `
    fn label_stmt() -> Node {
        todo!();
    }

    /// `Block = [ "!" ] "{" [ { AutomaticDeclarations } ] | { Statement } "}" .`
    fn block_stmt() -> Node {
        todo!();
    }

    /// `AutomaticDeclarations = Type [ PtrSpec ] ( FuncPtrDeclarator FuncPtrVarDecl  |  Identifier VarDecl) `
    fn parse_auto_decls(block: Stmt) -> Node {
        todo!();
    }

    /// `SelectionStmt = IfElseStmn | SwitchStmn | TypeofStmn | AltStmn .  `
    fn selection_stmt() -> Node {
        todo!();
    }

    /// `IfElseStmt = "if" "(" Expression ")" Statement [ "else" Statement ] .  `
    fn ifelse_stmt() -> Node {
        todo!();
    }

    /// `SwitchStmt = "switch" Expression SwitchBody .  `
    fn switch_stmt() -> Node {
        todo!();
    }

    /// `SwitchBody = ["!"] "{" { SwitchCase } "}" .  `
    fn switch_body(switch: Stmt) -> Node {
        todo!();
    }

    /// `SwitchCase = "case" Expression ":" { Statement } | "default" ":" { Statement } .  `
    fn switch_case() -> Node {
        todo!();
    }

    /// `TypeofStmt = "typeof" Expression TypeofBody .`
    fn typeof_stmt() -> Node {
        todo!();
    }

    /// `TypeofBody = ["!"] "{" { TypeofCase } "}" .`
    fn typeof_body(t_of: Stmt) -> Node {
        todo!();
    }

    /// `TypeofCase = "case" CastExpression ":" { Statement } | "default" ":" { Statement } .`
    fn typeof_case() -> Node {
        todo!();
    }

    /// `AltStmt = "alt" SwitchBody .`
    fn alt_stmt() -> Node {
        todo!();
    }

    /// `SwitchBody = ["!"] "{" { SwitchCase } "}" .`
    fn alt_body(alt: Stmt) -> Node {
        todo!();
    }

    /// `SwitchCase = "case" Expression ":" { Statement } | "default" ":" { Statement } .`
    fn alt_case() -> Node {
        todo!();
    }

    /// `LoopStmt = WhileStmn | DoStmn | ForStmn .`
    fn loop_stmt() -> Node {
        todo!();
    }

    /// `WhileStmt = "while" "(" Expression ")" Statement .`
    fn while_stmt() -> Node {
        todo!();
    }

    /// `DoStmt = "do" Statement "while" "(" Expression ")" .`
    fn do_stmt() -> Node {
        todo!();
    }

    /// `ForStmt = "for" "(" [ Expression ] ";" [ Expression ] ";" [ Expression ] ")" Statement .`
    fn for_stmt() -> Node {
        todo!();
    }

    /// `JumpStmt = GotoStmn | ContinueStmn | BreakStmn | ReturnStmn | BecomeStmt.`
    fn jump_stmt() -> Node {
        todo!();
    }

    /// `GotoStmt = "goto" Identifier ";" .`
    fn goto_stmt() -> Node {
        todo!();
    }

    /// `ContinueStmt = "continue" [ Literal ] ";" .`
    fn continue_stmt() -> Node {
        todo!();
    }

    /// `BreakStmt = "break" [ Literal ] ";" .`
    fn break_stmt() -> Node {
        todo!();
    }

    /// `ReturnStmt = "return" [ Expression ] ";" .`
    fn return_stmt() -> Node {
        todo!();
    }

    /// `BecomeStmt = "become" Expression ";" .`
    fn become_stmt() -> Node {
        todo!();
    }

    /// `ExceptionStmt = RaiseStmn | RescueStmn | CheckStmn .`
    fn exception_stmt() -> Node {
        todo!();
    }

    /// `RaiseStmt = "raise" [ Identifier ] .`
    fn raise_stmt() -> Node {
        todo!();
    }

    /// `RescueStmt = "rescue" ( Statement | Identifier Block ).`
    fn rescue_stmt() -> Node {
        todo!();
    }

    /// `CheckStmt = "check" Expression [ "," StringLit ] .`
    fn check_stmt() -> Node {
        todo!();
    }

    /// `ProcessStmt = ProcStmn | TaskStmn | ParStmn .`
    fn process_stmt() -> Node {
        todo!();
    }

    /// `ProcStmt = "proc" ExpressionList .`
    fn proc_stmt() -> Node {
        todo!();
    }

    /// `TaskStmt = "task" ExpressionList .`
    fn task_stmt() -> Node {
        todo!();
    }

    /// `ParStmt = "par" Block .`
    fn par_stmt() -> Node {
        todo!();
    }

    /// `AllocationStmt = AllocStmn | UnallocStmn .`
    fn allocation_stmt() -> Node {
        todo!();
    }

    /// `AllocStmt = "alloc" ExpressionList .`
    fn alloc_stmt() -> Node {
        todo!();
    }

    /// `UnallocStmt = "unalloc" ExpressionList .`
    fn unalloc_stmt() -> Node {
        todo!();
    }
}
