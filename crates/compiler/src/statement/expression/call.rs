use crate::{CompileAssign, GetType, prelude::*};

impl Compile for FunctionCall {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<()> {
        let target_ty = self.target.get_type(compiler, span)?;

        if let Type::Function(FunctionType {
            args,
            output_type: _,
        }) = target_ty
        {
            {
                let (span, value) = self.target.unpack();

                value.compile(compiler, span, chunk, None)?;
            }

            if args.len() != self.args.value.len() {
                return Err(CompileError::InvalidArgumentsCount {
                    expected: args.len(),
                    got: self.args.value.len(),
                    function_at: None,
                    at: span,
                });
            }

            let arg_count = self.args.value.len();

            for (provided_arg, expected_arg) in self.args.value.into_iter().zip(args.into_iter()) {
                let ty = provided_arg.get_type(compiler, span)?;

                if !ty.compare(&expected_arg) {
                    return Err(CompileError::TypeExpected {
                        expected: expected_arg,
                        found: ty,
                        at: provided_arg.span,
                    });
                }

                let (span, value) = provided_arg.unpack();

                value.compile(compiler, span, chunk, None)?;
            }

            chunk.push(span.line, OpCode::Call(arg_count));

            Ok(())
        } else {
            Err(CompileError::TypeExpected {
                expected: Type::Function(FunctionType {
                    args: Vec::new(),
                    output_type: Box::new(Type::None),
                }),
                found: target_ty,
                at: self.target.span,
            })
        }
    }
}

impl GetType for FunctionCall {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type> {
        let ty = self.target.get_type(compiler, span)?;

        if let Type::Function(FunctionType { output_type, .. }) = ty {
            Ok(*output_type)
        } else {
            Err(CompileError::TypeExpected {
                expected: Type::Function(FunctionType {
                    args: Vec::new(),
                    output_type: Box::new(Type::None),
                }),
                found: ty,
                at: self.target.span,
            })
        }
    }
}
