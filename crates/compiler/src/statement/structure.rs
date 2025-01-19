use crate::prelude::*;

impl Compile for StructStatement {
    fn compile(
        self,
        compiler: &mut crate::Compiler,
        span: Span,
        chunk: &mut Chunk,
    ) -> crate::CompileResult<()> {
        let fields: Vec<(String, Type)> = self
            .fields
            .value
            .into_iter()
            .map(|field| (field.value.name.value.0, field.value.ty.value))
            .collect();

        let variable = compiler.create_var(
            self.name.to_string(),
            false,
            Type::Struct(StructType {
                name: self.name.to_string(),
                fields: fields.clone(),
            }),
            Some(span),
        );

        let constant = Compiler::create_const(
            chunk,
            Value::object(Object::Struct(Struct {
                name: self.name.value.0,
                fields,
            })),
        );

        chunk.push(span.line, OpCode::LoadConst(constant));
        chunk.push(span.line, OpCode::SetLocal(None, variable));

        Ok(())
    }
}
