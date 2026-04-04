#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Null,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Struct(String),
    Fn(Vec<Type>, Box<Type>),
    Error(Box<Type>),
    Union(Vec<Type>),
    Unknown,
    Any,
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        if other == &Type::Any || self == other {
            return true;
        }
        match (self, other) {
            (Type::Int, Type::Float) => true, // Auto-coercion
            (Type::Union(variants), _) => variants.iter().any(|v| v.is_assignable_to(other)),
            (_, Type::Union(variants)) => variants.iter().any(|v| self.is_assignable_to(v)),
            _ => false,
        }
    }
}
