pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

use vm::VM;
use repl::REPL;

fn main() {
    let vm = VM::new();
    let mut repl = REPL::new(vm);
    repl.run();
}
