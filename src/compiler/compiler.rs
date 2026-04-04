use super::bytecode::*;
use super::register::RegisterAllocator;
use crate::parser::ast::*;
use crate::vm::Value;
use std::collections::HashMap;
use std::sync::Arc;

struct LoopContext {
    break_jumps: Vec<usize>,
    continue_jumps: Vec<usize>,
}

pub struct Compiler {
    chunk: FunctionChunk,
    allocator: RegisterAllocator,
    locals: HashMap<String, Reg>,
    classes: HashMap<String, Vec<String>>,
    loop_stack: Vec<LoopContext>,
    module_scope: bool,
}

impl Compiler {
    pub fn new(name: String) -> Self {
        Self::new_with_scope(name, true, HashMap::new())
    }

    fn new_with_scope(
        name: String,
        module_scope: bool,
        classes: HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            chunk: FunctionChunk {
                name,
                instrs: Vec::new(),
                constants: Vec::new(),
                names: Vec::new(),
                num_registers: 0,
                exports: Vec::new(),
            },
            allocator: RegisterAllocator::new(),
            locals: HashMap::new(),
            classes,
            loop_stack: Vec::new(),
            module_scope,
        }
    }

    fn new_child(&self, name: String) -> Self {
        Self::new_with_scope(name, false, self.classes.clone())
    }

    pub fn compile_stmts(&mut self, stmts: &[Stmt]) -> FunctionChunk {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
        if !matches!(self.chunk.instrs.last(), Some(Instr::Return { .. })) {
            let null_reg = self.load_const(Value::Null);
            self.emit(Instr::Return {
                src: Some(null_reg),
            });
        }
        self.chunk.num_registers = self.allocator.max_registers();
        self.chunk.clone()
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Export(inner) => self.compile_export(inner),
            Stmt::Assign { target, value, .. } => {
                let reg = self.compile_expr(value);
                self.store_name(target, reg);
            }
            Stmt::FieldAssign { obj, field, value } => {
                let o_reg = self.compile_expr(obj);
                let v_reg = self.compile_expr(value);
                let name_idx = self.add_name(field.clone());
                self.emit(Instr::StoreField {
                    obj: o_reg,
                    field: name_idx,
                    src: v_reg,
                });
            }
            Stmt::IndexAssign { obj, idx, value } => {
                let o_reg = self.compile_expr(obj);
                let i_reg = self.compile_expr(idx);
                let v_reg = self.compile_expr(value);
                self.emit(Instr::StoreIndex {
                    obj: o_reg,
                    idx: i_reg,
                    src: v_reg,
                });
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr);
            }
            Stmt::Return(expr) => {
                let reg = expr.as_ref().map(|e| self.compile_expr(e));
                self.emit(Instr::Return { src: reg });
            }
            Stmt::If {
                cond,
                then,
                elifs,
                else_,
            } => {
                let cond_reg = self.compile_expr(cond);
                let jump_to_else = self.emit_jump_if_false(cond_reg);
                for s in then {
                    self.compile_stmt(s);
                }
                let mut end_jumps = vec![self.emit_jump()];
                self.patch_jump(jump_to_else);

                for (c, b) in elifs {
                    let c_reg = self.compile_expr(c);
                    let j = self.emit_jump_if_false(c_reg);
                    for s in b {
                        self.compile_stmt(s);
                    }
                    end_jumps.push(self.emit_jump());
                    self.patch_jump(j);
                }

                if let Some(b) = else_ {
                    for s in b {
                        self.compile_stmt(s);
                    }
                }
                for j in end_jumps {
                    self.patch_jump(j);
                }
            }
            Stmt::While { cond, body } => {
                let loop_start = self.chunk.instrs.len() as u32;
                let cond_reg = self.compile_expr(cond);
                let jump_to_end = self.emit_jump_if_false(cond_reg);

                self.loop_stack.push(LoopContext {
                    break_jumps: Vec::new(),
                    continue_jumps: Vec::new(),
                });
                for s in body {
                    self.compile_stmt(s);
                }
                let loop_ctx = self.loop_stack.pop().unwrap();

                let continue_target = loop_start;
                for j in loop_ctx.continue_jumps {
                    self.patch_jump_to(j, continue_target);
                }

                self.emit(Instr::Jump {
                    target: Label(loop_start),
                });
                let end_label = self.chunk.instrs.len() as u32;
                self.patch_jump(jump_to_end);
                for j in loop_ctx.break_jumps {
                    self.patch_jump_to(j, end_label);
                }
            }
            Stmt::For {
                var,
                iter,
                step,
                body,
            } => {
                let iter_reg = self.compile_expr(iter);
                let len_reg = self.allocator.alloc();
                self.emit(Instr::Len {
                    dst: len_reg,
                    src: iter_reg,
                });

                let idx_reg = self.allocator.alloc();
                let zero_reg = self.load_const(Value::Int(0));
                self.emit(Instr::Move {
                    dst: idx_reg,
                    src: zero_reg,
                });

                let step_reg = if let Some(step_expr) = step {
                    self.compile_expr(step_expr)
                } else {
                    self.load_const(Value::Int(1))
                };

                let loop_start = self.chunk.instrs.len() as u32;
                let cond_reg = self.allocator.alloc();
                self.emit(Instr::CmpLt {
                    dst: cond_reg,
                    a: idx_reg,
                    b: len_reg,
                });
                let jump_to_end = self.emit_jump_if_false(cond_reg);

                let item_reg = self.allocator.alloc();
                self.emit(Instr::LoadIndex {
                    dst: item_reg,
                    obj: iter_reg,
                    idx: idx_reg,
                });
                self.assign_local(var, item_reg);

                self.loop_stack.push(LoopContext {
                    break_jumps: Vec::new(),
                    continue_jumps: Vec::new(),
                });
                for s in body {
                    self.compile_stmt(s);
                }
                let loop_ctx = self.loop_stack.pop().unwrap();

                let increment_label = self.chunk.instrs.len() as u32;
                for j in loop_ctx.continue_jumps {
                    self.patch_jump_to(j, increment_label);
                }

                self.emit(Instr::AddInt {
                    dst: idx_reg,
                    a: idx_reg,
                    b: step_reg,
                });
                self.emit(Instr::Jump {
                    target: Label(loop_start),
                });

                let end_label = self.chunk.instrs.len() as u32;
                self.patch_jump(jump_to_end);
                for j in loop_ctx.break_jumps {
                    self.patch_jump_to(j, end_label);
                }
            }
            Stmt::ForKV {
                key,
                val,
                iter,
                body,
            } => {
                let map_reg = self.compile_expr(iter);

                let callee_reg = self.allocator.alloc();
                let name_idx = self.add_name("__map_keys".into());
                self.emit(Instr::LoadGlobal {
                    dst: callee_reg,
                    name: name_idx,
                });

                let keys_reg = self.allocator.alloc();
                self.emit(Instr::Call {
                    dst: Some(keys_reg),
                    callee: callee_reg,
                    args: vec![map_reg],
                });

                let len_reg = self.allocator.alloc();
                self.emit(Instr::Len {
                    dst: len_reg,
                    src: keys_reg,
                });

                let idx_reg = self.allocator.alloc();
                let zero_reg = self.load_const(Value::Int(0));
                self.emit(Instr::Move {
                    dst: idx_reg,
                    src: zero_reg,
                });

                let loop_start = self.chunk.instrs.len() as u32;
                let cond_reg = self.allocator.alloc();
                self.emit(Instr::CmpLt {
                    dst: cond_reg,
                    a: idx_reg,
                    b: len_reg,
                });
                let jump_to_end = self.emit_jump_if_false(cond_reg);

                let key_reg = self.allocator.alloc();
                self.emit(Instr::LoadIndex {
                    dst: key_reg,
                    obj: keys_reg,
                    idx: idx_reg,
                });
                self.assign_local(key, key_reg);

                let val_reg = self.allocator.alloc();
                self.emit(Instr::LoadIndex {
                    dst: val_reg,
                    obj: map_reg,
                    idx: key_reg,
                });
                self.assign_local(val, val_reg);

                self.loop_stack.push(LoopContext {
                    break_jumps: Vec::new(),
                    continue_jumps: Vec::new(),
                });
                for s in body {
                    self.compile_stmt(s);
                }
                let loop_ctx = self.loop_stack.pop().unwrap();

                let increment_label = self.chunk.instrs.len() as u32;
                for j in loop_ctx.continue_jumps {
                    self.patch_jump_to(j, increment_label);
                }

                let one_reg = self.load_const(Value::Int(1));
                self.emit(Instr::AddInt {
                    dst: idx_reg,
                    a: idx_reg,
                    b: one_reg,
                });
                self.emit(Instr::Jump {
                    target: Label(loop_start),
                });

                let end_label = self.chunk.instrs.len() as u32;
                self.patch_jump(jump_to_end);
                for j in loop_ctx.break_jumps {
                    self.patch_jump_to(j, end_label);
                }
            }
            Stmt::Break => {
                let j = self.emit_jump();
                if let Some(loop_ctx) = self.loop_stack.last_mut() {
                    loop_ctx.break_jumps.push(j);
                }
            }
            Stmt::Continue => {
                let j = self.emit_jump();
                if let Some(loop_ctx) = self.loop_stack.last_mut() {
                    loop_ctx.continue_jumps.push(j);
                }
            }
            Stmt::Load { alias, source } => {
                let callee_reg = self.allocator.alloc();
                let name_idx = self.add_name("__load_module".into());
                self.emit(Instr::LoadGlobal {
                    dst: callee_reg,
                    name: name_idx,
                });

                let src_reg = self.load_const(Value::Str(Arc::new(source.clone())));
                let dst = self.allocator.alloc();
                self.emit(Instr::Call {
                    dst: Some(dst),
                    callee: callee_reg,
                    args: vec![src_reg],
                });
                self.store_name(alias, dst);
            }
            Stmt::FnDef(f) => {
                let mut inner_compiler = self.new_child(f.name.clone());
                for p in f.params.iter() {
                    let reg = inner_compiler.allocator.alloc();
                    inner_compiler.locals.insert(p.name.clone(), reg);
                }
                let inner_chunk = inner_compiler.compile_stmts(&f.body);
                let val = Value::Function(Arc::new(inner_chunk));
                let idx = self.add_constant(val);
                let dst = self.allocator.alloc();
                self.emit(Instr::LoadConst { dst, idx });
                self.store_name(&f.name, dst);
            }
            Stmt::ClsDef(c) => {
                let fields: Vec<String> = c.fields.iter().map(|p| p.name.clone()).collect();
                self.classes.insert(c.name.clone(), fields.clone());
                let val = Value::Class {
                    name: Arc::new(c.name.clone()),
                    fields,
                };
                let idx = self.add_constant(val);
                let dst = self.allocator.alloc();
                self.emit(Instr::LoadConst { dst, idx });
                self.store_name(&c.name, dst);
            }
        }
    }

    fn compile_export(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FnDef(f) => {
                self.compile_stmt(stmt);
                self.chunk.exports.push(f.name.clone());
            }
            Stmt::ClsDef(c) => {
                self.compile_stmt(stmt);
                self.chunk.exports.push(c.name.clone());
            }
            Stmt::Assign { target, .. } => {
                self.compile_stmt(stmt);
                self.chunk.exports.push(target.clone());
            }
            _ => self.compile_stmt(stmt),
        }
    }

    fn store_name(&mut self, name: &str, reg: Reg) {
        if self.module_scope {
            let name_idx = self.add_name(name.to_string());
            self.emit(Instr::StoreGlobal {
                name: name_idx,
                src: reg,
            });
        } else {
            self.assign_local(name, reg);
        }
    }

    fn assign_local(&mut self, name: &str, src: Reg) {
        if let Some(&local_reg) = self.locals.get(name) {
            self.emit(Instr::Move {
                dst: local_reg,
                src,
            });
        } else {
            let new_reg = self.allocator.alloc();
            self.emit(Instr::Move { dst: new_reg, src });
            self.locals.insert(name.to_string(), new_reg);
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> Reg {
        match expr {
            Expr::Int(n) => self.load_const(Value::Int(*n)),
            Expr::Float(n) => self.load_const(Value::Float(*n)),
            Expr::Str(s) => self.load_const(Value::Str(Arc::new(s.clone()))),
            Expr::Bool(b) => self.load_const(Value::Bool(*b)),
            Expr::Null => self.load_const(Value::Null),
            Expr::Ident(name) => {
                if let Some(&reg) = self.locals.get(name) {
                    let r = self.allocator.alloc();
                    self.emit(Instr::Move { dst: r, src: reg });
                    r
                } else {
                    let reg = self.allocator.alloc();
                    let name_idx = self.add_name(name.clone());
                    self.emit(Instr::LoadGlobal {
                        dst: reg,
                        name: name_idx,
                    });
                    reg
                }
            }
            Expr::BinOp { op, left, right } => match op {
                BinOp::And | BinOp::Or => self.compile_logical(*op, left, right),
                BinOp::DotDot => {
                    let l = self.compile_expr(left);
                    let r = self.compile_expr(right);
                    let dst = self.allocator.alloc();
                    self.emit(Instr::MakeRange {
                        dst,
                        start: l,
                        end: r,
                    });
                    dst
                }
                _ => {
                    let l = self.compile_expr(left);
                    let r = self.compile_expr(right);
                    let dst = self.allocator.alloc();
                    match op {
                        BinOp::Plus => self.emit(Instr::AddInt { dst, a: l, b: r }),
                        BinOp::Minus => self.emit(Instr::SubInt { dst, a: l, b: r }),
                        BinOp::Star => self.emit(Instr::MulInt { dst, a: l, b: r }),
                        BinOp::Slash => self.emit(Instr::DivInt { dst, a: l, b: r }),
                        BinOp::Percent => self.emit(Instr::Mod { dst, a: l, b: r }),
                        BinOp::EqEq => self.emit(Instr::CmpEq { dst, a: l, b: r }),
                        BinOp::BangEq => self.emit(Instr::CmpNe { dst, a: l, b: r }),
                        BinOp::Lt => self.emit(Instr::CmpLt { dst, a: l, b: r }),
                        BinOp::Gt => self.emit(Instr::CmpGt { dst, a: l, b: r }),
                        BinOp::LtEq => self.emit(Instr::CmpLe { dst, a: l, b: r }),
                        BinOp::GtEq => self.emit(Instr::CmpGe { dst, a: l, b: r }),
                        _ => {}
                    }
                    dst
                }
            },
            Expr::UnaryOp { op, expr } => {
                let reg = self.compile_expr(expr);
                let dst = self.allocator.alloc();
                match op {
                    UnaryOp::Minus => self.emit(Instr::Negate { dst, src: reg }),
                    UnaryOp::Not => self.emit(Instr::Not { dst, src: reg }),
                }
                dst
            }
            Expr::Call { callee, args } => {
                if let Expr::Ident(name) = &**callee {
                    if let Some(fields) = self.classes.get(name).cloned() {
                        return self.compile_struct_construct(name, fields, args);
                    }
                }

                let c_reg = self.compile_expr(callee);
                let mut arg_regs = Vec::new();
                for arg in args {
                    arg_regs.push(self.compile_expr(&arg.expr));
                }
                let dst = self.allocator.alloc();
                self.emit(Instr::Call {
                    dst: Some(dst),
                    callee: c_reg,
                    args: arg_regs,
                });
                dst
            }
            Expr::Index { obj, idx } => {
                let o_reg = self.compile_expr(obj);
                let i_reg = self.compile_expr(idx);
                let dst = self.allocator.alloc();
                self.emit(Instr::LoadIndex {
                    dst,
                    obj: o_reg,
                    idx: i_reg,
                });
                dst
            }
            Expr::Field { obj, field } => {
                let o_reg = self.compile_expr(obj);
                let dst = self.allocator.alloc();
                let name_idx = self.add_name(field.clone());
                self.emit(Instr::LoadField {
                    dst,
                    obj: o_reg,
                    field: name_idx,
                });
                dst
            }
            Expr::List(items) => {
                let mut regs = Vec::new();
                for item in items {
                    regs.push(self.compile_expr(item));
                }
                let dst = self.allocator.alloc();
                self.emit(Instr::MakeList { dst, items: regs });
                dst
            }
            Expr::Map(pairs) => {
                let mut regs = Vec::new();
                for (k, v) in pairs {
                    regs.push((self.compile_expr(k), self.compile_expr(v)));
                }
                let dst = self.allocator.alloc();
                self.emit(Instr::MakeMap { dst, pairs: regs });
                dst
            }
            Expr::Interp(parts) => {
                let mut regs = Vec::new();
                for part in parts {
                    match part {
                        InterpPart::Str(s) => {
                            regs.push(self.load_const(Value::Str(Arc::new(s.clone()))))
                        }
                        InterpPart::Expr(e) => {
                            let r = self.compile_expr(e);
                            let sreg = self.allocator.alloc();
                            self.emit(Instr::Stringify { dst: sreg, src: r });
                            regs.push(sreg);
                        }
                    }
                }
                let dst = self.allocator.alloc();
                self.emit(Instr::Concat { dst, parts: regs });
                dst
            }
            Expr::Ternary { cond, then, else_ } => {
                let cond_reg = self.compile_expr(cond);
                let jump_else = self.emit_jump_if_false(cond_reg);
                let then_reg = self.compile_expr(then);
                let result_reg = self.allocator.alloc();
                self.emit(Instr::Move {
                    dst: result_reg,
                    src: then_reg,
                });
                let jump_end = self.emit_jump();
                self.patch_jump(jump_else);
                let else_reg = self.compile_expr(else_);
                self.emit(Instr::Move {
                    dst: result_reg,
                    src: else_reg,
                });
                self.patch_jump(jump_end);
                result_reg
            }
            Expr::Propagate(expr) => {
                let reg = self.compile_expr(expr);
                self.emit(Instr::Propagate { src: reg });
                reg
            }
            Expr::Lambda {
                params,
                ret_ty: _,
                body,
            } => {
                let mut inner_compiler = self.new_child("<lambda>".into());
                for p in params.iter() {
                    let reg = inner_compiler.allocator.alloc();
                    inner_compiler.locals.insert(p.name.clone(), reg);
                }
                let inner_chunk = inner_compiler.compile_stmts(body);
                let val = Value::Function(Arc::new(inner_chunk));
                let idx = self.add_constant(val);
                let dst = self.allocator.alloc();
                self.emit(Instr::LoadConst { dst, idx });
                dst
            }
            Expr::Spawn(expr) => {
                match &**expr {
                    Expr::Call { callee, args } => {
                        let c_reg = self.compile_expr(callee);
                        let mut arg_regs = Vec::new();
                        for arg in args {
                            arg_regs.push(self.compile_expr(&arg.expr));
                        }
                        self.emit(Instr::Spawn {
                            callee: c_reg,
                            args: arg_regs,
                        });
                    }
                    _ => {
                        let c_reg = self.compile_expr(expr);
                        self.emit(Instr::Spawn {
                            callee: c_reg,
                            args: Vec::new(),
                        });
                    }
                }
                self.load_const(Value::Null)
            }
        }
    }

    fn compile_struct_construct(
        &mut self,
        name: &str,
        fields: Vec<String>,
        args: &[CallArg],
    ) -> Reg {
        let mut named: HashMap<String, Reg> = HashMap::new();
        let mut positional: Vec<Reg> = Vec::new();
        for arg in args {
            let reg = self.compile_expr(&arg.expr);
            if let Some(n) = &arg.name {
                named.insert(n.clone(), reg);
            } else {
                positional.push(reg);
            }
        }

        let mut field_regs = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            let reg = if let Some(r) = named.get(field) {
                *r
            } else if i < positional.len() {
                positional[i]
            } else {
                self.load_const(Value::Null)
            };
            let name_idx = self.add_name(field.clone());
            field_regs.push((name_idx, reg));
        }

        let dst = self.allocator.alloc();
        let class_idx = self.add_name(name.to_string());
        self.emit(Instr::MakeStruct {
            dst,
            class: class_idx,
            fields: field_regs,
        });
        dst
    }

    fn compile_logical(&mut self, op: BinOp, left: &Expr, right: &Expr) -> Reg {
        let left_reg = self.compile_expr(left);
        let result_reg = self.allocator.alloc();
        self.emit(Instr::Move {
            dst: result_reg,
            src: left_reg,
        });

        let jump = match op {
            BinOp::And => self.emit_jump_if_false(left_reg),
            BinOp::Or => self.emit_jump_if_true(left_reg),
            _ => self.emit_jump_if_false(left_reg),
        };

        let right_reg = self.compile_expr(right);
        self.emit(Instr::Move {
            dst: result_reg,
            src: right_reg,
        });
        self.patch_jump(jump);
        result_reg
    }

    fn emit(&mut self, instr: Instr) {
        self.chunk.instrs.push(instr);
    }

    fn load_const(&mut self, val: Value) -> Reg {
        let reg = self.allocator.alloc();
        let idx = self.add_constant(val);
        self.emit(Instr::LoadConst { dst: reg, idx });
        reg
    }

    fn add_constant(&mut self, val: Value) -> ConstIdx {
        self.chunk.constants.push(val);
        ConstIdx((self.chunk.constants.len() - 1) as u16)
    }

    fn add_name(&mut self, name: String) -> NameIdx {
        self.chunk.names.push(name);
        NameIdx((self.chunk.names.len() - 1) as u16)
    }

    fn emit_jump_if_false(&mut self, cond: Reg) -> usize {
        self.emit(Instr::JumpIfFalse {
            cond,
            target: Label(0),
        });
        self.chunk.instrs.len() - 1
    }

    fn emit_jump_if_true(&mut self, cond: Reg) -> usize {
        self.emit(Instr::JumpIfTrue {
            cond,
            target: Label(0),
        });
        self.chunk.instrs.len() - 1
    }

    fn emit_jump(&mut self) -> usize {
        self.emit(Instr::Jump { target: Label(0) });
        self.chunk.instrs.len() - 1
    }

    fn patch_jump(&mut self, index: usize) {
        let target = self.chunk.instrs.len() as u32;
        self.patch_jump_to(index, target);
    }

    fn patch_jump_to(&mut self, index: usize, target: u32) {
        match &mut self.chunk.instrs[index] {
            Instr::JumpIfFalse { target: t, .. } => *t = Label(target),
            Instr::JumpIfTrue { target: t, .. } => *t = Label(target),
            Instr::Jump { target: t } => *t = Label(target),
            _ => panic!("Not a jump instruction"),
        }
    }
}
