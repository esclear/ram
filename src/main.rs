#[macro_use]
extern crate nom;

mod instructions;
mod parser;

use instructions::Memory;

fn main() {
    let (_, foo) = parser::instruction("R[1]:=42+0;").unwrap();
    let (_, bar) = parser::instruction("R[1]:=R[1]/2;").unwrap();
    let (_, baz) = parser::instruction("R[R[1]]:=1337+0;").unwrap();

    let mut mem = Memory::new();

    foo.execute(&mut mem);
    println!("Value: {}", mem.get(1));

    bar.execute(&mut mem);
    println!("Value: {}", mem.get(1));

    baz.execute(&mut mem);
    println!("Value: {}", mem.get(1));
    println!("Value: {}", mem.get(21));

}
