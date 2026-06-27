// chisel - Pretty-printer over the Nimble AST
//
// Strict 4‑space indentation, one statement per line, block bodies indented.
// Original whitespace is completely ignored.

use crate::ast::*;
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
                out.push_str(": ");
                format_type(ty, out);
            }
            let _ = write!(out, " = ");
            format_expr(value, out);
        }
        Stmt::Let {
            name,
            type_annot,
            value,
            ..
        } => {
            let _ = write!(out, "{}let {}", indent_str, name);
            if let Some(ty) = type_annot {
                out.push_str(": ");
                format_type(ty, out);
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
                let _ = write!(out, "{}: ", p.name);
                format_type(&p.type_annot, out);
            }
            out.push(')');
            if let Some(ret) = return_type {
                out.push_str(" -> ");
                format_type(ret, out);
            }
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::StructDef { name, fields, .. } => {
            let _ = writeln!(out, "{}struct {}:", indent_str, name);
            for f in fields {
                let _ = write!(out, "{}    let {}: ", indent_str, f.name);
                format_type(&f.type_annot, out);
                out.push('\n');
            }
        }
        Stmt::InterfaceDef { name, methods, .. } => {
            let _ = writeln!(out, "{}interface {}:", indent_str, name);
            for m in methods {
                let _ = writeln!(out, "{}    fn {}(...)", indent_str, m.name);
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
                let _ = writeln!(out, "{}else:", indent_str);
                for b in ebody {
                    format_stmt(b, indent + 1, out);
                    out.push('\n');
                }
            }
        }
        Stmt::While {
            condition, body, ..
        } => {
            let _ = write!(out, "{}while ", indent_str);
            format_expr(condition, out);
            out.push_str(":\n");
            for b in body {
                format_stmt(b, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::Break { .. } => {
            let _ = write!(out, "{}break", indent_str);
        }
        Stmt::Continue { .. } => {
            let _ = write!(out, "{}continue", indent_str);
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
                let _ = write!(out, "{}: ", p.name);
                format_type(&p.type_annot, out);
            }
            out.push(')');
            if let Some(ret) = return_type {
                out.push_str(" -> ");
                format_type(ret, out);
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
        Stmt::Defer { body, .. } => {
            let _ = writeln!(out, "{}defer:", indent_str);
            for s in body {
                format_stmt(s, indent + 1, out);
                out.push('\n');
            }
        }
        Stmt::MacroDef {
            name, params, body, ..
        } => {
            let _ = write!(out, "{}macro {}(", indent_str, name);
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(p);
            }
            out.push_str("):\n");
            for s in body {
                format_stmt(s, indent + 1, out);
                out.push('\n');
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
        Expr::Binary {
            left, op, right, ..
        } => {
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
        Expr::StructLiteral { name, fields, .. } => {
            let _ = write!(out, "{}{{", name);
            for (i, (fname, fexpr)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: ", fname);
                format_expr(fexpr, out);
            }
            out.push('}');
        }
        Expr::Cast {
            expr, target_type, ..
        } => {
            format_expr(expr, out);
            out.push_str(" as ");
            format_type(target_type, out);
        }
        Expr::Lambda {
            params,
            return_type,
            body,
            ..
        } => {
            let _ = write!(out, "fn(");
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: ", p.name);
                format_type(&p.type_annot, out);
            }
            out.push(')');
            if let Some(ret) = return_type {
                out.push_str(" -> ");
                format_type(ret, out);
            }
            out.push_str(":\n");
            for s in body {
                format_stmt(s, 1, out);
                out.push('\n');
            }
        }
        Expr::MacroInvocation { name, args, .. } => {
            let _ = write!(out, "{}!(", name);
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_expr(arg, out);
            }
            out.push(')');
        }
    }
}

fn format_type(ty: &Type, out: &mut String) {
    out.push_str(&ty.name);
    if !ty.args.is_empty() {
        out.push('[');
        for (i, arg) in ty.args.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            format_type(arg, out);
        }
        out.push(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

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
