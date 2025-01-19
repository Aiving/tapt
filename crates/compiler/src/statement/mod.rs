mod expression;
mod func;
mod record;
mod structure;
mod variable;

use crate::{prelude::*, CompileAssign, GetType};

impl Compile for Statement {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        match self {
            Self::Variable(value) => value.compile(compiler, span, chunk),
            Self::Struct(value) => value.compile(compiler, span, chunk),
            Self::Record(value) => value.compile(compiler, span, chunk),
            Self::Func(value) => value.compile(compiler, span, chunk),
            Self::Expression(value) => value.compile(compiler, span, chunk, None),
            _ => todo!(),
        }
    }
}

impl GetType for Statement {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        match self {
            Self::Expression(value) => value.get_type(compiler, span),
            _ => Ok(Type::None),
        }
    }
}