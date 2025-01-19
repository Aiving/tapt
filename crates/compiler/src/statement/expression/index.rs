use crate::{CompileAssign, GetType, prelude::*};

impl CompileAssign for IndexExpression {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
        assign_value: Option<Positioned<Expression>>,
    ) -> CompileResult<()> {
        let target = self.target.get_type(compiler, span)?;

        {
            let (span, value) = self.target.unpack();

            value.compile(compiler, span, chunk, None)?;
        }

        if let Type::Record(RecordType { fields, .. }) = target {
            if let IndexKind::Number(index) = &self.index.value {
                if *index > fields.len() {
                    todo!()
                } else {
                    if let Some(assign_value) = assign_value {
                        let (span, value) = assign_value.unpack();

                        value.compile(compiler, span, chunk, None)?;

                        chunk.push(self.index.span.line, OpCode::SetProperty(*index));
                    } else {
                        chunk.push(self.index.span.line, OpCode::GetProperty(*index));
                    }

                    Ok(())
                }
            } else {
                todo!()
            }
        } else if let Type::Struct(structure) = target {
            if let IndexKind::Ident(name) = &self.index.value {
                if let Some(index) = structure
                    .fields
                    .iter()
                    .position(|(field_name, _)| field_name == &**name)
                {
                    if let Some(assign_value) = assign_value {
                        let (span, value) = assign_value.unpack();

                        value.compile(compiler, span, chunk, None)?;

                        chunk.push(self.index.span.line, OpCode::SetProperty(index));
                    } else {
                        chunk.push(self.index.span.line, OpCode::GetProperty(index));
                    }

                    Ok(())
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl GetType for IndexExpression {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        let target = self.target.get_type(compiler, span)?;

        if let Type::Record(RecordType { fields, .. }) = target {
            if let IndexKind::Number(index) = &self.index.value {
                if *index > fields.len() {
                    todo!()
                } else {
                    Ok(fields[*index].clone())
                }
            } else {
                todo!()
            }
        } else if let Type::Struct(structure) = target {
            if let IndexKind::Ident(name) = &self.index.value {
                if let Some((_, value)) = structure
                    .fields
                    .iter()
                    .find(|(field_name, _)| field_name == &**name)
                {
                    Ok(value.clone())
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}
