#[macro_use]
extern crate nom;

extern crate argparse;

mod instructions;
mod parser;
mod machine;
mod utils;

use std::path::Path;

use argparse::{ArgumentParser, Store, StoreTrue, Print, StoreOption};

use machine::Machine;
use std::fs::File;
use std::io::Read;

fn main() {
    // Build s parser and
    let mut options = Options::new();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Parser and interpreter for a simple model of computation.");

        parser.add_option(&["-V", "--version"],
                      Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");

        parser.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Print verbose output");
        parser.refer(&mut options.print_program)
            .add_option(&["-P", "--print-program"], StoreTrue, "Print the program as it was parsed");
        parser.refer(&mut options.print_memory)
            .add_option(&["-M", "--print-memory"], StoreTrue, "Print the memory as it was parsed");

        parser.refer(&mut options.maximum_steps)
            .add_option(&["-n", "--maximum-steps"], StoreOption, "Maximum number of executed instructions");

        parser.refer(&mut options.program)
            .add_argument("program", Store, "File to load the program from")
            .required();
        parser.refer(&mut options.memory)
            .add_argument("memory", StoreOption, "File to load the memory contents from");

        parser.parse_args_or_exit();
    }

    // Open the program file
    let program_file = File::open(&options.program);
    if program_file.is_err() {
        println!("Could not open the file at {}: {}", options.program, program_file.unwrap_err());
        ::std::process::exit(1);
    }

    // Read the program
    let mut program_file = program_file.unwrap();
    let mut program_contents = String::new();
    program_file.read_to_string(&mut program_contents).expect("Could not read from provided program file.");

    // Parse the program
    let program = parser::program(&program_contents).expect("Encountered an error while parsing the program!");
    if options.print_program {
        utils::print_program(&program);
    }

    let mut machine = Machine::new();
    machine.load_program(program);
    if let Some(steps) = options.maximum_steps {
        machine.set_remaining_steps(steps);
    }

    if options.verbose {
        println!("Starting execution.");
    }
    machine.run();
    if options.verbose {
        println!("Finished execution.");
    }

    // Print the memory after execution
    utils::print_memory(machine.get_memory());
}

#[derive(Debug)]
struct Options {
    program: String,
    verbose: bool,
    memory: Option<String>,
    print_program: bool,
    print_memory: bool,
    maximum_steps: Option<u32>
}

impl Options {
    fn new() -> Options {
        Options {
            program: "".to_string(),
            verbose: false,
            memory: None,
            print_program: false,
            print_memory: false,
            maximum_steps: None
        }
    }
}