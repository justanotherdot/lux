use lux_core::data::{Chunk, Instruction};

// NOTE: just a demo for now.
fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Instruction::Return as u8, 123);
    chunk.write(Instruction::Constant as u8, 123);
    let constant = chunk.add_constant(1.2) as u8;
    chunk.write(constant, 123);
    chunk.dissassemble("test chunk");
}
