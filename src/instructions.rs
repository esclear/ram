use super::machine::Memory;
use std::fmt;

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
    
    fn evaluate_arithmetic_value(&self, memory: &Memory) -> i32 {
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
                memory.set(target_register.resolve_address(memory), self.evaluate_arithmetic_value(memory));
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
    Divide
}

impl Operator {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match &self {
            Operator::Plus   => a + b,
            Operator::Minus  => a - b,
            Operator::Times  => a * b,
            Operator::Divide => a / b
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

pub trait Evaluable {
    fn evaluate(&self, memory: &Memory) -> i32;
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instruction::Arithmetic {target_register, left_operand, operator, right_operand}
                => write!(f, "{} := {} {} {};", target_register, left_operand, operator, right_operand),
            Instruction::ConditionalJump {left_operand, relation, right_operand, target}
                => write!(f, "if {} {} {} goto {};", left_operand, relation, right_operand, target)
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Operand::Integer(value) => write!(f, "{}", value),
            Operand::Data(register) => register.fmt(f)
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Register::Address(address) => write!(f, "R[{}]", address),
            Register::Register(register) => write!(f, "R[{}]", register)
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Times => write!(f, "*"),
            Operator::Divide => write!(f, "/")
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Relation::Lt  => write!(f, "<"),
            Relation::Leq => write!(f, "<="),
            Relation::Gt  => write!(f, ">"),
            Relation::Geq => write!(f, "<"),
            Relation::Eq  => write!(f, "=="),
            Relation::Neq => write!(f, "!="),
        }
    }
}