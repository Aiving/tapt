mod binary;
mod block;
mod call;
mod ident;
mod if_else;
mod index;
mod literal;
mod matching;
mod new;

use crate::{CompileAssign, GetType, prelude::*};

impl CompileAssign for Expression {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
        assign_value: Option<Positioned<Expression>>,
    ) -> CompileResult<()> {
        match self {
            Self::Literal(value) => value.compile(compiler, span, chunk),
            Self::FunctionCall(value) => value.compile(compiler, span, chunk),
            Self::Ident(value) => value.compile(compiler, span, chunk, assign_value),
            Self::NewInstance(value) => value.compile(compiler, span, chunk),
            Self::IfElse(value) => value.compile(compiler, span, chunk),
            Self::Block(value) => {
                value.compile(compiler, span, chunk)?;

                Ok(())
            }
            Self::Match(value) => value.compile(compiler, span, chunk),
            Self::Binary(value) => value.compile(compiler, span, chunk),
            Self::Index(value) => value.compile(compiler, span, chunk, assign_value),
            _ => todo!(),
        }
    }
}

impl GetType for Expression {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        match self {
            Self::Literal(value) => value.get_type(compiler, span),
            Self::FunctionCall(value) => value.get_type(compiler, span), // no functions
            Self::Ident(value) => value.get_type(compiler, span),
            Self::Object(_) => todo!(), // no objects
            Self::Array(_) => todo!(),  // no arrays
            Self::IfElse(value) => value.get_type(compiler, span),
            Self::Block(value) => value.get_type(compiler, span),
            Self::Match(value) => value.get_type(compiler, span),
            Self::Binary(value) => value.get_type(compiler, span),
            Self::Index(value) => value.get_type(compiler, span),
            Self::NewInstance(value) => value.target.get_type(compiler, span), // no objects/arrays
        }
    }
}
