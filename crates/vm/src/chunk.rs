use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::{op::OpCode, value::Value};

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub struct Chunk {
    pub code: Vec<(usize, OpCode)>,
    pub constants: Vec<Value>,
}

impl Chunk {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    #[must_use]
    pub fn constants(&self) -> usize {
        self.constants.len()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn push(&mut self, line: usize, byte: OpCode) {
        self.code.push((line, byte));
    }

    pub fn push_const(&mut self, value: Value) {
        self.constants.push(value);
    }

    #[must_use]
    pub fn get_const(&self, index: usize) -> &Value {
        &self.constants[index]
    }

    #[must_use]
    pub fn get_const_cloned(&self, index: usize) -> Value {
        self.constants[index].clone()
    }
}

struct CodePosition<'a> {
    offset: usize,
    code: &'a (usize, OpCode),
}

impl fmt::Debug for CodePosition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ offset: {:04}, line: {}, value: {:?} }}",
            self.offset, self.code.0, self.code.1
        )
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunk")
            .field("constants", &self.constants)
            .field(
                "instructions",
                &self
                    .code
                    .iter()
                    .fold(Vec::<CodePosition>::new(), |mut instructions, code| {
                        let last_offset = instructions
                            .last()
                            .map(|instruction| instruction.offset)
                            .unwrap_or_default();

                        instructions.push(CodePosition {
                            offset: last_offset + code.1.size(),
                            code,
                        });

                        instructions
                    }),
            )
            .finish()
    }
}

impl IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.code.index_mut(index)
    }
}

impl Index<usize> for Chunk {
    type Output = (usize, OpCode);

    fn index(&self, index: usize) -> &Self::Output {
        self.code.index(index)
    }
}
