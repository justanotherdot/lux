/// An individual chunk of bytecode.
///
/// This houses the individual instructions and their supplied arguments.
#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub code: Vec<u8>,
}

/// All supported instructions. The associated number for an `Instruction` is known as its
/// "opcode".
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Return = 0,
}
