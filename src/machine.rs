use std::collections::HashMap;

use crate::instructions::Instruction;
use crate::parser;
use crate::utils;

pub struct Machine {
    program_counter: u32,
    program: Vec<Instruction>,
    memory: Memory,
    steps_remaining: Option<u32>,
    debug_output: bool
}

impl Machine {
    pub fn new() -> Machine {
        Machine { program_counter: 0, program: Vec::new(), memory: Memory::new(), steps_remaining: None, debug_output: false }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    pub fn step(&mut self) -> bool {
        if let Some(instruction) = self.program.get(self.program_counter as usize) {
            if self.debug_output {
                println!("pc @ {:4} : {}", self.program_counter, instruction);
                println!("Memory before instruction execution:");
                utils::print_memory(self.get_memory());
            }

            match instruction.execute(&mut self.memory) {
                Some(instruction) => self.goto(instruction),
                None => self.program_counter += 1
            }
        }

        self.program_running()
    }

    pub fn program_running(&self) -> bool {
        self.program_counter < (self.program.len() as u32)
    }

    pub fn run(&mut self) {
        while self.program_running() {
            self.step();

            if let Some(cnt) = self.steps_remaining {
                self.steps_remaining = Some(cnt - 1);
                // If we did execute the last step just now (remaining is now 0), stop the execution.
                if cnt == 1 {
                    println!("Terminated execution because the maximum number of instructions was reached!");
                    break;
                }
            }
        }
    }

    fn goto(&mut self, instruction: u32) {
        self.program_counter = instruction - 1;
    }

    pub fn set_memory(&mut self, mem: &HashMap<u32, i32>) {
        self.memory.memory.clone_from(mem);
    }

    pub fn get_memory(&self) -> &HashMap<u32, i32> {
        &self.memory.memory
    }

    pub fn set_remaining_steps(&mut self, remaining_steps: u32) {
        self.steps_remaining = Some(remaining_steps);
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug_output = debug;
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