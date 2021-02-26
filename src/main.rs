pub mod vm;
pub mod instruction;
pub mod repl;

use vm::VM;
use repl::REPL;

fn main() {
    let vm = VM::new();
    let mut repl = REPL::new(vm);
    repl.run();
}
