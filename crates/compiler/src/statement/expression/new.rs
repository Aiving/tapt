use crate::{CompileAssign, GetType, InstanceArgsType, prelude::*};

impl Compile for NewInstanceExpression {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        let (slot, variable) = compiler.get_var(self.target.span, &self.target.value.0)?;

        let depth = variable.depth;

        if let Type::Record(RecordType { fields, .. }) = &variable.ty {
            if let InstanceArgs::Record(values) = self.args.value {
                if values.len() != fields.len() {
                    return Err(CompileError::InvalidArgumentsCount {
                        expected: fields.len(),
                        got: values.len(),
                        function_at: variable.span,
                        at: span,
                    });
                }

                for (value, field) in values.iter().zip(fields) {
                    let ty = value.get_type(compiler, span)?;

                    if !ty.compare(field) {
                        return Err(CompileError::TypeExpected {
                            expected: field.clone(),
                            found: ty,
                            at: value.span,
                        });
                    }
                }

                for value in values {
                    let (span, value) = value.unpack();

                    value.compile(compiler, span, chunk, None)?;
                }

                chunk.push(
                    span.line,
                    OpCode::GetLocal(depth as isize - compiler.scope_depth as isize, slot),
                );

                chunk.push(span.line, OpCode::CreateInstance);

                Ok(())
            } else {
                Err(CompileError::InvalidInstanceArgs {
                    expected: InstanceArgsType::Record,
                    got: InstanceArgsType::Struct,
                    instance_at: variable.span,
                    at: self.args.span,
                })
            }
        } else if let Type::Struct(structure) = &variable.ty {
            if let InstanceArgs::Struct(mut values) = self.args.value {
                for value in &values {
                    if structure
                        .fields
                        .iter()
                        .position(|field| field.0 == *value.value.name.value)
                        .is_none()
                    {
                        return Err(CompileError::PropertyNotExist {
                            target: structure.name.clone(),
                            property: value.value.name.value.0.clone(),
                            defined_at: variable.span,
                            at: value.span,
                        });
                    }
                }

                values.sort_by_key(|value| {
                    structure
                        .fields
                        .iter()
                        .position(|field| field.0 == *value.value.name.value)
                });

                for (value, (_, field)) in values.iter().zip(&structure.fields) {
                    let ty = value.value.value.get_type(compiler, span)?;

                    if !ty.compare(field) {
                        return Err(CompileError::TypeExpected {
                            expected: field.clone(),
                            found: ty,
                            at: value.value.value.span,
                        });
                    }
                }

                for value in values {
                    let (span, value) = value.value.value.unpack();

                    value.compile(compiler, span, chunk, None)?;
                }

                chunk.push(
                    span.line,
                    OpCode::GetLocal(depth as isize - compiler.scope_depth as isize, slot),
                );

                chunk.push(span.line, OpCode::CreateInstance);

                Ok(())
            } else {
                Err(CompileError::InvalidInstanceArgs {
                    expected: InstanceArgsType::Struct,
                    got: InstanceArgsType::Record,
                    instance_at: variable.span,
                    at: self.args.span,
                })
            }
        } else {
            Err(CompileError::OneOfTypeExpected {
                expected: vec![
                    Type::Record(RecordType {
                        name: String::new(),
                        fields: Vec::new(),
                    }),
                    Type::Struct(StructType {
                        name: String::new(),
                        fields: Vec::new(),
                    }),
                ],
                found: variable.ty.clone(),
                at: self.target.span,
            })
        }
    }
}
