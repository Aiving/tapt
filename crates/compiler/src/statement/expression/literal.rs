use crate::{GetType, prelude::*};

impl Compile for Literal {
    fn compile(self, _: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        Compiler::compile_const(chunk, span.line as usize, match self {
            Self::Range(_) => todo!(),
            Self::Number(value) => match value {
                Number::Float(value) => Value::Float(value),
                Number::Int(value) => Value::Integer(value),
            },
            Self::Boolean(value) => Value::Boolean(value),
            Self::String(value) => Value::object(Object::String(value)),
        });

        Ok(())
    }
}

impl GetType for Literal {
    fn get_type(&self, _: &Compiler, _: Span) -> CompileResult<Type> {
        Ok(match self {
            Self::Number(value) => match value {
                Number::Float(_) => Type::Float,
                Number::Int(_) => Type::Integer,
            },
            Self::String(_) => Type::String,
            Self::Boolean(_) => Type::Boolean,
            Self::Range(_) => todo!(), // no ranges
        })
    }
}
