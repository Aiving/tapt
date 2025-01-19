use crate::prelude::*;

impl Compile for RecordStatement {
    fn compile(
        self,
        compiler: &mut crate::Compiler,
        span: Span,
        chunk: &mut Chunk,
    ) -> crate::CompileResult<()> {
        let fields = self
            .fields
            .value
            .into_iter()
            .map(|field| field.value)
            .collect::<Vec<_>>();

        let variable = compiler.create_var(
            self.name.to_string(),
            false,
            Type::Record(RecordType {
                name: self.name.to_string(),
                fields: fields.clone(),
            }),
            Some(span),
        );

        let constant = Compiler::create_const(
            chunk,
            Value::object(Object::Record(Record {
                name: self.name.value.0,
                fields,
            })),
        );

        chunk.push(span.line, OpCode::LoadConst(constant));
        chunk.push(span.line, OpCode::SetLocal(None, variable));

        Ok(())
    }
}
