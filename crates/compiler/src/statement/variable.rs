use crate::{CompileAssign, GetType, prelude::*};

impl Compile for VariableStatement {
    fn compile(
        self,
        compiler: &mut crate::Compiler,
        span: Span,
        chunk: &mut Chunk,
    ) -> crate::CompileResult<()> {
        let slot = compiler.get_or_create_var(
            self.name.to_string(),
            self.value.get_type(compiler, span)?,
            self.mutable.value,
            Some(span),
        );

        let (span, value) = self.value.unpack();

        value.compile(compiler, span, chunk, None)?;

        chunk.push(span.line, OpCode::SetLocal(None, slot));

        Ok(())
    }
}
