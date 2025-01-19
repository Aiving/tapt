use tapt_parser::Ident;

use crate::{CompileAssign, GetType, prelude::*};

impl CompileAssign for Ident {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
        assign_value: Option<Positioned<Expression>>,
    ) -> CompileResult<()> {
        let (slot, variable) = compiler.get_var(span, &self.0)?;

        if let Some(value) = assign_value {
            if variable.mutable {
                let value_type = value.get_type(compiler, span)?;

                if value_type != variable.ty {
                    return Err(CompileError::TypeExpected {
                        expected: variable.ty.clone(),
                        found: value_type,
                        at: value.span,
                    });
                }

                let depth = variable.depth;

                {
                    let (span, value) = value.unpack();

                    value.compile(compiler, span, chunk, None)?;
                }

                chunk.push(span.line, OpCode::SetLocal(Some(depth), slot));
            } else {
                return Err(CompileError::ImmutableVariable {
                    name: self.to_string(),
                    accessed_at: span,
                    declared_at: variable.span,
                });
            }
        } else {
            chunk.push(
                span.line,
                OpCode::GetLocal(
                    variable.depth as isize - compiler.scope_depth as isize,
                    slot,
                ),
            );
        }

        Ok(())
    }
}

impl GetType for Ident {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        compiler
            .get_var(span, &self.0)
            .map(|(_, value)| value.ty.clone())
    }
}
