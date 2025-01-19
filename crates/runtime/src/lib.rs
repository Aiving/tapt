pub mod prelude {
    pub use crate::{FunctionBuilder, Runtime, RuntimeError};
    pub use tapt_compiler::prelude::*;
}

use crate::prelude::*;
use std::rc::Rc;

pub struct FunctionBuilder {
    name: String,
    args: Vec<Type>,
}

impl FunctionBuilder {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
        }
    }

    #[must_use]
    pub fn any_arg(mut self) -> Self {
        self.args.push(Type::Any);

        self
    }

    #[must_use]
    pub fn arg<T: AsType>(mut self) -> Self {
        self.args.push(T::as_type());

        self
    }

    pub fn build<O: Into<Value> + AsType, F: Fn(&VM, Args) -> O + 'static>(
        self,
        runtime: &mut Runtime,
        body: F,
    ) {
        let slot = runtime
            .vm
            .state
            .downcast_mut::<Compiler>()
            .unwrap()
            .add_native_func(&self.name, self.args.clone(), Some(O::as_type()));

        runtime
            .vm
            .frame_mut()
            .set_slot(slot, self.build_value(body));
    }

    pub fn build_value<O: Into<Value> + AsType, F: Fn(&VM, Args) -> O + 'static>(
        self,
        body: F,
    ) -> Value {
        Value::object(Object::NativeFunction(NativeFunction {
            meta: FunctionMetadata {
                name: self.name,
                args: self.args,
                output: O::as_type(),
            },
            func: Rc::new(move |vm, args| body(vm, args).into()),
        }))
    }
}

pub enum RuntimeError {
    ParseError(ParseError),
    CompileError(CompileError),
}

pub struct Runtime {
    vm: VM,
}

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        Self {
            vm: VM::new(Compiler::new()),
        }
    }

    pub fn run<T: AsRef<str>>(&mut self, code: T) -> Result<Value, RuntimeError> {
        let code = code.as_ref();
        let mut parser = Parser::new(Lexer::parse(code));

        match Block::parse_statements_until(&mut parser, &Token::EOF) {
            Ok((statements, return_statement)) => {
                println!("{statements:#?}");

                match self
                    .vm
                    .state
                    .downcast_mut::<Compiler>()
                    .unwrap()
                    .compile(statements, return_statement)
                {
                    Ok(value) => {
                        println!("{value:#?}");

                        Ok(self.vm.interpret(&value))
                    }
                    Err(err) => Err(RuntimeError::CompileError(err)),
                }
            }
            Err(err) => Err(RuntimeError::ParseError(err)),
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    
    #[test]
    fn test_runtime() {
        let mut runtime = Runtime::new();

        println!(
            "{:#?}",
            FunctionBuilder::new("sum")
                .arg::<i64>()
                .arg::<i64>()
                .build_value(|_, mut value| {
                    let a = value.get::<i64>();
                    let b = value.get::<i64>();

                    a + b
                },)
        );

        FunctionBuilder::new("println")
            .any_arg()
            .build(&mut runtime, |_, mut value| {
                let value = value.get::<Value>();

                println!("{value}");
            });

        FunctionBuilder::new("sum").arg::<i64>().arg::<i64>().build(
            &mut runtime,
            |_, mut value| {
                let a = value.get::<i64>();
                let b = value.get::<i64>();

                a + b
            },
        );

        runtime.run("println(sum(20, 40))");
    }
}
