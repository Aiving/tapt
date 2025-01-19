use crate::{prelude::*, CompilePositioned, GetType};

impl Compile for FuncStatement {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
    ) -> crate::CompileResult<()> {
        let mut alt_compiler = Compiler::new();
        let mut function_chunk = Chunk::new();

        alt_compiler.push_scope();

        let mut args = Vec::new();

        for arg in self.args.value {
            args.push(arg.value.ty.value.clone());

            alt_compiler.create_var(
                arg.value.name.value.0,
                true,
                arg.value.ty.value,
                Some(arg.span),
            );
        }

        let output_span = self.output_type.as_ref().map(|value| value.span);
        let output = self.output_type.map_or(Type::None, |value| value.value);

        let ty = Type::Function(FunctionType {
            args: args.clone(),
            output_type: Box::new(output.clone()),
        });

        let variable = compiler.create_var(self.name.to_string(), false, ty.clone(), Some(span));

        alt_compiler.create_var(self.name.to_string(), false, ty, Some(span));

        for value in self.body.value.statements {
            value.compile(&mut alt_compiler, &mut function_chunk)?;
        }

        let mut body_output_type = Type::None;

        if let Some(statement) = self.body.value.return_statement {
            body_output_type = statement.get_type(&alt_compiler, span)?;

            statement.compile(&mut alt_compiler, &mut function_chunk)?;

            function_chunk.push(self.body.span.line, OpCode::Return);
        }

        alt_compiler.pop_scope(self.body.span.line, &mut function_chunk);

        function_chunk.push(0, OpCode::Halt);

        if body_output_type != output {
            return Err(CompileError::TypeExpected {
                expected: output,
                found: body_output_type,
                at: span,
            });
        }

        let constant = Compiler::create_const(
            chunk,
            Value::object(Object::Function(Function {
                meta: FunctionMetadata {
                    name: self.name.value.0,
                    args,
                    output,
                },
                chunk: function_chunk,
            })),
        );

        chunk.push(span.line, OpCode::LoadConst(constant));
        chunk.push(span.line, OpCode::SetLocal(None, variable));

        Ok(())
    }
}
