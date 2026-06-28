use crate::ast::{BinaryOp, Expr, Param, Program, Stmt, Type, UnaryOp};
use crate::lexer::Span;

/// High-level Intermediate Representation (HIR) - a desugared AST.
///
/// HIR removes semantically transparent wrappers (e.g. `Grouping`)
/// and simplifies expression structure while preserving all `Span`
/// information for diagnostics and codegen.
#[derive(Debug, Clone, PartialEq)]
pub struct HirProgram {
    pub statements: Vec<HirStmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirStmt {
    Var {
        name: String,
        type_annot: Option<Type>,
        value: HirExpr,
        span: Span,
    },
    Let {
        name: String,
        type_annot: Option<Type>,
        value: HirExpr,
        span: Span,
    },
    FunctionDef {
        name: String,
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Vec<HirStmt>,
        span: Span,
    },
    StructDef {
        name: String,
        fields: Vec<Param>,
        span: Span,
    },
    InterfaceDef {
        name: String,
        methods: Vec<Param>,
        span: Span,
    },
    If {
        condition: HirExpr,
        body: Vec<HirStmt>,
        elifs: Vec<(HirExpr, Vec<HirStmt>)>,
        else_body: Option<Vec<HirStmt>>,
        span: Span,
    },
    While {
        condition: HirExpr,
        body: Vec<HirStmt>,
        span: Span,
    },
    For {
        variable: String,
        iterable: HirExpr,
        body: Vec<HirStmt>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Return {
        value: Option<HirExpr>,
        span: Span,
    },
    ExternFn {
        name: String,
        params: Vec<Param>,
        return_type: Option<Type>,
        span: Span,
    },
    Load {
        module_path: Vec<String>,
        symbols: Option<Vec<String>>,
        alias: Option<String>,
        is_pub: bool,
        span: Span,
    },
    Defer {
        body: Vec<HirStmt>,
        span: Span,
    },
    MacroDef {
        name: String,
        params: Vec<String>,
        body: Vec<HirStmt>,
        span: Span,
    },
    Expr(HirExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirExpr {
    IntLiteral(i64, Span),
    FloatLiteral(f64, Span),
    StringLiteral(String, Span),
    BoolLiteral(bool, Span),
    Identifier(String, Span),
    Binary {
        left: Box<HirExpr>,
        op: BinaryOp,
        right: Box<HirExpr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        operand: Box<HirExpr>,
        span: Span,
    },
    Call {
        callee: Box<HirExpr>,
        args: Vec<HirExpr>,
        span: Span,
    },
    Assign {
        target: Box<HirExpr>,
        value: Box<HirExpr>,
        span: Span,
    },
    MemberAccess {
        object: Box<HirExpr>,
        member: String,
        span: Span,
    },
    StructLiteral {
        name: String,
        fields: Vec<(String, HirExpr)>,
        span: Span,
    },
    Cast {
        expr: Box<HirExpr>,
        target_type: Type,
        span: Span,
    },
    Lambda {
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Vec<HirStmt>,
        span: Span,
    },
    MacroInvocation {
        name: String,
        args: Vec<HirExpr>,
        span: Span,
    },
}

impl HirExpr {
    pub fn span(&self) -> Span {
        match self {
            HirExpr::IntLiteral(_, s)
            | HirExpr::FloatLiteral(_, s)
            | HirExpr::StringLiteral(_, s)
            | HirExpr::BoolLiteral(_, s)
            | HirExpr::Identifier(_, s) => *s,
            HirExpr::Binary { span, .. }
            | HirExpr::Unary { span, .. }
            | HirExpr::Call { span, .. }
            | HirExpr::Assign { span, .. }
            | HirExpr::MemberAccess { span, .. }
            | HirExpr::StructLiteral { span, .. }
            | HirExpr::Cast { span, .. }
            | HirExpr::Lambda { span, .. }
            | HirExpr::MacroInvocation { span, .. } => *span,
        }
    }
}

/// Lower an AST `Program` into HIR by stripping semantically transparent wrappers.
pub fn lower_program(prog: &Program) -> HirProgram {
    HirProgram {
        statements: prog.statements.iter().map(lower_stmt).collect(),
        span: prog.span,
    }
}

fn lower_stmt(stmt: &Stmt) -> HirStmt {
    match stmt {
        Stmt::Var {
            name,
            type_annot,
            value,
            span,
        } => HirStmt::Var {
            name: name.clone(),
            type_annot: type_annot.clone(),
            value: lower_expr(value),
            span: *span,
        },
        Stmt::Let {
            name,
            type_annot,
            value,
            span,
        } => HirStmt::Let {
            name: name.clone(),
            type_annot: type_annot.clone(),
            value: lower_expr(value),
            span: *span,
        },
        Stmt::FunctionDef {
            name,
            params,
            return_type,
            body,
            span,
        } => HirStmt::FunctionDef {
            name: name.clone(),
            params: params.clone(),
            return_type: return_type.clone(),
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Stmt::StructDef { name, fields, span } => HirStmt::StructDef {
            name: name.clone(),
            fields: fields.clone(),
            span: *span,
        },
        Stmt::InterfaceDef {
            name,
            methods,
            span,
        } => HirStmt::InterfaceDef {
            name: name.clone(),
            methods: methods.clone(),
            span: *span,
        },
        Stmt::If {
            condition,
            body,
            elifs,
            else_body,
            span,
        } => HirStmt::If {
            condition: lower_expr(condition),
            body: body.iter().map(lower_stmt).collect(),
            elifs: elifs
                .iter()
                .map(|(c, b)| (lower_expr(c), b.iter().map(lower_stmt).collect()))
                .collect(),
            else_body: else_body
                .as_ref()
                .map(|b| b.iter().map(lower_stmt).collect()),
            span: *span,
        },
        Stmt::While {
            condition,
            body,
            span,
        } => HirStmt::While {
            condition: lower_expr(condition),
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Stmt::For {
            variable,
            iterable,
            body,
            span,
        } => HirStmt::For {
            variable: variable.clone(),
            iterable: lower_expr(iterable),
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Stmt::Break { span } => HirStmt::Break { span: *span },
        Stmt::Continue { span } => HirStmt::Continue { span: *span },
        Stmt::Return { value, span } => HirStmt::Return {
            value: value.as_ref().map(lower_expr),
            span: *span,
        },
        Stmt::ExternFn {
            name,
            params,
            return_type,
            span,
        } => HirStmt::ExternFn {
            name: name.clone(),
            params: params.clone(),
            return_type: return_type.clone(),
            span: *span,
        },
        Stmt::Load {
            module_path,
            symbols,
            alias,
            is_pub,
            span,
        } => HirStmt::Load {
            module_path: module_path.clone(),
            symbols: symbols.clone(),
            alias: alias.clone(),
            is_pub: *is_pub,
            span: *span,
        },
        Stmt::Defer { body, span } => HirStmt::Defer {
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Stmt::MacroDef {
            name,
            params,
            body,
            span,
        } => HirStmt::MacroDef {
            name: name.clone(),
            params: params.clone(),
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Stmt::Expr(e) => HirStmt::Expr(lower_expr(e)),
    }
}

fn lower_expr(expr: &Expr) -> HirExpr {
    match expr {
        Expr::IntLiteral(v, s) => HirExpr::IntLiteral(*v, *s),
        Expr::FloatLiteral(v, s) => HirExpr::FloatLiteral(*v, *s),
        Expr::StringLiteral(v, s) => HirExpr::StringLiteral(v.clone(), *s),
        Expr::BoolLiteral(v, s) => HirExpr::BoolLiteral(*v, *s),
        Expr::Identifier(v, s) => HirExpr::Identifier(v.clone(), *s),
        Expr::Binary {
            left,
            op,
            right,
            span,
        } => HirExpr::Binary {
            left: Box::new(lower_expr(left)),
            op: *op,
            right: Box::new(lower_expr(right)),
            span: *span,
        },
        Expr::Unary { op, operand, span } => HirExpr::Unary {
            op: *op,
            operand: Box::new(lower_expr(operand)),
            span: *span,
        },
        Expr::Call { callee, args, span } => {
            let callee = lower_expr(callee);
            let args = args.iter().map(lower_expr).collect();
            HirExpr::Call {
                callee: Box::new(callee),
                args,
                span: *span,
            }
        }
        Expr::Assign {
            target,
            value,
            span,
        } => HirExpr::Assign {
            target: Box::new(lower_expr(target)),
            value: Box::new(lower_expr(value)),
            span: *span,
        },
        // Grouping is a transparent wrapper - strip it in HIR.
        Expr::Grouping { expr: inner, .. } => lower_expr(inner),
        Expr::MemberAccess {
            object,
            member,
            span,
        } => HirExpr::MemberAccess {
            object: Box::new(lower_expr(object)),
            member: member.clone(),
            span: *span,
        },
        Expr::StructLiteral { name, fields, span } => HirExpr::StructLiteral {
            name: name.clone(),
            fields: fields
                .iter()
                .map(|(n, e)| (n.clone(), lower_expr(e)))
                .collect(),
            span: *span,
        },
        Expr::Cast {
            expr: inner,
            target_type,
            span,
        } => HirExpr::Cast {
            expr: Box::new(lower_expr(inner)),
            target_type: target_type.clone(),
            span: *span,
        },
        Expr::Lambda {
            params,
            return_type,
            body,
            span,
        } => HirExpr::Lambda {
            params: params.clone(),
            return_type: return_type.clone(),
            body: body.iter().map(lower_stmt).collect(),
            span: *span,
        },
        Expr::MacroInvocation { name, args, span } => HirExpr::MacroInvocation {
            name: name.clone(),
            args: args.iter().map(lower_expr).collect(),
            span: *span,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn parse_and_lower(src: &str) -> HirProgram {
        let mut parser = Parser::new(src).expect("parser creation failed");
        let prog = parser.parse().expect("parse failed");
        lower_program(&prog)
    }

    #[test]
    fn lowering_strips_grouping() {
        // Grouping in AST: Expr::Grouping { expr: Expr::IntLiteral(1, _), .. }
        // HIR: direct HirExpr::IntLiteral
        let hir = parse_and_lower("let x = (1)\n");
        match &hir.statements[0] {
            HirStmt::Let { value, .. } => {
                assert!(matches!(value, HirExpr::IntLiteral(1, _)));
            }
            other => panic!("expected Let, got {:?}", other),
        }
    }

    #[test]
    fn lowering_preserves_binary() {
        let hir = parse_and_lower("let x = 1 + 2\n");
        match &hir.statements[0] {
            HirStmt::Let { value, .. } => {
                assert!(matches!(
                    value,
                    HirExpr::Binary {
                        op: BinaryOp::Add,
                        ..
                    }
                ));
            }
            other => panic!("expected Let, got {:?}", other),
        }
    }

    #[test]
    fn lowering_preserves_function_def() {
        let hir = parse_and_lower("fn f(x: Int) -> Int:\n    return x\n");
        match &hir.statements[0] {
            HirStmt::FunctionDef { name, params, .. } => {
                assert_eq!(name, "f");
                assert_eq!(params.len(), 1);
            }
            other => panic!("expected FunctionDef, got {:?}", other),
        }
    }

    #[test]
    fn lowering_handles_call_and_member_access() {
        let hir = parse_and_lower("let r = a.b(c)\n");
        match &hir.statements[0] {
            HirStmt::Let { value, .. } => match value {
                HirExpr::Call { callee, args, .. } => {
                    match callee.as_ref() {
                        HirExpr::MemberAccess { member, .. } => {
                            assert_eq!(member, "b");
                        }
                        other => panic!("expected MemberAccess, got {:?}", other),
                    }
                    assert_eq!(args.len(), 1);
                }
                other => panic!("expected Call, got {:?}", other),
            },
            other => panic!("expected Let, got {:?}", other),
        }
    }

    #[test]
    fn lower_empty_program() {
        let hir = parse_and_lower("");
        assert!(hir.statements.is_empty());
    }
}
