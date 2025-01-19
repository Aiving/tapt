use crate::{CompileAssign, GetType, prelude::*};

impl Compile for MatchExpression {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        let target_type = self.target.get_type(compiler, span)?;

        {
            let (span, value) = self.target.unpack();

            value.compile(compiler, span, chunk, None)?;
        }

        let mut jumps = Vec::new();

        for variant in self.variants {
            match variant.value.case.value {
                MatchCase::Ident(ident) => {
                    chunk.push(variant.span.line, OpCode::PushFrame);

                    compiler.push_scope();

                    let slot = compiler.create_var(
                        ident.to_string(),
                        false,
                        target_type,
                        Some(variant.value.case.span),
                    );

                    chunk.push(
                        variant.value.case.span.line,
                        OpCode::SetLocal(None, slot),
                    );

                    {
                        let (span, value) = variant.value.then.unpack();

                        value.compile(compiler, span, chunk, None)?;
                    }

                    chunk.push(variant.span.line, OpCode::Return);

                    compiler.pop_scope(variant.span.line, chunk);

                    chunk.push(variant.span.line, OpCode::PopFrame);

                    break; // ignore next variants :jokerge:
                }
                MatchCase::Value(expression) => {
                    let case_type = expression.get_type(compiler, span)?;

                    if case_type != target_type {
                        return Err(CompileError::TypeExpected {
                            expected: target_type,
                            found: case_type,
                            at: variant.value.case.span,
                        });
                    }

                    chunk.push(variant.span.line, OpCode::Copy);

                    expression.compile(compiler, variant.value.case.span, chunk, None)?;

                    chunk.push(variant.value.case.span.line, OpCode::Equal);

                    let (start, end) = compiler.track_position(chunk, |compiler, chunk| {
                        chunk.push(
                            variant.value.case.span.line,
                            OpCode::JumpIfFalse(0),
                        );
                        chunk.push(variant.value.case.span.line, OpCode::Pop);

                        let (span, value) = variant.value.then.unpack();

                        value.compile(compiler, span, chunk, None)?;

                        chunk.push(variant.value.case.span.line, OpCode::Jump(0));

                        Ok(())
                    })?;

                    Compiler::patch_jump(chunk, start, end - 1);

                    jumps.push(end - 1);
                }
            }
        }

        let end = chunk.len() - 1;

        for jump in jumps {
            Compiler::patch_jump(chunk, jump, end);
        }

        Ok(())
    }
}

impl GetType for MatchExpression {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        match self.variants.len() {
            0 => Ok(Type::None),
            1 => self.variants[0].value.then.get_type(compiler, span),
            _ => {
                let primary = self.variants[0].value.then.get_type(compiler, span)?;

                for variant in &self.variants[1..] {
                    let maybe_primary = variant.value.then.get_type(compiler, span)?;

                    if maybe_primary != primary {
                        return Err(CompileError::TypeExpected {
                            expected: primary,
                            found: maybe_primary,
                            at: variant.value.then.span,
                        });
                    }
                }

                Ok(primary)
            }
        }
    }
}
