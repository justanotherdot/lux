/// Errors pertaining to instructions and opcodes.
#[derive(Debug, PartialEq)]
pub enum InstructionError {
    UnknownOpcode(u8),
}
