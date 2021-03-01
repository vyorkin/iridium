use crate::assembler::parsing::program;
use crate::vm::VM;
use std::io::{self, Write};

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
                    let program = match program(s) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    self.vm.add_bytes(program.to_bytes());
                    self.vm.step();
                }
            }
        }
    }
}
