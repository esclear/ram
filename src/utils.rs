use crate::instructions::Instruction;
use std::collections::HashMap;

use crate::parser::memory_cell;

pub fn print_program(program: &Vec<Instruction>) {
    for (index, instruction) in program.iter().enumerate() {
        println!("\t{:3}: {}", index + 1, instruction);
    }
}

pub fn print_memory(memory: &HashMap<u32, i32>) {
    let mut keys: Vec<(&u32, &i32)> = memory.iter().collect();
    keys.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

    for (address, value) in keys {
        println!("\t{:3}: {}", address, value);
    }
}

pub fn parse_memory(memory: &str) -> HashMap<u32, i32> {
    let mut mem: HashMap<u32, i32> = HashMap::new();
    for (line, cell) in memory.lines().enumerate() {
        if let Ok((_, (k, v))) = memory_cell(cell) {
            mem.insert(k, v);
        } else {
            println!("{:?}", memory_cell(cell));
            panic!("Can't parse the memory contents at line {}", line + 1);
        }
    }
    mem
}