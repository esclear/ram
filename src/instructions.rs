use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Arithmetic {
        target_register: Register,
        left_operand:    Box<Operand>,
        operator:        Operator,
        right_operand:   Box<Operand>,
    },
    ConditionalJump {
        left_operand:  Box<Operand>,
        relation:      Relation,
        right_operand: Box<Operand>,
        target:        u32
    }
}

impl Instruction {
    fn condition_holds(&self, memory: &Memory) -> bool {
        match self {
            Instruction::ConditionalJump {left_operand, relation, right_operand, ..} => relation.holds(left_operand.evaluate(memory), right_operand.evaluate(memory)),
            _ => false
        }
    }
    
    fn evaluate_aritmetic_value(&self, memory: &Memory) -> i32 {
        match self {
            Instruction::Arithmetic {left_operand, operator, right_operand, ..} => operator.apply(left_operand.evaluate(memory), right_operand.evaluate(memory)),
            _ => 0
        }
    }

    pub fn execute(&self, memory: &mut Memory) -> Option<u32> {
        match self {
            Instruction::ConditionalJump { target, .. } => {
                if self.condition_holds(memory) {
                    Some(*target)
                } else {
                    None
                }
            },
            Instruction::Arithmetic { target_register, .. } => {
                memory.set(target_register.resolve_address(memory), self.evaluate_aritmetic_value(memory));
                None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divides
}

impl Operator {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match &self {
            Operator::Plus    => a + b,
            Operator::Minus   => a - b,
            Operator::Times   => a * b,
            Operator::Divides => a / b
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Relation {
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq
}

impl Relation {
    fn holds(&self, a: i32, b: i32) -> bool {
        match self {
            Relation::Lt  => a <  b,
            Relation::Leq => a <= b,
            Relation::Gt  => a >  b,
            Relation::Geq => a >= b,
            Relation::Eq  => a == b,
            Relation::Neq => a != b
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
    Address(u32),
    Register(Box<Register>)
}

impl Register {
    fn resolve_address(&self, memory: &Memory) -> u32 {
        match &self {
            Register::Address(addr) => *addr,
            Register::Register(reg) => memory.get(reg.resolve_address(memory)) as u32
        }
    }
}

impl Evaluable for Register {
    fn evaluate(&self, memory: &Memory) -> i32 {
        memory.get(self.resolve_address(memory))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Integer(i32),
    Data(Register)
}

impl Evaluable for Operand {
    fn evaluate(&self, memory: &Memory) -> i32 {
        match self {
            Operand::Data(register) => register.evaluate(memory),
            Operand::Integer(value) => *value
        }
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

pub trait Evaluable {
    fn evaluate(&self, memory: &Memory) -> i32;
}