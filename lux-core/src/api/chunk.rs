/// An individual chunk of bytecode.
///
/// This houses the individual instructions and their supplied arguments.
#[derive(Debug, PartialEq)]
pub struct Chunk {
    code: Vec<u8>,
}

/// All supported instructions. The associated number for an `Instruction` is known as its
/// "opcode".
pub enum Instruction {
    Return = 0,
}

pub enum InstructionError {
    UnknownOpcode(u8),
}

// TODO: Debug + Display trait impls for Instruction, possibly.
impl Instruction {
    fn name(&self) -> String {
        match self {
            Instruction::Return => String::from("OP_RETURN"),
        }
    }

    fn from_byte(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            0 => Ok(Instruction::Return),
            1..=u8::MAX => Err(InstructionError::UnknownOpcode(byte)),
        }
    }
}

fn unknown_instruction(opcode: u8, offset: usize) -> usize {
    println!("Unknown opcode {}", opcode);
    offset + 1
}

fn simple_instruction(instruction: &Instruction, offset: usize) -> usize {
    println!("{}", instruction.name());
    offset + 1
}

fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    // FIXME: an offset may exceed the bounds of the stored code.
    // hence it would be an error that should be known when
    // disassembling. This error should not be in the form of a panic.
    let opcode = chunk.code[offset];
    let instruction = Instruction::from_byte(opcode);
    match instruction {
        Ok(instruction @ Instruction::Return) => simple_instruction(&instruction, offset),
        Err(InstructionError::UnknownOpcode(opcode)) => unknown_instruction(opcode, offset),
    }
}

impl Chunk {
    /// Create a new bytecode chunk.
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn dissassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = dissassemble_instruction(self, offset);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chunks_store_code() {
        let mut chunk = Chunk::new();
        chunk.write(Instruction::Return as u8);
        assert_eq!(chunk, Chunk { code: vec![0] });
    }
}