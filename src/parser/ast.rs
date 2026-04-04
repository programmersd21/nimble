use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    Ident(String),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<CallArg>,
    },
    Index {
        obj: Box<Expr>,
        idx: Box<Expr>,
    },
    Field {
        obj: Box<Expr>,
        field: String,
    },
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Lambda {
        params: Vec<Param>,
        ret_ty: Option<Type>,
        body: Vec<Stmt>,
    },
    Interp(Vec<InterpPart>),
    Ternary {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Propagate(Box<Expr>),
    Spawn(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum InterpPart {
    Str(String),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct CallArg {
    pub name: Option<String>,
    pub expr: Expr,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    DotDot,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Minus,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign {
        target: String,
        ty: Option<Type>,
        value: Expr,
    },
    FieldAssign {
        obj: Expr,
        field: String,
        value: Expr,
    },
    IndexAssign {
        obj: Expr,
        idx: Expr,
        value: Expr,
    },
    Return(Option<Expr>),
    If {
        cond: Expr,
        then: Vec<Stmt>,
        elifs: Vec<(Expr, Vec<Stmt>)>,
        else_: Option<Vec<Stmt>>,
    },
    For {
        var: String,
        iter: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    ForKV {
        key: String,
        val: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Expr(Expr),
    Load {
        alias: String,
        source: String,
    },
    FnDef(FnDef),
    ClsDef(ClsDef),
    Export(Box<Stmt>),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClsDef {
    pub name: String,
    pub fields: Vec<Param>,
}
