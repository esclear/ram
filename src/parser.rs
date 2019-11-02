use nom::{
    character::complete::{char, digit1}
};

use crate::instructions::*;

pub fn program(program: &str) -> Option<Vec<Instruction>> {
    // As a hacky workaround for strange behaviour of nom, add a null byte to the end.
    let mut program = String::from(program);
    program.push('\0');

    let instructions = _program(&program);
    _program(&program).map(|(r, p)| if r.eq("\x00") { Some(p) } else { None }).unwrap_or_else(|_| None)
}

named!(_program<&str, Vec<Instruction>>, preceded!(wso, many0!(instruction)));

named!(pub instruction<&str, Instruction>, terminated!(alt!(arithm_instruction | conditional_jump), tuple!(opt!(char!(';')), wso)));

named!(arithm_instruction<&str, Instruction>, map!(
    tuple!(terminated!(register, tuple!(wso, tag!(":="), wso)), terminated!(operand, wso), terminated!(operator, wso), operand),
    |(target, left, op, right)| Instruction::Arithmetic { target_register: target, left_operand: Box::new(left), operator: op, right_operand: Box::new(right) }
));

named!(conditional_jump<&str, Instruction>, map!(
    tuple!(preceded!(pair!(tag!("if"), ws), operand), delimited!(ws, relation, ws), terminated!(operand, tuple!(ws, tag!("goto"), ws)), uint),
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
    tag!("/") => { |_| Operator::Divide } |
    tag!("div") => { |_| Operator::Divide }
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

named!(pub memory_cell<&str, (u32, i32)>, map!(tuple!(wso, uint, wso, char!(':'), wso, int), |(_,a,_,_,_,d)| (a, d)));

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\r' | '\n' => true,
        _ => false
    }
}

named!(ws<&str, &str>, take_while1!(is_whitespace));
named!(wso<&str, &str>, take_while!(is_whitespace));