use crate::{CompilePositioned, GetType, prelude::*};

impl Compile<Type> for Block {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
    ) -> CompileResult<Type> {
        chunk.push(span.line, OpCode::PushFrame);

        compiler.push_scope();

        for value in self.statements {
            value.compile(compiler, chunk)?;
        }

        let mut ty = Type::None;

        if let Some(statement) = self.return_statement {
            ty = statement.get_type(compiler, span)?;

            statement.compile(compiler, chunk)?;

            chunk.push(span.line, OpCode::Return);
        }

        compiler.pop_scope(span.line, chunk);

        chunk.push(span.line, OpCode::PopFrame);

        Ok(ty)
    }
}

impl GetType for Block {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        self.return_statement
            .as_ref()
            .map_or(Ok(Type::None), |stmt| stmt.get_type(compiler, span))
    }
}
