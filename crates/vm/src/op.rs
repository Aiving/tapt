#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)] // !!!! BYTE code !!!!
pub enum OpCode {
    Pop,
    LoadConst(usize),
    Equal,
    Greater,
    Less,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Return,
    SetProperty(usize),
    GetProperty(usize),
    GetLocal(isize, usize),
    SetLocal(Option<usize>, usize),
    Jump(isize),
    JumpIfFalse(usize),
    Call(usize),
    Copy,
    PushFrame,
    PopFrame,
    CreateInstance,
    Halt, // You should halt yourself NOW!
}

impl OpCode {
    #[must_use]
    pub const fn size(&self) -> usize {
        match self {
            Self::LoadConst(_) | Self::GetLocal(..) | Self::SetLocal(..) => 2,
            _ => 1,
        }
    }
}
