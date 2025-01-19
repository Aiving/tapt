use crate::{CompileAssign, CompilePositioned, GetType, prelude::*};

impl Compile for IfElseExpression {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        let condition_type = self.condition.get_type(compiler, span)?;

        if condition_type != Type::Boolean {
            return Err(CompileError::TypeExpected {
                expected: Type::Boolean,
                found: condition_type,
                at: self.condition.span,
            });
        }

        {
            let (span, value) = self.condition.unpack();

            value.compile(compiler, span, chunk, None)?;
        }

        let (start, end) = compiler.track_position(chunk, move |compiler, chunk| {
            chunk.push(span.line, OpCode::JumpIfFalse(0));

            self.block.compile(compiler, chunk)?;

            Ok(())
        })?;

        Compiler::patch_jump(chunk, start, end - 1);

        if let Some(else_block) = self.else_block {
            Compiler::patch_jump(chunk, start, end);

            let (start, end) = compiler.track_position(chunk, move |compiler, chunk| {
                chunk.push(span.line, OpCode::Jump(0));

                let (span, value) = else_block.unpack();

                value.compile(compiler, span, chunk, None)
            })?;

            Compiler::patch_jump(chunk, start, end - 1);
        }

        Ok(())
    }
}

impl GetType for IfElseExpression {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        self.block.get_type(compiler, span)
    }
}
