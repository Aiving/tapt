mod statement;

pub mod prelude {
    pub use crate::{Compile, CompileError, CompileResult, Compiler};
    pub use tapt_parser::prelude::*;
    pub use tapt_vm::*;
}

use std::fmt;

use tapt_parser::prelude::{Expression, FunctionType, Positioned, Span, Statement, Type};
use tapt_vm::{Chunk, OpCode, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompileError {
    VariableNotExist {
        name: String,
        accessed_at: Span,
    },
    PropertyNotExist {
        target: String,
        property: String,
        defined_at: Option<Span>,
        at: Span,
    },
    ImmutableVariable {
        name: String,
        accessed_at: Span,
        declared_at: Option<Span>,
    },
    TypeExpected {
        expected: Type,
        found: Type,
        at: Span,
    },
    OneOfTypeExpected {
        expected: Vec<Type>,
        found: Type,
        at: Span,
    },
    InvalidArgumentsCount {
        expected: usize,
        got: usize,
        function_at: Option<Span>,
        at: Span,
    },
    InvalidInstanceArgs {
        expected: InstanceArgsType,
        got: InstanceArgsType,
        instance_at: Option<Span>,
        at: Span,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceArgsType {
    Struct,
    Record,
}

impl fmt::Display for InstanceArgsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Struct => "struct",
            Self::Record => "record",
        })
    }
}

pub type CompileResult<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub depth: usize,
    pub mutable: bool,
    pub ty: Type,
    pub span: Option<Span>,
}

#[derive(Default)]
pub struct Compiler {
    pub variables: Vec<Variable>,
    pub scope_depth: usize,
}

pub trait GetType {
    fn get_type(&self, compiler: &Compiler, span: Span) -> CompileResult<Type>;
}

pub trait Compile<O = ()> {
    fn compile(self, compiler: &mut Compiler, span: Span, chunk: &mut Chunk) -> CompileResult<O>;
}

pub trait CompileAssign<O = ()> {
    fn compile(
        self,
        compiler: &mut Compiler,
        span: Span,
        chunk: &mut Chunk,
        assign_value: Option<Positioned<Expression>>,
    ) -> CompileResult<O>;
}

pub trait CompilePositioned<O> {
    fn compile(self, compiler: &mut Compiler, chunk: &mut Chunk) -> CompileResult<O>;
}

impl<O, T: Compile<O>> CompilePositioned<O> for Positioned<T> {
    fn compile(self, compiler: &mut Compiler, chunk: &mut Chunk) -> CompileResult<O> {
        self.value.compile(compiler, self.span, chunk)
    }
}

impl<T: GetType> GetType for Positioned<T> {
    fn get_type(&self, compiler: &Compiler, _: Span) -> CompileResult<Type> {
        self.value.get_type(compiler, self.span)
    }
}

impl Compiler {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            variables: Vec::new(),
            scope_depth: 0,
        }
    }

    pub fn add_native_func<T: Into<String>>(
        &mut self,
        name: T,
        args: Vec<Type>,
        output_type: Option<Type>,
    ) -> usize {
        let name: String = name.into();

        self.create_var(
            name,
            false,
            Type::Function(FunctionType {
                args,
                output_type: Box::new(output_type.unwrap_or(Type::None)),
            }),
            None,
        )
    }

    /// # Errors
    ///
    /// Returns compile error
    pub fn compile(
        &mut self,
        block: Vec<Positioned<Statement>>,
        return_statement: Option<Positioned<Statement>>,
    ) -> CompileResult<Chunk> {
        let mut chunk = Chunk::new();

        for value in block {
            value.compile(self, &mut chunk)?;
        }

        if let Some(statement) = return_statement {
            let line = statement.span.line;

            statement.compile(self, &mut chunk)?;

            chunk.push(line, OpCode::Return);
        }

        chunk.push(0, OpCode::Halt);

        Ok(chunk)
    }

    fn push_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn pop_scope(&mut self, _: usize, _: &mut Chunk) {
        // for _ in self
        //     .variables
        //     .iter()
        //     .filter(|value| value.depth == self.scope_depth)
        // {
        //     chunk.push(line, OpCode::Pop);
        // }

        self.scope_depth -= 1;
    }

    fn track_position(
        &mut self,
        chunk: &mut Chunk,
        func: impl FnOnce(&mut Self, &mut Chunk) -> CompileResult<()>,
    ) -> CompileResult<(usize, usize)> {
        let start = chunk.len();

        func(self, chunk)?;

        Ok((start, chunk.len()))
    }

    pub fn get_var(
        &self,
        accessed_at: Span,
        name: impl AsRef<str>,
    ) -> CompileResult<(usize, &Variable)> {
        let name = name.as_ref();

        for (index, var) in self.variables.iter().enumerate().rev() {
            if var.name == name {
                return Ok((index, var));
            }
        }

        Err(CompileError::VariableNotExist {
            name: name.to_string(),
            accessed_at,
        })
    }

    fn get_or_create_var(
        &mut self,
        name: String,
        ty: Type,
        mutable: bool,
        span: Option<Span>,
    ) -> usize {
        for (index, var) in self.variables.iter_mut().enumerate().rev() {
            // it's already created by type checker (lmao)
            if var.depth == self.scope_depth && var.name == name {
                return index;
            }

            // shadowing :jokerge:
            if var.name == name {
                var.mutable = mutable;
                var.ty = ty;
                var.span = span;

                return index;
            }
        }

        self.create_var(name, mutable, ty, span)
    }

    fn create_var(&mut self, name: String, mutable: bool, ty: Type, span: Option<Span>) -> usize {
        self.variables.push(Variable {
            name,
            depth: self.scope_depth,
            mutable,
            ty,
            span,
        });

        self.variables.len() - 1
    }

    #[allow(clippy::cast_possible_wrap)]
    fn patch_jump(chunk: &mut Chunk, start: usize, end: usize) {
        match &mut chunk[start].1 {
            OpCode::Jump(offset) => *offset = (end as isize) - (start as isize),
            OpCode::JumpIfFalse(offset) => *offset = end - start,
            _ => {}
        }
    }

    fn create_const(chunk: &mut Chunk, value: Value) -> usize {
        let constant = chunk.constants();

        chunk.push_const(value);

        constant
    }

    fn compile_const(chunk: &mut Chunk, line: usize, value: Value) {
        let constant = Self::create_const(chunk, value);

        chunk.push(line, OpCode::LoadConst(constant));
    }
}
