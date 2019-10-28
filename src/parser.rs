
use nom::{
    IResult,
    sequence::delimited,
    character::complete::{char, digit1},
    bytes::complete::{is_not, tag},
    error::ErrorKind
};

use crate::instructions::*;

pub fn arithm_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, target_register) = register(input)?;
    
    let (input, _) = tag(":=")(input)?;

    let (input, left_operand)  = operand(input).map(|(s, o)| (s, Box::new(o)))?;
    let (input, operator)      = operator(input)?;
    let (input, right_operand) = operand(input).map(|(s, o)| (s, Box::new(o)))?;
    
    let (input, _) = tag(";")(input)?;

    Ok((input, Instruction::Arithmetic { target_register, left_operand, operator, right_operand }))
}

pub fn conditional_jump(input: &str) -> IResult<&str, Instruction> {
    // If keyword
    let (input, _) = tag("if")(input)?;

    // Condition
    let (input, left_operand)  = operand(input)?;
    let (input, relation)      = relation(input)?;
    let (input, right_operand) = operand(input)?;
    
    // Keyword goto
    let (input, _) = tag("goto")(input)?;
    
    // Address
    let (input, address) = uint(input)?;
    
    let (input, _) = tag(";")(input)?;
    
    let cond_jump = Instruction::ConditionalJump {
        left_operand: Box::new(left_operand),
        relation,
        right_operand: Box::new(right_operand),
        target: address
    };
    
    Ok((input, cond_jump))
}


pub fn register(input: &str) -> IResult<&str, Register> {
    let (input, _)       = tag("R")(input)?;
    let (input, address) = delimited(char('['), address, char(']'))(input)?;

    Ok((input, Register { address }))
}

named!(address<&str, Address>, alt!(
    uint     => { |addr| Address::Raw(addr) } |
    register => { |reg|  Address::Register(Box::new(reg)) }
));

named!(uint<&str, u32>, map_res!(digit1, |s: &str| s.parse::<u32>()));
named!(int<&str, i32>, map_res!(pair!(opt!(tag!("-")), uint),
    |(p, abs): (Option<&str>, u32)|
        Result::Ok::<i32, (&str, ErrorKind)>(if p.is_some() { -1 } else { 1 } * (abs as i32))
));

named!(operator<&str, Operator>, alt!(
    tag!("+") => { |_| Operator::Plus  } |
    tag!("-") => { |_| Operator::Minus } |
    tag!("*") => { |_| Operator::Times } |
    tag!("/") => { |_| Operator::Divides }
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