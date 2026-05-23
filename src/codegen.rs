use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::env::Environment;
use crate::lexer::Span;
use crate::types::Type;

// Codegen – LLVM IR textual emitter

/// Translates a type-checked Nimble AST into textual LLVM IR (`.ll`).
///
/// This generator emits human-readable LLVM IR that can be compiled with
/// standard tools like `clang -c`.
pub struct Codegen {
    ir: String,
    indent: u32,
    label_counter: u32,
    reg_counter: u32,
    symbols: HashMap<String, String>,
    symbol_types: HashMap<String, String>,
    register_types: HashMap<String, String>,
    string_globals: Vec<String>,
    string_global_names: HashMap<String, String>,
    current_fn: Option<String>,
    declared_externs: std::collections::HashSet<String>,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            ir: String::new(),
            indent: 0,
            label_counter: 0,
            reg_counter: 0,
            symbols: HashMap::new(),
            symbol_types: HashMap::new(),
            register_types: HashMap::new(),
            string_globals: Vec::new(),
            string_global_names: HashMap::new(),
            current_fn: None,
            declared_externs: std::collections::HashSet::new(),
        }
    }

    /// Consume the generator and return the generated LLVM IR as a string.
    pub fn into_ir(self) -> String {
        self.ir
    }

    // ── Registration helpers ─────────────────────────────────────────────

    fn fresh_reg(&mut self) -> String {
        let r = self.reg_counter;
        self.reg_counter += 1;
        format!("%{}", r)
    }

    fn fresh_label(&mut self) -> String {
        let l = self.label_counter;
        self.label_counter += 1;
        format!(".L{}", l)
    }

    fn indent_str(&self) -> String {
        "  ".repeat(self.indent as usize)
    }

    fn push(&mut self, s: &str) {
        self.ir.push_str(s);
    }

    fn push_indent(&mut self, s: &str) {
        self.push(&self.indent_str());
        self.push(s);
        self.push("\n");
    }

    fn push_blank(&mut self) {
        self.push("\n");
    }

    /// Returns `true` if the last emitted instruction is a terminator
    /// (`ret` or `br`).  Used to avoid emitting dead code.
    fn last_is_terminator(&self) -> bool {
        for line in self.ir.lines().rev() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.ends_with(':') {
                continue;
            }
            return trimmed.starts_with("ret ") || trimmed.starts_with("br ");
        }
        false
    }

    // ── Type mapping ─────────────────────────────────────────────────────

    /// Map a Nimble type to its LLVM IR type string.
    fn llvm_type(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "i64".to_string(),
            Type::Float => "double".to_string(),
            Type::Bool => "i1".to_string(),
            Type::String => "ptr".to_string(), // i8* pointer
            Type::Void => "void".to_string(),
            Type::Function(params, ret) => {
                let ret_ty = self.llvm_type(ret);
                let param_tys: Vec<String> =
                    params.iter().map(|p| self.llvm_type(p)).collect();
                if param_tys.is_empty() {
                    format!("{} ()", ret_ty)
                } else {
                    format!("{} ({})", ret_ty, param_tys.join(", "))
                }
            }
            // Fallback – treat user-defined types as opaque pointers.
            _ => "ptr".to_string(),
        }
    }

    // ── Initializer constant ─────────────────────────────────────────────

    fn llvm_zero(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "0".to_string(),
            Type::Float => "0.0".to_string(),
            Type::Bool => "false".to_string(),
            Type::String => "null".to_string(),
            _ => "zeroinitializer".to_string(),
        }
    }

    // ── Public entry point ───────────────────────────────────────────────

    /// Generate LLVM IR for the entire program, optionally with extern fn
    /// statements collected from loaded modules.
    ///
    /// Returns the IR text on success, or a human-readable error string.
    pub fn generate(
        &mut self,
        program: &Program,
        env: &Environment,
    ) -> Result<&str, String> {
        let empty_externs = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        self.generate_with_externs(program, env, &empty_externs)
    }

    /// Same as [`generate_with_externs`], but also emits function bodies
    /// from loaded modules (non-extern Nimble functions).
    pub fn generate_with_externs_and_module_fns(
        &mut self,
        program: &Program,
        env: &Environment,
        module_externs: &std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
        module_fns: &std::rc::Rc<std::cell::RefCell<Vec<(Stmt, Environment)>>>,
    ) -> Result<&str, String> {
        let externs = module_externs.borrow();
        let fns = module_fns.borrow();
        self.generate_with_externs_internal(program, env, &externs, &fns)
    }

    /// Same as [`generate`], but also emits `declare` for extern fn
    /// statements collected from loaded modules.
    pub fn generate_with_externs(
        &mut self,
        program: &Program,
        env: &Environment,
        module_externs: &std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
    ) -> Result<&str, String> {
        let externs = module_externs.borrow();
        self.generate_with_externs_internal(program, env, &externs, &[])
    }

    fn generate_with_externs_internal(
        &mut self,
        program: &Program,
        env: &Environment,
        module_externs: &[Stmt],
        module_fns: &[(Stmt, Environment)],
    ) -> Result<&str, String> {
        // Module header
        self.push("; ModuleID = 'nimble_program'\n");
        self.push("source_filename = \"nimble_program.nimble\"\n");
        self.push("target datalayout = \"e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        self.push("target triple = \"x86_64-pc-windows-msvc\"\n");
        self.push_blank();

        // Built-in extern declarations from the ember runtime.
        self.push("declare void @nimble_print(ptr)\n");
        self.push("declare void @nimble_print_str(ptr)\n");
        self.push("declare void @nimble_print_i64(i64)\n");
        self.push("declare void @nimble_print_f64(double)\n");

        // Emit extern fn declarations from loaded modules.
        for ext in module_externs {
            // Re-generate the stmt, this time for codegen, ensuring it has the correct name.
            self.gen_stmt(ext, env)?;
        }

        // Pre-pass: collect all string literals from the AST so their globals
        // are emitted at module level (before any function body).
        self.collect_string_literals(program);
        for (fn_stmt, _) in module_fns {
            self.collect_stmt_strings(fn_stmt);
        }
        let globals: Vec<String> = self.string_globals.drain(..).collect();
        for g in &globals {
            self.push(g);
            self.push("\n");
        }
        self.push_blank();

        // Emit function bodies from loaded modules (using each module's own env).
        for (fn_stmt, fn_env) in module_fns {
            self.gen_stmt(fn_stmt, fn_env)?;
        }

        // Generate code for each top‑level statement.
        for stmt in &program.statements {
            self.gen_stmt(stmt, env)?;
        }

        Ok("ok")
    }

    // ── String literal pre-pass ──────────────────────────────────────────

    /// Walk the AST and collect all string literal expressions, emitting
    /// their global constant definitions into `string_globals`.
    fn collect_string_literals(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.collect_stmt_strings(stmt);
        }
    }

    fn collect_stmt_strings(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var { value, .. } | Stmt::Let { value, .. } => {
                self.collect_expr_strings(value);
            }
            Stmt::FunctionDef { body, .. } => {
                for s in body {
                    self.collect_stmt_strings(s);
                }
            }
            Stmt::If { condition, body, elifs, else_body, .. } => {
                self.collect_expr_strings(condition);
                for s in body {
                    self.collect_stmt_strings(s);
                }
                for (cond, els) in elifs {
                    self.collect_expr_strings(cond);
                    for s in els {
                        self.collect_stmt_strings(s);
                    }
                }
                if let Some(alt) = else_body {
                    for s in alt {
                        self.collect_stmt_strings(s);
                    }
                }
            }
            Stmt::While { condition, body, .. } => {
                self.collect_expr_strings(condition);
                for s in body {
                    self.collect_stmt_strings(s);
                }
            }
            Stmt::For { iterable, body, .. } => {
                self.collect_expr_strings(iterable);
                for s in body {
                    self.collect_stmt_strings(s);
                }
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.collect_expr_strings(v);
                }
            }
            Stmt::Expr(expr) => self.collect_expr_strings(expr),
            Stmt::ExternFn { .. } => {}
            Stmt::Load { .. } => {}
        }
    }

    fn collect_expr_strings(&mut self, expr: &Expr) {
        match expr {
            Expr::StringLiteral(s, _) => {
                // Only emit once per unique string content.
                if !self.string_global_names.contains_key(s) {
                    let global_name = format!(".str.{}", self.reg_counter);
                    self.reg_counter += 1;
                    let escaped = s
                        .replace("\\", "\\5C")
                        .replace("\"", "\\22")
                        .replace("\n", "\\0A")
                        .replace("\r", "\\0D")
                        .replace("\t", "\\09");
                    self.string_globals.push(format!(
                        "@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"",
                        global_name,
                        s.len() + 1,
                        escaped
                    ));
                    self.string_global_names
                        .insert(s.clone(), global_name);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.collect_expr_strings(left);
                self.collect_expr_strings(right);
            }
            Expr::Unary { operand, .. } => self.collect_expr_strings(operand),
            Expr::Call { callee, args, .. } => {
                self.collect_expr_strings(callee);
                for a in args {
                    self.collect_expr_strings(a);
                }
            }
            Expr::Assign { target, value, .. } => {
                self.collect_expr_strings(target);
                self.collect_expr_strings(value);
            }
            Expr::Grouping { expr: inner, .. } => self.collect_expr_strings(inner),
            Expr::MemberAccess { object, .. } => self.collect_expr_strings(object),
            Expr::Cast { expr, .. } => self.collect_expr_strings(expr),
            Expr::IntLiteral(..) | Expr::FloatLiteral(..) | Expr::BoolLiteral(..) | Expr::Identifier(..) => {}
        }
    }

    // ── Statement generation ─────────────────────────────────────────────

    fn gen_stmt(&mut self, stmt: &Stmt, env: &Environment) -> Result<(), String> {
        match stmt {
            Stmt::ExternFn { name, params, return_type, .. } => {
                // Skip extern declarations for built-in functions - they are
                // declared by the codegen itself (mapped to ember runtime).
                if matches!(name.as_str(), "print" | "print_int" | "print_str" | "print_float") {
                    return Ok(());
                }
                
                if self.declared_externs.contains(name) {
                    return Ok(());
                }
                
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| Type::from(&p.type_annot))
                    .collect();
                let ret = match return_type {
                    Some(t) => Type::from(t),
                    None => Type::Void,
                };
                let ret_llvm = self.llvm_type(&ret);
                let param_llvm: Vec<String> =
                    param_types.iter().map(|p| self.llvm_type(p)).collect();
                
                let param_str = if param_llvm.is_empty() {
                    String::new()
                } else {
                    param_llvm.join(", ")
                };
                self.push_indent(&format!(
                    "declare {} @{}({})",
                    ret_llvm, name, param_str
                ));
                self.declared_externs.insert(name.clone());
                Ok(())
            }

            Stmt::Var { name, value, span: _, .. }
            | Stmt::Let { name, value, span: _, .. } => {
                let value_reg = self.gen_expr(value, env)?;
                let llvm_ty = self.resolve_ir_type(&value_reg);

                let ptr_reg = self.fresh_reg();
                self.push_indent(&format!(
                    "{} = alloca {}, align 8",
                    ptr_reg, llvm_ty
                ));
                self.push_indent(&format!(
                    "store {} {}, ptr {}, align 8",
                    llvm_ty, value_reg, ptr_reg
                ));

                self.symbols.insert(name.clone(), ptr_reg.clone());
                self.symbol_types.insert(ptr_reg, llvm_ty);
                Ok(())
            }

            Stmt::FunctionDef {
                name,
                params,
                return_type: _,
                body,
                ..
            } => {
                let sym = env.lookup(name).ok_or_else(|| {
                    format!("internal: function `{}` not found in env", name)
                })?;
                let ft = match &sym.type_ {
                    Type::Function(p, r) => (p.clone(), *r.clone()),
                    other => {
                        return Err(format!(
                            "internal: expected function type for `{}`, got {}",
                            name, other
                        ));
                    }
                };
                // Reset per-function state (unnamed values and unnamed labels
                // are numbered from 0 within each function in LLVM IR).
                self.reg_counter = 0;
                self.label_counter = 0;
                self.register_types.clear();

                let ret_llvm = self.llvm_type(&ft.1);
                let param_llvm: Vec<String> =
                    ft.0.iter().map(|p| self.llvm_type(p)).collect();

                // Define the function with named parameters.
                let named_params: Vec<String> = params
                    .iter()
                    .zip(param_llvm.iter())
                    .map(|(p, ty)| format!("{} %{}", ty, p.name))
                    .collect();
                let param_str = if named_params.is_empty() {
                    String::new()
                } else {
                    named_params.join(", ")
                };
                self.push_indent(&format!(
                    "define {} @{}({}) {{",
                    ret_llvm, name, param_str
                ));
                self.indent += 1;

                self.current_fn = Some(name.clone());

                // Create allocas for all parameters at the entry block.
                let entry_label = self.fresh_label();
                self.push_indent(&format!("{}:", entry_label));

                // Alloca + store each parameter.
                for (param, param_ty) in params.iter().zip(param_llvm.iter()) {
                    let ptr_reg = self.fresh_reg();
                    self.push_indent(&format!(
                        "{} = alloca {}, align 8",
                        ptr_reg, param_ty
                    ));
                    self.push_indent(&format!(
                        "store {} %{}, ptr {}, align 8",
                        param_ty, param.name, ptr_reg
                    ));
                    self.symbols.insert(
                        param.name.clone(),
                        ptr_reg.clone(),
                    );
                    self.symbol_types
                        .insert(ptr_reg, param_ty.clone());
                }

                // Generate body statements.
                for s in body {
                    self.gen_stmt(s, env)?;
                }

                // If the function doesn't end with a return, emit a default.
                // For non-void functions this is a poison value.
                {
                    let term = self.ir.lines().last().map(|l| l.trim()).unwrap_or("");
                    if !term.starts_with("ret ") {
                        if ret_llvm == "void" {
                            self.push_indent("ret void");
                        } else {
                            self.push_indent(&format!(
                                "ret {} {}",
                                ret_llvm, self.llvm_zero(&ft.1)
                            ));
                        }
                    }
                }

                self.indent -= 1;
                self.push_indent("}");
                self.push_blank();

                Ok(())
            }

            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                ..
            } => {
                // condition → i1
                let cond_reg = self.gen_expr(condition, env)?;

                let then_label = self.fresh_label();
                let else_label = self.fresh_label();
                let merge_label = self.fresh_label();

                // br i1 %cond, label %then, label %else
                self.push_indent(&format!(
                    "br i1 {}, label %{}, label %{}",
                    cond_reg, then_label, else_label
                ));

                // then block
                self.push_indent(&format!("{}:", then_label));
                for s in body {
                    self.gen_stmt(s, env)?;
                }
                if !self.last_is_terminator() {
                    self.push_indent(&format!("br label %{}", merge_label));
                }

                // else block – may contain elif / else chains
                self.push_indent(&format!("{}:", else_label));

                // Handle elifs by recursive if-else chaining
                let mut current_elifs = elifs.as_slice();
                let mut current_else = else_body.as_deref();

                // We need to handle the elif/else chain.  The simplest way
                // is to emit the elifs as nested if/else IR.
                self.gen_elif_chain(
                    &mut current_elifs,
                    &mut current_else,
                    merge_label.clone(),
                    env,
                )?;

                // merge block
                self.push_indent(&format!("{}:", merge_label));

                Ok(())
            }

            Stmt::While { condition, body, .. } => {
                let header_label = self.fresh_label();
                let body_label = self.fresh_label();
                let exit_label = self.fresh_label();

                self.push_indent(&format!("br label %{}", header_label));
                self.push_indent(&format!("{}:", header_label));

                let cond_reg = self.gen_expr(condition, env)?;
                self.push_indent(&format!(
                    "br i1 {}, label %{}, label %{}",
                    cond_reg, body_label, exit_label
                ));

                self.push_indent(&format!("{}:", body_label));
                for s in body {
                    self.gen_stmt(s, env)?;
                }
                if !self.last_is_terminator() {
                    self.push_indent(&format!("br label %{}", header_label));
                }

                self.push_indent(&format!("{}:", exit_label));

                Ok(())
            }

            Stmt::For {
                variable,
                iterable,
                body,
                ..
            } => {
                // For now, iterate a simple numeric range (not a general
                // iterator).  The loop variable has the type of the
                // iterable expression.
                let iter_type = env
                    .lookup(variable)
                    .map(|s| s.type_.clone())
                    .unwrap_or(Type::Int);

                let llvm_ty = self.llvm_type(&iter_type);

                let start_reg = self.gen_expr(iterable, env)?;

                let ptr_reg = self.fresh_reg();
                self.push_indent(&format!(
                    "{} = alloca {}, align 8",
                    ptr_reg, llvm_ty
                ));
                self.push_indent(&format!(
                    "store {} {}, ptr {}, align 8",
                    llvm_ty, start_reg, ptr_reg
                ));
                self.symbols.insert(variable.clone(), ptr_reg.clone());
                self.symbol_types.insert(ptr_reg, llvm_ty);

                // Body is generated once (the user can mutate the loop var).
                for s in body {
                    self.gen_stmt(s, env)?;
                }

                Ok(())
            }

            Stmt::Return { value, .. } => {
                match value {
                    Some(val) => {
                        let reg = self.gen_expr(val, env)?;
                        self.push_indent(&format!("ret {} {}", self.type_of(&reg), reg));
                    }
                    None => {
                        self.push_indent("ret void");
                    }
                }
                Ok(())
            }

            Stmt::Load { .. } => {
                // Already processed by the typechecker; no codegen needed.
                Ok(())
            }

            Stmt::Expr(expr) => {
                self.gen_expr(expr, env)?;
                Ok(())
            }
        }
    }

    // ── Elif / else chain helper ─────────────────────────────────────────

    fn gen_elif_chain(
        &mut self,
        elifs: &mut &[(Expr, Vec<Stmt>)],
        else_body: &mut Option<&[Stmt]>,
        merge_label: String,
        env: &Environment,
    ) -> Result<(), String> {
        if elifs.is_empty() {
            // Emit the else body (if any).
            if let Some(ebody) = else_body {
                for s in *ebody {
                    self.gen_stmt(s, env)?;
                }
                if !self.last_is_terminator() {
                    self.push_indent(&format!("br label %{}", merge_label));
                }
            } else {
                // No else body - branch straight to merge to keep the block
                // terminated.
                self.push_indent(&format!("br label %{}", merge_label));
            }
            return Ok(());
        }

        let (elif_cond, elif_body) = &elifs[0];
        let cond_reg = self.gen_expr(elif_cond, env)?;

        let then_label = self.fresh_label();
        let next_label = self.fresh_label();

        self.push_indent(&format!(
            "br i1 {}, label %{}, label %{}",
            cond_reg, then_label, next_label
        ));

        self.push_indent(&format!("{}:", then_label));
        for s in elif_body {
            self.gen_stmt(s, env)?;
        }
        if !self.last_is_terminator() {
            self.push_indent(&format!("br label %{}", merge_label));
        }

        self.push_indent(&format!("{}:", next_label));

        // Recurse for remaining elifs.
        *elifs = &elifs[1..];
        self.gen_elif_chain(elifs, else_body, merge_label, env)
    }

    // ── Expression generation ────────────────────────────────────────────

    fn gen_expr(
        &mut self,
        expr: &Expr,
        env: &Environment,
    ) -> Result<String, String> {
        match expr {
            Expr::IntLiteral(n, _) => {
                let reg = self.fresh_reg();
                self.push_indent(&format!(
                    "{} = add i64 0, {}",
                    reg, n
                ));
                self.register_types.insert(reg.clone(), "i64".to_string());
                Ok(reg)
            }

            Expr::FloatLiteral(f, _) => {
                let reg = self.fresh_reg();
                self.push_indent(&format!(
                    "{} = fadd double 0.0, {:.20}",
                    reg, f
                ));
                self.register_types.insert(reg.clone(), "double".to_string());
                Ok(reg)
            }

            Expr::BoolLiteral(b, _) => {
                let reg = self.fresh_reg();
                let val = if *b { "true" } else { "false" };
                self.push_indent(&format!(
                    "{} = add i1 0, {}",
                    reg, val
                ));
                self.register_types.insert(reg.clone(), "i1".to_string());
                Ok(reg)
            }

            Expr::StringLiteral(s, _) => {
                // Look up pre-assigned global name from the pre-pass.
                let global_name = self.string_global_names.get(s).expect("string literal not found in pre-pass; this is a bug").clone();

                let reg = self.fresh_reg();
                self.push_indent(&format!(
                    "{} = getelementptr inbounds [{} x i8], ptr @{}, i64 0, i64 0",
                    reg,
                    s.len() + 1,
                    global_name
                ));
                self.register_types.insert(reg.clone(), "ptr".to_string());
                Ok(reg)
            }

            Expr::Identifier(name, _) => {
                let ptr_reg = self.symbols.get(name).cloned();
                match ptr_reg {
                    Some(ptr) => {
                        let ty = self
                            .symbol_types
                            .get(&ptr)
                            .cloned()
                            .unwrap_or_else(|| "i64".to_string());
                        let val_reg = self.fresh_reg();
                        self.push_indent(&format!(
                            "{} = load {}, ptr {}, align 8",
                            val_reg, ty, ptr
                        ));
                        self.register_types.insert(val_reg.clone(), ty.clone());
                        Ok(val_reg)
                    }
                    None => Err(format!("variable `{}` not in codegen scope", name)),
                }
            }

            Expr::Binary {
                left,
                op,
                right,
                span,
            } => self.gen_binary(left, op, right, span, env),

            Expr::Unary { op, operand, span } => self.gen_unary(op, operand, span, env),

            Expr::Call {
                callee,
                args,
                span,
            } => self.gen_call(callee, args, span, env),

            Expr::Assign {
                target,
                value,
                span: _,
            } => {
                // The target must be an identifier (L-value).
                let target_name = match target.as_ref() {
                    Expr::Identifier(n, _) => n.clone(),
                    _ => {
                        return Err("assignment target must be an identifier".to_string());
                    }
                };
                let val_reg = self.gen_expr(value, env)?;
                let ty = self
                    .symbol_types
                    .get(
                        self.symbols
                            .get(&target_name)
                            .ok_or_else(|| {
                                format!("variable `{}` not found", target_name)
                            })?,
                    )
                    .cloned()
                    .unwrap_or_else(|| "i64".to_string());

                let ptr_reg = self.symbols.get(&target_name).ok_or_else(|| {
                    format!("variable `{}` not found", target_name)
                })?;
                self.push_indent(&format!(
                    "store {} {}, ptr {}, align 8",
                    ty, val_reg, ptr_reg
                ));
                Ok(val_reg)
            }

            Expr::Grouping { expr, .. } => self.gen_expr(expr, env),

            Expr::MemberAccess { object, member, .. } => {
                if let Some(qualified_name) = self.flatten_qualified_name(object.as_ref(), member.clone()) {
                    return self.gen_identifier(&qualified_name, env);
                }
                Err("non-identifier member access not yet supported".to_string())
            }

            Expr::Cast { expr, target_type, .. } => {
                let val_reg = self.gen_expr(expr, env)?;
                let from_ty = self.resolve_ir_type(&val_reg);
                let to_ty = self.llvm_type(&crate::types::Type::from(target_type));

                if from_ty == to_ty {
                    return Ok(val_reg);
                }

                let result = self.fresh_reg();
                match (from_ty.as_str(), to_ty.as_str()) {
                    ("i64", "double") => {
                        self.push_indent(&format!("{} = sitofp i64 {} to double", result, val_reg));
                    }
                    ("double", "i64") => {
                        self.push_indent(&format!("{} = fptosi double {} to i64", result, val_reg));
                    }
                    ("i1", "i64") => {
                        self.push_indent(&format!("{} = zext i1 {} to i64", result, val_reg));
                    }
                    ("i64", "i1") => {
                        self.push_indent(&format!("{} = icmp ne i64 {}, 0", result, val_reg));
                    }
                    ("ptr", "i64") => {
                        self.push_indent(&format!("{} = ptrtoint ptr {} to i64", result, val_reg));
                    }
                    ("i64", "ptr") => {
                        self.push_indent(&format!("{} = inttoptr i64 {} to ptr", result, val_reg));
                    }
                    _ => {
                        self.push_indent(&format!(
                            "{} = bitcast {} {} to {}",
                            result, from_ty, val_reg, to_ty
                        ));
                    }
                }
                self.register_types.insert(result.clone(), to_ty);
                Ok(result)
            }
        }
    }

    fn gen_identifier(&mut self, name: &str, _env: &Environment) -> Result<String, String> {
        if let Some(ptr_reg) = self.symbols.get(name).cloned() {
            let ty = self.symbol_types.get(&ptr_reg).cloned().unwrap_or_else(|| "i64".to_string());
            let val_reg = self.fresh_reg();
            self.push_indent(&format!(
                "{} = load {}, ptr {}, align 8",
                val_reg, ty, ptr_reg
            ));
            self.register_types.insert(val_reg.clone(), ty.clone());
            Ok(val_reg)
        } else {
            Err(format!("variable `{}` not in codegen scope", name))
        }
    }

    // ── Binary operations ────────────────────────────────────────────────

    fn gen_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        _span: &Span,
        env: &Environment,
    ) -> Result<String, String> {
        let left_reg = self.gen_expr(left, env)?;
        let right_reg = self.gen_expr(right, env)?;

        // Determine if this is an integer or float operation based on the
        // left operand type (both should be the same after type checking).
        let left_ir_type = self.resolve_ir_type(&left_reg);

        let result = self.fresh_reg();

        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                let (_, int_instr, float_instr) = match op {
                    BinaryOp::Add => ("add", "add", "fadd"),
                    BinaryOp::Sub => ("sub", "sub", "fsub"),
                    BinaryOp::Mul => ("mul", "mul", "fmul"),
                    BinaryOp::Div => ("div", "sdiv", "fdiv"),
                    BinaryOp::Mod => ("mod", "srem", "frem"),
                    _ => unreachable!(),
                };

                let result_type = if left_ir_type == "double" {
                    self.push_indent(&format!(
                        "{} = {} double {}, {}",
                        result, float_instr, left_reg, right_reg
                    ));
                    "double"
                } else {
                    self.push_indent(&format!(
                        "{} = {} i64 {}, {}",
                        result, int_instr, left_reg, right_reg
                    ));
                    "i64"
                };
                self.register_types.insert(result.clone(), result_type.to_string());
                Ok(result)
            }

            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::Greater
            | BinaryOp::LessEqual
            | BinaryOp::GreaterEqual => {
                let left_ir_type = self.resolve_ir_type(&left_reg);
                let is_float = left_ir_type == "double";

                let cmp = match op {
                    BinaryOp::Equal => if is_float { "oeq" } else { "eq" },
                    BinaryOp::NotEqual => if is_float { "one" } else { "ne" },
                    BinaryOp::Less => if is_float { "olt" } else { "slt" },
                    BinaryOp::Greater => if is_float { "ogt" } else { "sgt" },
                    BinaryOp::LessEqual => if is_float { "ole" } else { "sle" },
                    BinaryOp::GreaterEqual => if is_float { "oge" } else { "sge" },
                    _ => unreachable!(),
                };

                let result = self.fresh_reg();
                if is_float {
                    self.push_indent(&format!(
                        "{} = fcmp {} double {}, {}",
                        result, cmp, left_reg, right_reg
                    ));
                } else {
                    self.push_indent(&format!(
                        "{} = icmp {} i64 {}, {}",
                        result, cmp, left_reg, right_reg
                    ));
                }
                self.register_types.insert(result.clone(), "i1".to_string());
                Ok(result)
            }

            BinaryOp::And | BinaryOp::Or => {
                // Boolean AND / OR using select.
                if matches!(op, BinaryOp::And) {
                    // result = select i1 %left, i1 %right, i1 false
                    self.push_indent(&format!(
                        "{} = select i1 {}, i1 {}, i1 false",
                        result, left_reg, right_reg
                    ));
                } else {
                    // result = select i1 %left, i1 true, i1 %right
                    self.push_indent(&format!(
                        "{} = select i1 {}, i1 true, i1 {}",
                        result, left_reg, right_reg
                    ));
                }
                self.register_types.insert(result.clone(), "i1".to_string());
                Ok(result)
            }
        }
    }

    // ── Unary operations ─────────────────────────────────────────────────

    fn gen_unary(
        &mut self,
        op: &UnaryOp,
        operand: &Expr,
        _span: &Span,
        env: &Environment,
    ) -> Result<String, String> {
        let operand_reg = self.gen_expr(operand, env)?;
        let result = self.fresh_reg();

        match op {
            UnaryOp::Negate => {
                let op_type = self.resolve_ir_type(&operand_reg);
                if op_type == "double" {
                    self.push_indent(&format!(
                        "{} = fsub double 0.0, {}",
                        result, operand_reg
                    ));
                    self.register_types.insert(result.clone(), "double".to_string());
                } else {
                    self.push_indent(&format!(
                        "{} = sub i64 0, {}",
                        result, operand_reg
                    ));
                    self.register_types.insert(result.clone(), "i64".to_string());
                }
                Ok(result)
            }
            UnaryOp::Not => {
                self.push_indent(&format!(
                    "{} = xor i1 {}, true",
                    result, operand_reg
                ));
                self.register_types.insert(result.clone(), "i1".to_string());
                Ok(result)
            }
        }
    }

    // ── Function calls ───────────────────────────────────────────────────

    fn flatten_qualified_name(&self, expr: &Expr, tail: String) -> Option<String> {
        match expr {
            Expr::Identifier(name, _) => Some(format!("{}.{}", name, tail)),
            Expr::MemberAccess { object, member, .. } => {
                let new_tail = format!("{}.{}", member, tail);
                self.flatten_qualified_name(object, new_tail)
            }
            _ => None,
        }
    }

    fn gen_call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
        _span: &Span,
        env: &Environment,
    ) -> Result<String, String> {
        // Resolve the callee name.
        let (fn_name, full_name) = match callee {
            Expr::Identifier(name, _) => (name.clone(), name.clone()),
            Expr::MemberAccess { .. } => {
                let qualified_name = self.flatten_qualified_name(callee, String::new())
                    .ok_or_else(|| "calls on complex member access not yet supported".to_string())?;
                
                // Trim trailing dots from the qualified name if it was a chain like "m.sin."
                let clean_name = qualified_name.trim_end_matches('.');
                
                let target_name = clean_name
                    .rsplit('.')
                    .next()
                    .unwrap()
                    .to_string();
                
                (target_name, clean_name.to_string())
            }
            _ => {
                return Err("calls on non-identifier callees not yet supported".to_string());
            }
        };

        let arg_regs: Result<Vec<String>, String> =
            args.iter().map(|a| self.gen_expr(a, env)).collect();
        let arg_regs = arg_regs?;

        // Map built-in Nimble function names to their LLVM runtime symbols.
        let llvm_fn_name = match fn_name.as_str() {
            "print" => "nimble_print".to_string(),
            "print_int" => "nimble_print_i64".to_string(),
            "print_str" => "nimble_print_str".to_string(),
            "print_float" => "nimble_print_f64".to_string(),
            _ => fn_name.clone(),
        };

        // Look up the function type from the environment using the full qualified name.
        let fn_lookup_name = fn_name.clone();
        let (param_types, ret_type) = match env.lookup(&full_name) {
            Some(sym) => match &sym.type_ {
                Type::Function(params, ret) => (params.clone(), *ret.clone()),
                other => {
                    return Err(format!("`{}` is not a function (type: {})", full_name, other));
                }
            },
            None => {
                // If not in env, check if the function exists without the prefix in the global scope.
                if let Some(sym) = env.lookup(&fn_lookup_name) {
                     match &sym.type_ {
                        Type::Function(params, ret) => (params.clone(), *ret.clone()),
                        other => {
                            return Err(format!("`{}` is not a function (type: {})", fn_lookup_name, other));
                        }
                    }
                } else {
                    return Err(format!("function `{}` not found in env", full_name));
                }
            }
        };

        let _arg_types: Vec<String> = param_types.iter().map(|t| self.llvm_type(t)).collect();
        let ret_llvm = self.llvm_type(&ret_type);

        // Build the argument list for the call instruction.
        let mut call_args = Vec::new();
        for (reg, ty) in arg_regs.iter().zip(param_types.iter()) {
            let llvm_ty = self.llvm_type(ty);
            call_args.push(format!("{} {}", llvm_ty, reg));
        }

        let result = self.fresh_reg();

        if ret_llvm == "void" {
            self.push_indent(&format!(
                "call {} @{}({})",
                ret_llvm,
                llvm_fn_name.trim_start_matches('@'),
                call_args.join(", ")
            ));
            // Return a dummy i64 0 for void calls used in expressions.
            let dummy = self.fresh_reg();
            self.push_indent(&format!("{} = add i64 0, 0", dummy));
            self.register_types.insert(dummy.clone(), "i64".to_string());
            Ok(dummy)
        } else {
            self.push_indent(&format!(
                "{} = call {} @{}({})",
                result,
                ret_llvm,
                llvm_fn_name.trim_start_matches('@'),
                call_args.join(", ")
            ));
            self.register_types.insert(result.clone(), ret_llvm.clone());
            Ok(result)
        }
    }

    // ── Type resolution helpers ──────────────────────────────────────────

    /// Determine the LLVM IR type of an SSA register.
    fn resolve_ir_type(&self, reg: &str) -> String {
        if let Some(ty) = self.register_types.get(reg) {
            return ty.clone();
        }
        // Fallback: check the symbol table.
        for (_name, ptr_reg) in &self.symbols {
            if let Some(ty) = self.symbol_types.get(ptr_reg) {
                if !ty.is_empty() {
                    return ty.clone();
                }
            }
        }
        "i64".to_string()
    }

    /// Get the LLVM IR type string for a register.
    fn type_of(&self, reg: &str) -> String {
        if let Some(ty) = self.register_types.get(reg) {
            return ty.clone();
        }
        // Scan the IR for a definition of this register.
        for line in self.ir.lines().rev() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("{} =", reg)) {
                let rhs = trimmed.split('=').nth(1).unwrap_or("").trim();
                if rhs.starts_with("add i64")
                    || rhs.starts_with("sub i64")
                    || rhs.starts_with("mul i64")
                    || rhs.starts_with("sdiv i64")
                    || rhs.starts_with("load i64")
                {
                    return "i64".to_string();
                }
                if rhs.starts_with("fadd double")
                    || rhs.starts_with("fsub double")
                    || rhs.starts_with("fmul double")
                    || rhs.starts_with("fdiv double")
                    || rhs.starts_with("load double")
                {
                    return "double".to_string();
                }
                if rhs.starts_with("load i1") || rhs.starts_with("add i1")
                    || rhs.starts_with("icmp") || rhs.starts_with("fcmp")
                    || rhs.starts_with("select i1") || rhs.starts_with("xor i1")
                {
                    return "i1".to_string();
                }
                if rhs.starts_with("getelementptr")
                    || rhs.starts_with("load ptr")
                {
                    return "ptr".to_string();
                }
                // Fallback: extract type from the instruction.
                let first_word = rhs.split_whitespace().next().unwrap_or("i64");
                // If it starts with a known keyword, return i64 as default.
                if first_word == "call" {
                    if let Some(ret) = rhs.split_whitespace().nth(1) {
                        if ret != "void" {
                            return ret.to_string();
                        }
                    }
                }
                if rhs.starts_with("alloca") {
                    if let Some(_after) = rhs.split_whitespace().nth(1) {
                        return format!("ptr"); // alloca returns ptr
                    }
                }
            }
        }
        "i64".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::typechecker::TypeChecker;

    fn generate_ir(source: &str) -> Result<String, String> {
        let prog = Parser::new(source)
            .map_err(|e| format!("parse error: {}", e))?
            .parse()
            .map_err(|e| format!("parse error: {}", e))?;

        let env = TypeChecker::new(source)
            .check_program(&prog)
            .map_err(|e| format!("type error: {}", e))?;

        let mut cg = Codegen::new();
        cg.generate(&prog, &env)
            .map_err(|e| format!("codegen error: {}", e))?;
        Ok(cg.into_ir())
    }

    // ── Basic structural tests ───────────────────────────────────────────

    #[test]
    fn ir_contains_module_header() {
        let ir = generate_ir("let x = 42\n").unwrap();
        assert!(ir.contains("ModuleID"), "missing ModuleID: {}", ir);
        assert!(ir.contains("target triple"), "missing target triple: {}", ir);
    }

    #[test]
    fn ir_int_literal() {
        let ir = generate_ir("let x = 42\n").unwrap();
        assert!(ir.contains("add i64"), "no i64 add: {}", ir);
        assert!(ir.contains("alloca i64"), "no alloca: {}", ir);
        assert!(ir.contains("store"), "no store: {}", ir);
    }

    #[test]
    fn ir_float_literal() {
        let ir = generate_ir("let x = 3.14\n").unwrap();
        assert!(ir.contains("fadd double"), "no fadd: {}", ir);
    }

    #[test]
    fn ir_bool_literal() {
        let ir = generate_ir("let x = true\n").unwrap();
        assert!(ir.contains("i1"), "no i1: {}", ir);
    }

    #[test]
    fn ir_string_literal() {
        let ir = generate_ir("let x = \"hello\"\n").unwrap();
        assert!(ir.contains("getelementptr"), "no getelementptr: {}", ir);
        assert!(ir.contains("private unnamed_addr constant"), "no global: {}", ir);
    }

    #[test]
    fn ir_add_int() {
        let ir = generate_ir("let x = 5 + 5\n").unwrap();
        assert!(ir.contains("add i64"), "no integer add: {}", ir);
    }

    #[test]
    fn ir_add_float() {
        let ir = generate_ir("let x = 1.0 + 2.0\n").unwrap();
        assert!(ir.contains("fadd double"), "no float add: {}", ir);
    }

    #[test]
    fn ir_comparison() {
        let ir = generate_ir("let x = 1 < 2\n").unwrap();
        assert!(ir.contains("icmp"), "no icmp: {}", ir);
    }

    #[test]
    fn ir_unary_minus() {
        let ir = generate_ir("let x = -5\n").unwrap();
        assert!(ir.contains("sub i64"), "no sub: {}", ir);
    }

    #[test]
    fn ir_unary_not() {
        let ir = generate_ir("let x = !true\n").unwrap();
        assert!(ir.contains("xor i1"), "no xor: {}", ir);
    }

    #[test]
    fn ir_function_no_args() {
        let ir = generate_ir("fn f() -> Int:\n    return 42\n").unwrap();
        assert!(ir.contains("define i64 @f()"), "bad fn def: {}", ir);
        assert!(ir.contains("ret i64"), "no return: {}", ir);
    }

    #[test]
    fn ir_function_call() {
        let ir = generate_ir("fn f() -> Int:\n    return 42\nfn g() -> Int:\n    return f()\n")
            .unwrap();
        assert!(ir.contains("call i64 @f"), "no call: {}", ir);
    }

    #[test]
    fn ir_function_with_params() {
        let ir = generate_ir("fn add(a: Int, b: Int) -> Int:\n    return a + b\n")
            .unwrap();
        assert!(
            ir.contains("define i64 @add(i64 %a, i64 %b)"),
            "bad fn signature: {}",
            ir
        );
        assert!(ir.contains("add i64"), "no add: {}", ir);
    }

    #[test]
    fn ir_var_mutable() {
        let ir = generate_ir("var x = 10\nx = 20\n").unwrap();
        assert!(ir.contains("store"), "no store for assignment: {}", ir);
    }

    // ── Control flow tests ───────────────────────────────────────────────

    #[test]
    fn ir_if_basic() {
        let ir = generate_ir("if true:\n    let x = 1\n").unwrap();
        assert!(ir.contains("br i1"), "no conditional branch: {}", ir);
        assert!(ir.contains("label %"), "no label ref: {}", ir);
    }

    #[test]
    fn ir_if_elif_else() {
        let ir = generate_ir(
            "let c = 5\nif c < 3:\n    let x = 1\nelif c < 7:\n    let x = 2\nelse:\n    let x = 3\n",
        )
        .unwrap();
        assert!(ir.contains("br i1"), "no conditional branch: {}", ir);
        // We should see at least two branch instructions for if/elif.
        let br_count = ir.matches("br i1").count();
        assert!(br_count >= 2, "expected >=2 br i1, found {}", br_count);
    }

    #[test]
    fn ir_while_loop() {
        let ir = generate_ir("var x = 0\nwhile x < 10:\n    x = x + 1\n").unwrap();
        assert!(ir.contains("br i1"), "no conditional branch: {}", ir);
        // Should have a backward branch (loop backedge).
        assert!(
            ir.lines().filter(|l| l.trim().starts_with("br label %")).count() >= 2,
            "expected at least 2 unconditional branches (loop entry + backedge): {}",
            ir
        );
    }

    #[test]
    fn ir_return_value() {
        let ir = generate_ir("fn f() -> Int:\n    return 42\n").unwrap();
        assert!(ir.contains("ret i64"), "bad return: {}", ir);
    }

    #[test]
    fn ir_return_void() {
        let ir = generate_ir("fn f():\n    return\n").unwrap();
        assert!(ir.contains("ret void"), "no void return: {}", ir);
    }

    // ── Error cases ──────────────────────────────────────────────────────

    #[test]
    fn ir_empty_program() {
        let source = "";
        let prog = Parser::new(source).unwrap().parse().unwrap();
        let env = TypeChecker::new(source).check_program(&prog).unwrap();
        let mut cg = Codegen::new();
        assert!(cg.generate(&prog, &env).is_ok());
        let ir = cg.into_ir();
        assert!(ir.contains("ModuleID"), "empty program should still emit header");
    }

    // ── Example file compilation tests ──────────────────────────────────

    #[test]
    fn example_hello_world() {
        let source = "fn main() -> Int:\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("define i64 @main()"), "bad main: {}", ir);
        assert!(ir.contains("ret i64"), "no return: {}", ir);
    }

    #[test]
    fn example_fibonacci() {
        let source = "fn fib(n: Int) -> Int:\n    if n <= 1:\n        return n\n    else:\n        return fib(n - 1) + fib(n - 2)\n\nfn main() -> Int:\n    return fib(10)\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("@fib("), "no fib fn: {}", ir);
        assert!(ir.contains("@main("), "no main fn: {}", ir);
        assert!(ir.contains("call i64 @fib"), "no fib call: {}", ir);
    }

    #[test]
    fn example_fizzbuzz() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn fizzbuzz(n: Int) -> Void:\n    if n % 3 == 0 && n % 5 == 0:\n        print_int(1)\n    elif n % 3 == 0:\n        print_int(2)\n    elif n % 5 == 0:\n        print_int(3)\n    else:\n        print_int(4)\n\nfn main() -> Int:\n    var i = 1\n    while i <= 100:\n        fizzbuzz(i)\n        i = i + 1\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("define void @fizzbuzz"), "no fizzbuzz fn: {}", ir);
        assert!(ir.contains("define i64 @main"), "no main fn: {}", ir);
        assert!(ir.contains("br i1"), "no conditional branch in IR: {}", ir);
        assert!(ir.contains("call void @fizzbuzz"), "no fizzbuzz call: {}", ir);
    }

    #[test]
    fn example_extern_fn() {
        let source = "extern fn printf(fmt: String) -> Int\n\nfn main() -> Int:\n    printf(\"hello\")\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("declare i64 @printf"), "no printf declare: {}", ir);
        assert!(ir.contains("call i64 @printf"), "no printf call: {}", ir);
    }

    #[test]
    fn example_for_loop() {
        let source = "fn main() -> Int:\n    for i in 0:\n        let x = i\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("@main("), "no main fn: {}", ir);
        assert!(ir.contains("alloca"), "no alloca in for: {}", ir);
    }

    #[test]
    fn example_variables() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn main() -> Int:\n    let x = 42\n    var y = 10\n    y = 20\n    y += 5\n    y -= 3\n    y *= 2\n    y /= 4\n    y %= 3\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("define i64 @main"), "no main fn: {}", ir);
        assert!(ir.contains("srem"), "no srem for %=: {}", ir);
    }

    #[test]
    fn example_types() {
        let source = "extern fn print_int(x: Int) -> Void\nextern fn print_str(x: String) -> Void\n\nfn main() -> Int:\n    let a: Int = 42\n    let b: Float = 3.14\n    let c: Bool = true\n    let d: String = \"hello\"\n    var inferred = 99\n    inferred = 100\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("i64"), "no i64: {}", ir);
        assert!(ir.contains("double"), "no double: {}", ir);
        assert!(ir.contains("i1"), "no i1: {}", ir);
    }

    #[test]
    fn example_operators() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn main() -> Int:\n    let a = 10\n    let b = 3\n    let s = a + b\n    let d = a - b\n    let p = a * b\n    let q = a / b\n    let r = a % b\n    let eq = a == b\n    let ne = a != b\n    let lt = a < b\n    let le = a <= b\n    let neg = -a\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("srem"), "no srem: {}", ir);
        assert!(ir.contains("sdiv"), "no sdiv: {}", ir);
        assert!(ir.contains("icmp"), "no icmp: {}", ir);
    }

    #[test]
    fn example_control_flow() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn classify(n: Int) -> Int:\n    if n > 0:\n        return 1\n    elif n < 0:\n        return -1\n    else:\n        return 0\n\nfn countdown(n: Int) -> Void:\n    var i = n\n    while i >= 0:\n        print_int(i)\n        i = i - 1\n\nfn main() -> Int:\n    countdown(5)\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("@classify("), "no classify: {}", ir);
        assert!(ir.contains("@countdown("), "no countdown: {}", ir);
        assert!(ir.contains("br i1"), "no branches: {}", ir);
    }

    #[test]
    fn example_functions() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn double(x: Int) -> Int:\n    return x * 2\n\nfn recursive(n: Int) -> Int:\n    if n == 0:\n        return 1\n    else:\n        return n * recursive(n - 1)\n\nfn main() -> Int:\n    print_int(double(21))\n    print_int(recursive(5))\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("@double("), "no double: {}", ir);
        assert!(ir.contains("@recursive("), "no recursive: {}", ir);
        assert!(ir.contains("call i64 @recursive"), "no recursive call: {}", ir);
    }

    #[test]
    fn example_booleans() {
        let source = "extern fn print_int(x: Int) -> Void\n\nfn test_and(a: Bool, b: Bool) -> Bool:\n    return a && b\n\nfn test_or(a: Bool, b: Bool) -> Bool:\n    return a || b\n\nfn test_not(flag: Bool) -> Bool:\n    return !flag\n\nfn main() -> Int:\n    let t = true\n    let f = false\n\n    let r1 = test_and(t, t)\n    let r2 = test_or(f, f)\n    let r3 = test_not(f)\n\n    if t && !f:\n        print_int(1)\n\n    if f || t:\n        print_int(2)\n\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("@test_and"), "no test_and fn: {}", ir);
        assert!(ir.contains("@test_or"), "no test_or fn: {}", ir);
        assert!(ir.contains("@test_not"), "no test_not fn: {}", ir);
        assert!(ir.contains("@main("), "no main fn: {}", ir);
        assert!(ir.contains("call void @nimble_print_i64"), "no print_int in main: {}", ir);
    }

    #[test]
    fn example_strings() {
        let source = "extern fn printf(fmt: String) -> Int\n\nfn main() -> Int:\n    printf(\"hello\")\n    return 0\n";
        let ir = generate_ir(source).unwrap();
        assert!(ir.contains("private unnamed_addr constant"), "no string constant: {}", ir);
        assert!(ir.contains("getelementptr"), "no gep: {}", ir);
    }

    #[test]
    fn ir_cast() {
        let ir = generate_ir("let x = 42\nlet y = x as Float\nlet z = y as Int\n").unwrap();
        assert!(ir.contains("sitofp i64"), "no sitofp: {}", ir);
        assert!(ir.contains("fptosi double"), "no fptosi: {}", ir);
    }
}
