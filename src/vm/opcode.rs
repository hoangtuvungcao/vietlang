/// VietLang Bytecode Instructions (Phase 8: Bytecode VM)
/// Defines the low-level instruction set for the stack-based virtual machine.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    /// Load constant from pool: OpConstant [index_u16]
    OpConstant = 1,
    /// Pop top of stack
    OpPop = 2,
    /// Arithmetic operators
    OpAdd = 3,
    OpSub = 4,
    OpMul = 5,
    OpDiv = 6,
    OpMod = 7,
    /// Logical / Bitwise
    OpNot = 8,
    OpNeg = 9,
    /// Comparison operators
    OpEqual = 10,
    OpNotEqual = 11,
    OpGreaterThan = 12,
    OpLessThan = 13,
    OpGreaterEqual = 14,
    OpLessEqual = 15,
    /// Flow control jumps
    OpJump = 16,
    OpJumpIfFalse = 17,
    /// Globals and Locals
    OpSetGlobal = 18,
    OpGetGlobal = 19,
    OpSetLocal = 20,
    OpGetLocal = 21,
    /// Functions & Call frames
    OpCall = 22,
    OpReturn = 23,
    /// Terminate execution
    OpHalt = 24,
}

impl OpCode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(OpCode::OpConstant),
            2 => Some(OpCode::OpPop),
            3 => Some(OpCode::OpAdd),
            4 => Some(OpCode::OpSub),
            5 => Some(OpCode::OpMul),
            6 => Some(OpCode::OpDiv),
            7 => Some(OpCode::OpMod),
            8 => Some(OpCode::OpNot),
            9 => Some(OpCode::OpNeg),
            10 => Some(OpCode::OpEqual),
            11 => Some(OpCode::OpNotEqual),
            12 => Some(OpCode::OpGreaterThan),
            13 => Some(OpCode::OpLessThan),
            14 => Some(OpCode::OpGreaterEqual),
            15 => Some(OpCode::OpLessEqual),
            16 => Some(OpCode::OpJump),
            17 => Some(OpCode::OpJumpIfFalse),
            18 => Some(OpCode::OpSetGlobal),
            19 => Some(OpCode::OpGetGlobal),
            20 => Some(OpCode::OpSetLocal),
            21 => Some(OpCode::OpGetLocal),
            22 => Some(OpCode::OpCall),
            23 => Some(OpCode::OpReturn),
            24 => Some(OpCode::OpHalt),
            _ => None,
        }
    }
}
