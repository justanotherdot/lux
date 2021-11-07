use lux_core::data::{Chunk, Instruction};

// NOTE: just a demo for now.
fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Instruction::Return as u8, 1);
    chunk.dissassemble("test chunk");
}
