use crate::instructions::Instruction;
use std::collections::HashMap;

pub fn print_program(program: &Vec<Instruction>) {
    for (index, instruction) in program.iter().enumerate() {
        println!("{:3}: {}", index + 1, instruction);
    }
}

pub fn print_memory(memory: &HashMap<u32, i32>) {
    let mut keys: Vec<(&u32, &i32)> = memory.iter().collect();
    keys.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    for (address, value) in keys {
        println!("{:3}: {}", address, value);
    }
}