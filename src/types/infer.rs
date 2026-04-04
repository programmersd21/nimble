use crate::parser::ast::*;
use crate::types::ty::Type;
use std::collections::HashMap;

pub struct TypeEnv {
    vars: HashMap<String, Type>,
    parent: Option<Box<TypeEnv>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn extend(parent: TypeEnv) -> Self {
        Self {
            vars: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Type> {
        self.vars
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }

    pub fn set(&mut self, name: String, ty: Type) {
        self.vars.insert(name, ty);
    }
}

pub struct Inferencer {
    env: TypeEnv,
}

impl Inferencer {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
        }
    }

    pub fn infer_stmts(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.infer_stmt(stmt)?;
        }
        Ok(())
    }

    fn infer_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Assign { target, ty, value } => {
                let inferred = self.infer_expr(value)?;
                if let Some(expected) = ty {
                    if !inferred.is_assignable_to(expected) {
                        return Err(format!(
                            "Type mismatch: cannot assign {:?} to {:?}",
                            inferred, expected
                        ));
                    }
                    self.env.set(target.clone(), expected.clone());
                } else {
                    self.env.set(target.clone(), inferred);
                }
            }
            Stmt::FnDef(f) => {
                let ret_ty = f.ret_ty.clone().unwrap_or(Type::Null);
                let param_tys: Vec<Type> = f
                    .params
                    .iter()
                    .map(|p| p.ty.clone().unwrap_or(Type::Any))
                    .collect();
                self.env
                    .set(f.name.clone(), Type::Fn(param_tys, Box::new(ret_ty)));
                // We'd normally check the body here with a new scope
            }
            Stmt::ClsDef(c) => {
                self.env.set(c.name.clone(), Type::Struct(c.name.clone()));
            }
            Stmt::If {
                cond,
                then: _,
                elifs: _,
                else_: _,
            } => {
                let cond_ty = self.infer_expr(cond)?;
                if cond_ty != Type::Bool {
                    return Err("Condition must be bool".into());
                }
                // Check bodies
            }
            Stmt::For { .. } | Stmt::ForKV { .. } | Stmt::While { .. } => {
                // Loop bodies ignored for now
            }
            _ => {}
        }
        Ok(())
    }

    fn infer_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Null => Ok(Type::Null),
            Expr::Ident(name) => self
                .env
                .get(name)
                .ok_or(format!("Undefined variable '{}'", name)),
            Expr::BinOp { op, left, right } => {
                let l = self.infer_expr(left)?;
                let r = self.infer_expr(right)?;
                match op {
                    BinOp::Plus | BinOp::Minus | BinOp::Star | BinOp::Slash | BinOp::Percent => {
                        if l == Type::Int && r == Type::Int {
                            Ok(Type::Int)
                        } else if (l == Type::Int || l == Type::Float)
                            && (r == Type::Int || r == Type::Float)
                        {
                            Ok(Type::Float)
                        } else if l == Type::Str && r == Type::Str && matches!(op, BinOp::Plus) {
                            Ok(Type::Str)
                        } else {
                            Err("Invalid operands for arithmetic".into())
                        }
                    }
                    BinOp::EqEq
                    | BinOp::BangEq
                    | BinOp::Lt
                    | BinOp::Gt
                    | BinOp::LtEq
                    | BinOp::GtEq => Ok(Type::Bool),
                    BinOp::And | BinOp::Or => {
                        if l == Type::Bool && r == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err("Logical operators require bool".into())
                        }
                    }
                    BinOp::DotDot => Ok(Type::List(Box::new(Type::Int))),
                }
            }
            Expr::List(items) => {
                if items.is_empty() {
                    return Ok(Type::List(Box::new(Type::Any)));
                }
                let first = self.infer_expr(&items[0])?;
                Ok(Type::List(Box::new(first)))
            }
            Expr::Call { callee, args } => {
                let c_ty = self.infer_expr(callee)?;
                if let Type::Fn(param_tys, ret_ty) = c_ty {
                    if param_tys.len() != args.len() {
                        return Err("Argument count mismatch".into());
                    }
                    Ok(*ret_ty)
                } else if let Type::Struct(name) = c_ty {
                    Ok(Type::Struct(name)) // Constructor
                } else {
                    Err("Expression is not callable".into())
                }
            }
            _ => Ok(Type::Any),
        }
    }
}
