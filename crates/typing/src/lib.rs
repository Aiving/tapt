use derive_more::derive::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    #[display("any")]
    Any,
    #[display("none")]
    None,
    #[display("int")]
    Integer,
    #[display("float")]
    Float,
    #[display("bool")]
    Boolean,
    #[display("string")]
    String,
    #[display("func")]
    Function(FunctionType),
    #[display("record")]
    Record(RecordType),
    #[display("struct")]
    Struct(StructType),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionType {
    pub args: Vec<Type>,
    pub output_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordType {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

pub trait AsType {
    fn as_type() -> Type;
}

impl AsType for bool {
    fn as_type() -> Type {
        Type::Boolean
    }
}

impl AsType for i64 {
    fn as_type() -> Type {
        Type::Integer
    }
}

impl AsType for f32 {
    fn as_type() -> Type {
        Type::Float
    }
}

impl AsType for String {
    fn as_type() -> Type {
        Type::String
    }
}

impl AsType for () {
    fn as_type() -> Type {
        Type::None
    }
}

impl Type {
    #[must_use]
    pub fn compare(&self, other: &Self) -> bool {
        if matches!(self, Self::Any) || matches!(other, Self::Any) {
            true
        } else {
            self == other
        }
    }
}
