pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

use repl::REPL;
use vm::VM;

fn main() {
    let vm = VM::new();
    let mut repl = REPL::new(vm);
    repl.run();
}
