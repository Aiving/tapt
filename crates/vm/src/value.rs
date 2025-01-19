use std::{cmp::Ordering, fmt, rc::Rc, cell::RefCell, vec::IntoIter};

use tapt_typing::Type;

use crate::{chunk::Chunk, VM};

pub struct Args {
    pub(crate) args: IntoIter<Value>,
}

impl Args {
    pub fn get<T: From<Value>>(&mut self) -> T {
        let Some(value) = self.args.next() else {
            unreachable!()
        };

        value.into()
    }
}

pub type NativeFunc = Rc<dyn Fn(&VM, Args) -> Value>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    String(String),
    Function(Function),
    NativeFunction(NativeFunction),
    Struct(Struct),
    Record(Record),
    StructInstance(StructInstance),
    RecordInstance(RecordInstance),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Record {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RecordInstance {
    pub name: String,
    pub fields: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructInstance {
    pub name: String,
    pub fields: Vec<(String, Value)>,
}

impl PartialOrd for StructInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub meta: FunctionMetadata,
    pub func: NativeFunc,
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("meta", &self.meta)
            .finish_non_exhaustive()
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta
    }
}

impl PartialOrd for NativeFunction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.meta.partial_cmp(&other.meta)
    }
}

impl Object {
    fn as_string(&self) -> Option<String> {
        if let Self::String(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FunctionMetadata {
    pub name: String,
    pub args: Vec<Type>,
    pub output: Type,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function {
    pub meta: FunctionMetadata,
    pub chunk: Chunk,
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Integer(i64),
    Float(f32),
    Boolean(bool),
    Object(Rc<RefCell<Object>>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Boolean(l0), Self::Boolean(r0)) => l0.partial_cmp(r0),
            (Self::Object(l0), Self::Object(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("()"),
            Self::Integer(value) => value.fmt(f),
            Self::Float(value) => value.fmt(f),
            Self::Boolean(value) => value.fmt(f),
            Self::Object(value) => match &*value.borrow() {
                Object::String(value) => fmt::Debug::fmt(value, f),
                Object::Function(func) => write!(
                    f,
                    "func {}({}): {}",
                    func.meta.name,
                    func.meta
                        .args
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                    func.meta.output
                ),
                Object::NativeFunction(func) => write!(
                    f,
                    "func[native] {}({}): {}",
                    func.meta.name,
                    func.meta
                        .args
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                    func.meta.output
                ),
                Object::Record(instance) => write!(
                    f,
                    "record {}({})",
                    instance.name,
                    instance
                        .fields
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Object::RecordInstance(instance) => write!(
                    f,
                    "record[instance] {}({})",
                    instance.name,
                    instance
                        .fields
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Object::Struct(instance) => write!(
                    f,
                    "struct {} {{\n{}\n}}",
                    instance.name,
                    instance
                        .fields
                        .iter()
                        .map(|(name, ty)| format!("  {name}: {ty}"))
                        .collect::<Vec<_>>()
                        .join(",\n")
                ),
                Object::StructInstance(instance) => write!(
                    f,
                    "struct[instance] {} {{\n{}\n}}",
                    instance.name,
                    instance
                        .fields
                        .iter()
                        .map(|(name, value)| format!("  {name}: {value}"))
                        .collect::<Vec<_>>()
                        .join(",\n")
                ),
            },
        }
    }
}

impl Value {
    #[must_use]
    pub fn object(value: Object) -> Self {
        Self::Object(Rc::new(RefCell::new(value)))
    }

    #[must_use]
    pub fn as_string(&self) -> Option<String> {
        if let Self::Object(value) = self {
            value.borrow().as_string()
        } else {
            None
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::object(Object::String(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::object(Object::String(value.into()))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Self::None
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        let Value::Boolean(value) = value else {
            unreachable!()
        };

        value
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        let Value::Integer(value) = value else {
            unreachable!()
        };

        value
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        let Value::Float(value) = value else {
            unreachable!()
        };

        value
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        let Value::Object(value) = value else {
            unreachable!()
        };

        let Object::String(value) = &*value.borrow() else {
            unreachable!()
        };

        value.clone()
    }
}
