use std::collections::HashMap;

/// The internal representation of a Nimble type during semantic analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,

    /// A user‑defined interface type (structural constraint).
    Interface(String),

    /// A user‑defined struct type (nominal).
    Struct(String),

    /// A generic instantiation, e.g. `Array[Int]` → `GenericInstance("Array", [Int])`.
    GenericInstance(String, Vec<Type>),

    /// A type variable created during Hindley‑Milner inference.
    Variable(usize),

    /// Function type: `Fn(params...) -> return_type`.
    Function(Vec<Type>, Box<Type>),
}

/// A mapping from type‑variable IDs to their resolved types.
///
/// Used to communicate results of unification back to the caller.
pub type Substitution = HashMap<usize, Type>;


impl Type {
    /// Create a fresh type variable with the given id.
    pub fn new_var(id: usize) -> Self {
        Type::Variable(id)
    }

    /// Apply a substitution to `self`, recursively replacing variables.
    pub fn apply(&self, subst: &Substitution) -> Type {
        match self {
            Type::Variable(id) => {
                match subst.get(id) {
                    Some(resolved) => resolved.apply(subst),
                    None => Type::Variable(*id),
                }
            }
            Type::Function(params, ret) => {
                let new_params: Vec<Type> =
                    params.iter().map(|p| p.apply(subst)).collect();
                let new_ret = ret.apply(subst);
                Type::Function(new_params, Box::new(new_ret))
            }
            Type::GenericInstance(name, args) => {
                let new_args: Vec<Type> =
                    args.iter().map(|a| a.apply(subst)).collect();
                Type::GenericInstance(name.clone(), new_args)
            }
            other => other.clone(),
        }
    }

    /// Collect all free (i.e. unresolved) type‑variable ids contained in this
    /// type.  Used by the occurs check in unification.
    pub fn free_vars(&self) -> Vec<usize> {
        match self {
            Type::Variable(id) => vec![*id],
            Type::Function(params, ret) => {
                let mut vars = ret.free_vars();
                for p in params {
                    vars.extend(p.free_vars());
                }
                vars
            }
            Type::GenericInstance(_, args) => {
                let mut vars = Vec::new();
                for a in args {
                    vars.extend(a.free_vars());
                }
                vars
            }
            _ => vec![],
        }
    }
}

/// Pretty‑print a type for error messages.
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::Void => write!(f, "Void"),
            Type::Interface(name) => write!(f, "interface {}", name),
            Type::Struct(name) => write!(f, "struct {}", name),
            Type::GenericInstance(name, args) => {
                write!(f, "{}[", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, "]")
            }
            Type::Variable(id) => write!(f, "?{}", id),
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
        }
    }
}


impl From<&crate::ast::Type> for Type {
    fn from(t: &crate::ast::Type) -> Self {
        match t.name.to_lowercase().as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => {
                Type::Struct(t.name.clone())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_variable_creation() {
        let tv = Type::new_var(42);
        assert_eq!(tv, Type::Variable(42));
    }

    #[test]
    fn apply_substitution_to_variable() {
        let mut subst = Substitution::new();
        subst.insert(0, Type::Int);
        let tv = Type::new_var(0);
        assert_eq!(tv.apply(&subst), Type::Int);
    }

    #[test]
    fn apply_substitution_recursively() {
        let mut subst = Substitution::new();
        subst.insert(0, Type::Int);
        let ft = Type::Function(vec![Type::new_var(0)], Box::new(Type::Bool));
        let expected = Type::Function(vec![Type::Int], Box::new(Type::Bool));
        assert_eq!(ft.apply(&subst), expected);
    }

    #[test]
    fn free_vars_of_function_type() {
        let ft = Type::Function(
            vec![Type::new_var(0), Type::Int],
            Box::new(Type::new_var(1)),
        );
        let mut vars = ft.free_vars();
        vars.sort();
        assert_eq!(vars, vec![0, 1]);
    }

    #[test]
    fn free_vars_no_variables() {
        assert!(Type::Int.free_vars().is_empty());
        assert!(Type::Bool.free_vars().is_empty());
    }

    #[test]
    fn display_type() {
        assert_eq!(format!("{}", Type::Int), "Int");
        assert_eq!(format!("{}", Type::new_var(0)), "?0");
        assert_eq!(
            format!("{}", Type::Function(vec![Type::Int], Box::new(Type::Bool))),
            "fn(Int) -> Bool"
        );
    }

    #[test]
    fn from_ast_type_primitive() {
        let ast_t = crate::ast::Type {
            name: "Int".into(),
            span: crate::lexer::Span::new(1, 1, 0),
        };
        assert_eq!(Type::from(&ast_t), Type::Int);
    }

    #[test]
    fn from_ast_type_user_defined() {
        let ast_t = crate::ast::Type {
            name: "MyStruct".into(),
            span: crate::lexer::Span::new(1, 1, 0),
        };
        assert_eq!(Type::from(&ast_t), Type::Struct("MyStruct".into()));
    }
}
