#[macro_use]
extern crate nom;

mod instructions;
mod parser;

use instructions::{Memory, Instruction};

fn main() {
    let parsed = parser::arithm_instruction("R[2]:=1*R[R[1]];");
    // let parsed = parser::conditional_jump("ifR[R[1]]<R[2]goto123;");

    println!("{:?}", parsed);
    
    let instruction: Instruction = parsed.unwrap().1;
    
    let mut mem = Memory::new();
    
    mem.set(1, 2);
    mem.set(2, 42);
    
    println!("Value: {}", instruction.evaluate_aritmetic_value(&mem));
}
