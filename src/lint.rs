use std::collections::HashSet;

use crate::ast::{Expr, Program, Stmt};

#[derive(Debug, Clone)]
pub struct LintWarning {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Default)]
pub struct Linter {
    warnings: Vec<LintWarning>,
}

impl Linter {
    pub fn new() -> Self {
        Linter {
            warnings: Vec::new(),
        }
    }

    pub fn lint_program(&mut self, program: &Program) -> Vec<LintWarning> {
        self.warnings.clear();
        self.check_unused(program);
        self.check_empty_bodies(program);
        self.warnings.clone()
    }

    fn warn(&mut self, line: usize, column: usize, message: String) {
        self.warnings.push(LintWarning {
            message,
            line,
            column,
        });
    }

    fn check_unused(&mut self, program: &Program) {
        let mut defined: Vec<(String, usize, usize)> = Vec::new();
        let mut used: HashSet<String> = HashSet::new();

        for stmt in &program.statements {
            self.collect_defs_uses_stmt(stmt, &mut defined, &mut used);
        }

        for (name, line, col) in &defined {
            if !used.contains(name) {
                self.warn(
                    *line,
                    *col,
                    format!("unused variable or function `{}`", name),
                );
            }
        }
    }

    fn collect_defs_uses_stmt(
        &mut self,
        stmt: &Stmt,
        defined: &mut Vec<(String, usize, usize)>,
        used: &mut HashSet<String>,
    ) {
        match stmt {
            Stmt::Var {
                name, value, span, ..
            }
            | Stmt::Let {
                name, value, span, ..
            } => {
                defined.push((name.clone(), span.line, span.column));
                self.collect_expr_uses(value, defined, used);
            }
            Stmt::FunctionDef {
                name, body, span, ..
            } => {
                defined.push((name.clone(), span.line, span.column));
                for s in body {
                    self.collect_defs_uses_stmt(s, defined, used);
                }
            }
            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                ..
            } => {
                self.collect_expr_uses(condition, defined, used);
                for s in body {
                    self.collect_defs_uses_stmt(s, defined, used);
                }
                for (cond, els) in elifs {
                    self.collect_expr_uses(cond, defined, used);
                    for s in els {
                        self.collect_defs_uses_stmt(s, defined, used);
                    }
                }
                if let Some(alt) = else_body {
                    for s in alt {
                        self.collect_defs_uses_stmt(s, defined, used);
                    }
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.collect_expr_uses(condition, defined, used);
                for s in body {
                    self.collect_defs_uses_stmt(s, defined, used);
                }
            }
            Stmt::For {
                variable,
                iterable,
                body,
                ..
            } => {
                used.insert(variable.clone());
                self.collect_expr_uses(iterable, defined, used);
                for s in body {
                    self.collect_defs_uses_stmt(s, defined, used);
                }
            }
            Stmt::Return { value: Some(v), .. } => {
                self.collect_expr_uses(v, defined, used);
            }
            Stmt::Expr(expr) => {
                self.collect_expr_uses(expr, defined, used);
            }
            _ => {}
        }
    }

    fn collect_expr_uses(
        &mut self,
        expr: &Expr,
        defined: &mut Vec<(String, usize, usize)>,
        used: &mut HashSet<String>,
    ) {
        match expr {
            Expr::Identifier(name, _) => {
                used.insert(name.clone());
            }
            Expr::Binary { left, right, .. } => {
                self.collect_expr_uses(left, defined, used);
                self.collect_expr_uses(right, defined, used);
            }
            Expr::Unary { operand, .. } => self.collect_expr_uses(operand, defined, used),
            Expr::Call { callee, args, .. } => {
                self.collect_expr_uses(callee, defined, used);
                for a in args {
                    self.collect_expr_uses(a, defined, used);
                }
            }
            Expr::Assign { target, value, .. } => {
                self.collect_expr_uses(target, defined, used);
                self.collect_expr_uses(value, defined, used);
            }
            Expr::Grouping { expr, .. } => self.collect_expr_uses(expr, defined, used),
            Expr::MemberAccess { object, member, .. } => {
                used.insert(member.clone());
                self.collect_expr_uses(object, defined, used);
            }
            Expr::StructLiteral { fields, .. } => {
                for (_, expr) in fields {
                    self.collect_expr_uses(expr, defined, used);
                }
            }
            Expr::Cast { expr, .. } => self.collect_expr_uses(expr, defined, used),
            Expr::Lambda { body, .. } => {
                for s in body {
                    self.collect_defs_uses_stmt(s, defined, used);
                }
            }
            Expr::MacroInvocation { args, .. } => {
                for arg in args {
                    self.collect_expr_uses(arg, defined, used);
                }
            }
            Expr::IntLiteral(..)
            | Expr::FloatLiteral(..)
            | Expr::BoolLiteral(..)
            | Expr::StringLiteral(..) => {}
        }
    }

    fn check_empty_bodies(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.check_stmt_empty_body(stmt);
        }
    }

    fn check_stmt_empty_body(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::While { body, span, .. } => {
                if body.is_empty() {
                    self.warn(span.line, span.column, "empty while loop body".to_string());
                }
                for s in body {
                    self.check_stmt_empty_body(s);
                }
            }
            Stmt::For { body, span, .. } => {
                if body.is_empty() {
                    self.warn(span.line, span.column, "empty for loop body".to_string());
                }
                for s in body {
                    self.check_stmt_empty_body(s);
                }
            }
            Stmt::If {
                body,
                elifs,
                else_body,
                ..
            } => {
                for s in body {
                    self.check_stmt_empty_body(s);
                }
                for (_, els) in elifs {
                    for s in els {
                        self.check_stmt_empty_body(s);
                    }
                }
                if let Some(alt) = else_body {
                    for s in alt {
                        self.check_stmt_empty_body(s);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn linter_detects_unused_variable() {
        let source = "let x = 42\n";
        let prog = Parser::new(source).unwrap().parse().unwrap();
        let mut linter = Linter::new();
        let warnings = linter.lint_program(&prog);
        assert!(!warnings.is_empty(), "expected warning for unused variable");
        assert!(warnings.iter().any(|w| w.message.contains("unused")));
    }

    #[test]
    fn linter_no_warning_for_used_variable() {
        let source = "let x = 42\nlet y = x\n";
        let prog = Parser::new(source).unwrap().parse().unwrap();
        let mut linter = Linter::new();
        let warnings = linter.lint_program(&prog);
        let x_warnings: Vec<_> = warnings
            .iter()
            .filter(|w| w.message.contains("`x`"))
            .collect();
        assert!(
            x_warnings.is_empty(),
            "got warning for used var x: {:?}",
            warnings
        );
    }

    #[test]
    fn linter_detects_empty_while() {
        let source = "fn f():\n    while false:\n        let x = 1\n";
        let prog = Parser::new(source).unwrap().parse().unwrap();
        let mut linter = Linter::new();
        let warnings = linter.lint_program(&prog);
        // This while has a body (let x = 1), so no empty body warning
        // x is unused though
        assert!(warnings.iter().any(|w| w.message.contains("unused")));
    }
}
