/// An individual chunk of bytecode.
///
/// This houses the individual instructions and their supplied arguments.
#[derive(Debug, PartialEq)]
pub struct Chunk {
    // TODO: to TinyVec
    pub code: Vec<u8>,
    // TODO: to TinyVec, max 255 I believe?
    pub constants: Vec<Value>,
}

/// All supported instructions. The associated number for an `Instruction` is known as its
/// "opcode".
///
/// These are pinned to exact opcodes to make the mapping clear, as well
/// as assuring compatibility when earlier opcodes are removed or repurposed.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Return = 0,
    Constant = 1,
}

// TODO: This ought to be an enum, unless constants are
// only ever one type.
pub type Value = f64;
