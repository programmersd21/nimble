// chisel - Pretty-printer over the Nimble AST
//
// Strict 4‑space indentation, one statement per line, block bodies indented.
// Original whitespace is completely ignored.

use nimble::ast::*;
use std::fmt::Write;

/// Format an entire program into canonical form.
pub fn format_program(prog: &Program) -> String {
    let mut out = String::new();
    for stmt in &prog.statements {
        format_stmt(stmt, 0, &mut out);
        out.push('\n');
    }
    out
}


fn format_stmt(stmt: &Stmt, indent: usize, out: &mut String) {
    let indent_str = "    ".repeat(indent);
    match stmt {
        Stmt::Var {
            name,
            type_annot,
            value,
            ..
        } => {
            let _ = write!(out, "{}var {}", indent_str, name);
            if let Some(ty) = type_annot {
                let _ = write!(out, ": {}", ty.name);
            }
            let _ = write!(out, " = ");
            format_expr(value, out);
            // TODO: add test for type-annotated var
        }
        Stmt::Let {
            name,
            type_annot,
            value,
            ..
        } => {
            let _ = write!(out, "{}let {}", indent_str, name);
            if let Some(ty) = type_annot {
                let _ = write!(out, ": {}", ty.name);
            }
            let _ = write!(out, " = ");
            format_expr(value, out);
        }
        Stmt::FunctionDef {
            name,
            params,
            return_type,
            body,
            ..
        } => {
            let _ = write!(out, "{}fn {}(", indent_str, name);
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: {}", p.name, p.type_annot.name);
            }
            out.push(')');
            if let Some(ret) = return_type {
                let _ = write!(out, " -> {}", ret.name);
            }
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::If {
            condition,
            body,
            elifs,
            else_body,
            ..
        } => {
            let _ = write!(out, "{}if ", indent_str);
            format_expr(condition, out);
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
            for (elif_cond, elif_body) in elifs {
                let _ = write!(out, "{}elif ", indent_str);
                format_expr(elif_cond, out);
                out.push_str(":\n");
                for b in elif_body {
                    format_stmt(b, indent + 1, out);
                    out.push('\n');
                }
            }
            if let Some(ebody) = else_body {
                let _ = write!(out, "{}else:\n", indent_str);
                for b in ebody {
                    format_stmt(b, indent + 1, out);
                    out.push('\n');
                }
            }
        }
        Stmt::While { condition, body, .. } => {
            let _ = write!(out, "{}while ", indent_str);
            format_expr(condition, out);
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::For {
            variable,
            iterable,
            body,
            ..
        } => {
            let _ = write!(out, "{}for {} in ", indent_str, variable);
            format_expr(iterable, out);
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::ExternFn {
            name,
            params,
            return_type,
            ..
        } => {
            let _ = write!(out, "{}extern fn {}(", indent_str, name);
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: {}", p.name, p.type_annot.name);
            }
            out.push(')');
            if let Some(ret) = return_type {
                let _ = write!(out, " -> {}", ret.name);
            }
        }
        Stmt::Return { value, .. } => {
            let _ = write!(out, "{}return", indent_str);
            if let Some(val) = value {
                out.push(' ');
                format_expr(val, out);
            }
        }
        Stmt::Expr(expr) => {
            let _ = write!(out, "{}", indent_str);
            format_expr(expr, out);
        }
        Stmt::Load {
            module_path,
            symbols,
            alias,
            is_pub,
            ..
        } => {
            let _ = write!(out, "{}", indent_str);
            if *is_pub {
                out.push_str("pub ");
            }
            let _ = write!(out, "load {}", module_path.join("."));
            if let Some(a) = alias {
                let _ = write!(out, " as {}", a);
            }
            if let Some(syms) = symbols {
                let _ = write!(out, "::{{{}}}", syms.join(", "));
            }
        }
    }
}


fn format_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::IntLiteral(val, _) => {
            let _ = write!(out, "{}", val);
        }
        Expr::FloatLiteral(val, _) => {
            let _ = write!(out, "{}", val);
        }
        Expr::StringLiteral(val, _) => {
            let escaped = val
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t");
            let _ = write!(out, "\"{}\"", escaped);
        }
        Expr::BoolLiteral(val, _) => {
            let _ = write!(out, "{}", val);
        }
        Expr::Identifier(name, _) => {
            out.push_str(name);
        }
        Expr::Binary { left, op, right, .. } => {
            format_expr(left, out);
            let op_str = match op {
                BinaryOp::Add => " + ",
                BinaryOp::Sub => " - ",
                BinaryOp::Mul => " * ",
                BinaryOp::Div => " / ",
                BinaryOp::Equal => " == ",
                BinaryOp::NotEqual => " != ",
                BinaryOp::Less => " < ",
                BinaryOp::Greater => " > ",
                BinaryOp::LessEqual => " <= ",
                BinaryOp::GreaterEqual => " >= ",
                BinaryOp::And => " && ",
                BinaryOp::Or => " || ",
                BinaryOp::Mod => " % ",
            };
            out.push_str(op_str);
            format_expr(right, out);
        }
        Expr::Unary { op, operand, .. } => {
            let op_str = match op {
                UnaryOp::Negate => "-",
                UnaryOp::Not => "!",
            };
            out.push_str(op_str);
            format_expr(operand, out);
        }
        Expr::Call { callee, args, .. } => {
            format_expr(callee, out);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_expr(arg, out);
            }
            out.push(')');
        }
        Expr::Assign { target, value, .. } => {
            format_expr(target, out);
            out.push_str(" = ");
            format_expr(value, out);
        }
        Expr::Grouping { expr, .. } => {
            out.push('(');
            format_expr(expr, out);
            out.push(')');
        }
        Expr::MemberAccess { object, member, .. } => {
            format_expr(object, out);
            out.push('.');
            out.push_str(member);
        }
        Expr::Cast { expr, target_type, .. } => {
            format_expr(expr, out);
            let _ = write!(out, " as {}", target_type.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nimble::Parser;

    fn fmt(source: &str) -> String {
        let prog = Parser::new(source).unwrap().parse().unwrap();
        format_program(&prog)
    }

    #[test]
    fn format_var_decl() {
        let result = fmt("var x = 42\n");
        assert_eq!(result, "var x = 42\n");
    }

    #[test]
    fn format_let_decl() {
        let result = fmt("let name = \"hello\"\n");
        assert_eq!(result, "let name = \"hello\"\n");
    }

    #[test]
    fn format_function_def() {
        let result = fmt("fn add(a: Int, b: Int) -> Int:\n    return a + b\n");
        assert!(result.contains("fn add(a: Int, b: Int) -> Int:\n"));
        assert!(result.contains("    return a + b"));
    }

    #[test]
    fn format_if_else() {
        let result = fmt("if x > 0:\n    return 1\nelse:\n    return 0\n");
        assert!(result.contains("if x > 0:\n"));
        assert!(result.contains("    return 1\n"));
        assert!(result.contains("else:\n"));
        assert!(result.contains("    return 0"));
    }

    #[test]
    fn format_while_loop() {
        let result = fmt("while i < 10:\n    i = i + 1\n");
        assert!(result.contains("while i < 10:\n"));
        assert!(result.contains("    i = i + 1"));
    }

    #[test]
    fn format_for_loop() {
        let result = fmt("for x in items:\n    print(x)\n");
        assert!(result.contains("for x in items:\n"));
        assert!(result.contains("    print(x)"));
    }

    #[test]
    fn format_binary_ops() {
        let result = fmt("var z = 1 + 2 * 3\n");
        assert_eq!(result, "var z = 1 + 2 * 3\n");
    }

    #[test]
    fn format_call() {
        let result = fmt("print(hello, 42)\n");
        assert_eq!(result, "print(hello, 42)\n");
    }

    #[test]
    fn format_grouping() {
        let result = fmt("var x = (1 + 2) * 3\n");
        assert!(result.contains("(1 + 2) * 3"));
    }

    #[test]
    fn format_string_literal() {
        let result = fmt("var s = \"hello\\nworld\"\n");
        assert_eq!(result, "var s = \"hello\\nworld\"\n");
    }
}
