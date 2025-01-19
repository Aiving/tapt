use crate::{CompileAssign, GetType, prelude::*};

impl Compile for BinaryExpression {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        if matches!(self.operator.value, Operator::And | Operator::Or) {
            {
                let (span, value) = self.lhs.unpack();

                value.compile(compiler, span, chunk, None)?;
            }

            match self.operator.value {
                Operator::And => {
                    let (start, end) = compiler.track_position(chunk, move |compiler, chunk| {
                        chunk.push(span.line, OpCode::JumpIfFalse(0));
                        chunk.push(span.line, OpCode::Pop);

                        let (span, value) = self.rhs.unpack();

                        value.compile(compiler, span, chunk, None)
                    })?;

                    Compiler::patch_jump(chunk, start, end);
                }
                Operator::Or => {
                    let (start, end) = compiler.track_position(chunk, move |compiler, chunk| {
                        chunk.push(span.line, OpCode::JumpIfFalse(0));
                        chunk.push(span.line, OpCode::Jump(0));
                        chunk.push(span.line, OpCode::Pop);

                        let (span, value) = self.rhs.unpack();

                        value.compile(compiler, span, chunk, None)
                    })?;

                    Compiler::patch_jump(chunk, start, start + 1);
                    Compiler::patch_jump(chunk, start + 1, end);
                }
                _ => unreachable!(),
            }
        } else if self.operator.value == Operator::Assign {
            let (span, value) = self.lhs.unpack();

            value.compile(compiler, span, chunk, Some(self.rhs))?;
        } else {
            {
                let (span, value) = self.lhs.unpack();

                value.compile(compiler, span, chunk, None)?;

                let (span, value) = self.rhs.unpack();

                value.compile(compiler, span, chunk, None)?;
            }

            match self.operator.value {
                Operator::Add => chunk.push(self.operator.span.line, OpCode::Add),
                Operator::Sub => chunk.push(self.operator.span.line, OpCode::Sub),
                Operator::Mul => chunk.push(self.operator.span.line, OpCode::Mul),
                Operator::Div => chunk.push(self.operator.span.line, OpCode::Div),
                Operator::Equal => {
                    chunk.push(self.operator.span.line, OpCode::Equal);
                }
                Operator::NotEqual => {
                    chunk.push(self.operator.span.line, OpCode::Equal);
                    chunk.push(self.operator.span.line, OpCode::Negate);
                }
                Operator::LessThan => {
                    chunk.push(self.operator.span.line, OpCode::Less);
                }
                Operator::GreaterThan => {
                    chunk.push(self.operator.span.line, OpCode::Greater);
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl GetType for BinaryExpression {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        let primary = self.lhs.get_type(compiler, span)?;
        let maybe_primary = self.rhs.get_type(compiler, span)?;

        if matches!(
            self.operator.value,
            Operator::Add
                | Operator::Sub
                | Operator::Mul
                | Operator::Div
                | Operator::GreaterThan
                | Operator::LessThan
        ) && !matches!(primary, Type::Float | Type::Integer)
        {
            return Err(CompileError::OneOfTypeExpected {
                expected: vec![Type::Float, Type::Integer],
                found: primary,
                at: self.lhs.span,
            });
        }

        if matches!(self.operator.value, Operator::And | Operator::Or) && primary != Type::Boolean {
            return Err(CompileError::TypeExpected {
                expected: Type::Boolean,
                found: primary,
                at: self.lhs.span,
            });
        }

        if primary != maybe_primary {
            return Err(CompileError::TypeExpected {
                expected: primary,
                found: maybe_primary,
                at: self.rhs.span,
            });
        }

        if matches!(
            self.operator.value,
            Operator::Equal | Operator::GreaterThan | Operator::LessThan | Operator::NotEqual
        ) {
            return Ok(Type::Boolean);
        }

        Ok(primary)
    }
}
