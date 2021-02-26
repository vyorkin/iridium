use crate::vm::VM;
use std::{
    io::{self, Write},
    num::ParseIntError,
};

pub struct REPL {
    vm: VM,
    command_buffer: Vec<String>,
}

impl REPL {
    /// Creates a new REPL.
    pub fn new(vm: VM) -> REPL {
        REPL {
            vm,
            command_buffer: vec![],
        }
    }

    /// Runs loop similar to the VM execution loop, but the
    /// instructions are taken from the user directly at the
    /// terminal and not from pre-compiled bytecode.
    pub fn run(&mut self) -> ! {
        println!("Welcome to Irridium");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            print!(">>> ");
            stdout.flush().expect("Unable to flush STDOUT");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read from STDIN");
            let cmd = buffer.trim();
            self.command_buffer.push(cmd.to_string());
            match cmd {
                ".program" => {
                    for instr in &self.vm.program {
                        println!("{}", instr);
                    }
                }
                ".registers" => {
                    println!("{:#?}", self.vm.registers);
                }
                ".history" => {
                    for cmd in &self.command_buffer {
                        println!("{}", cmd)
                    }
                }
                ".quit" => {
                    println!("Bye");
                    std::process::exit(0);
                }
                s => {
                    match self.parse_hex(s) {
                        Ok(bytes) => {
                            self.vm.add_bytes(bytes);
                        }
                        Err(err) => {
                            println!("Unable to decode hex string: {}. Please enter 4 groups of 2 hex characters", err);
                        }
                    };
                    self.vm.step();
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a
    /// leading `0x` and returns a `Vec<u8>`.
    /// Example for a LOAD command: 00 01 03 E8.
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_str in split {
            let byte = u8::from_str_radix(&hex_str, 16)?;
            results.push(byte);
        }
        Ok(results)
    }
}
