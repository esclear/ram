use nom::{
    character::complete::{char, digit1}
};

use crate::instructions::*;

named!(pub instruction<&str, Instruction> alt!(arithm_instruction | conditional_jump));

named!(arithm_instruction<&str, Instruction>, map!(
    tuple!(terminated!(register, tag!(":=")), operand, operator, operand),
    |(target, left, op, right)| Instruction::Arithmetic { target_register: target, left_operand: Box::new(left), operator: op, right_operand: Box::new(right) }
));

named!(conditional_jump<&str, Instruction>, map!(
    tuple!(preceded!(tag!("if"), operand), relation, terminated!(operand, tag!("goto")), uint),
    |(left, rel, right, target)| Instruction::ConditionalJump { left_operand: Box::new(left), relation: rel, right_operand: Box::new(right), target }
));

named!(register<&str, Register>, preceded!(tag!("R"), delimited!(char('['), address, char(']'))));

named!(address<&str, Register>, alt!(
    uint     => { |addr| Register::Address(addr) } |
    register => { |reg|  Register::Register(Box::new(reg)) }
));

named!(uint<&str, u32>, flat_map!(digit1, parse_to!(u32)));
named!(int<&str, i32>, flat_map!(recognize!(pair!(opt!(tag!("-")), uint)), parse_to!(i32)));

named!(operator<&str, Operator>, alt!(
    tag!("+") => { |_| Operator::Plus  } |
    tag!("-") => { |_| Operator::Minus } |
    tag!("*") => { |_| Operator::Times } |
    tag!("/") => { |_| Operator::Divides } |
    tag!("div") => { |_| Operator::Divides }
));
named!(relation<&str, Relation>, alt!(
    tag!("<")  => { |_| Relation::Lt  } |
    tag!("<=") => { |_| Relation::Leq } |
    tag!(">")  => { |_| Relation::Gt  } |
    tag!(">=") => { |_| Relation::Geq } |
    tag!("==") => { |_| Relation::Eq  } |
    tag!("!=") => { |_| Relation::Neq }
));

named!(operand<&str, Operand>, alt!(
    int      => { |val| Operand::Integer(val) } |
    register => { |reg| Operand::Data(reg) }
));