use std::collections::HashMap;

use crate::instructions::Instruction;
use crate::parser;

pub struct Machine {
    program_counter: u32,
    program: Vec<Instruction>,
    memory: Memory,
    steps: u32
}

impl Machine {
    pub fn new() -> Machine {
        Machine { program_counter: 0, program: Vec::new(), memory: Memory::new(), steps: 1024 }
    }

    pub fn load_program(&mut self, program: &str) -> bool {
        // As a hacky workaround for strange behaviour of nom, add a null byte to the end.
        let mut program = String::from(program);
        program.push('\0');

        let instructions = parser::program(&program);
        println!("{:?}", instructions);
        if let Ok((_, program)) = instructions {
            self.program = program;
            true
        } else {
            false
        }
    }

    pub fn step(&mut self) -> bool {
        if let Some(instruction) = self.program.get(self.program_counter as usize) {
            match instruction.execute(&mut self.memory) {
                Some(instruction) => self.goto(instruction),
                None => self.program_counter += 1
            }
        }

        println!("pc = {}", self.program_counter);

        self.program_running()
    }

    pub fn program_running(&self) -> bool {
        self.program_counter < (self.program.len() as u32)
    }

    pub fn run(&mut self) {
        while self.program_running() {
            self.step();
        }
    }
}

impl Machine {
    fn goto(&mut self, instruction: u32) {
        self.program_counter = instruction - 1;
    }

    pub fn set_memory(&mut self, mem: &HashMap<u32, i32>) {
        self.memory.memory.clone_from(mem);
    }

    pub fn get_memory(&self) -> &HashMap<u32, i32> {
        &self.memory.memory
    }
}

pub struct Memory {
    memory: HashMap<u32, i32>
}

impl Memory {
    pub fn get(&self, address: u32) -> i32 {
        *self.memory.get(&address).expect(&format!("The memory cell at {} has not been initialized!", address))
    }

    pub fn set(&mut self, address: u32, value: i32) {
        self.memory.insert(address, value);
    }

    pub fn new() -> Memory {
        Memory { memory: HashMap::new() }
    }
}