use lux_core::data::{Chunk, Instruction};
use lux_core::vm::Vm;

fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();
    chunk.write(Instruction::Constant as u8, 123);
    let constant = chunk.add_constant(1.2) as u8;
    chunk.write(constant, 123);
    chunk.write(Instruction::Constant as u8, 123);
    let constant = chunk.add_constant(3.4) as u8;
    chunk.write(constant, 123);
    chunk.write(Instruction::Add as u8, 123);
    chunk.write(Instruction::Constant as u8, 123);
    let constant = chunk.add_constant(5.6) as u8;
    chunk.write(constant, 123);
    chunk.write(Instruction::Divide as u8, 123);
    chunk.write(Instruction::Negate as u8, 123);
    chunk.write(Instruction::Return as u8, 123);
    chunk.dissassemble("test chunk");
    vm.interpret(chunk).expect("vm interpret");
}
