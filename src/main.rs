mod instruction;
mod memory;
mod registers;
mod utilities;
mod vm;
use std::env::args;

use vm::VM;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        panic!("pass path to lc-3 image");
    }

    let mut vm = VM::new();

    vm.load_program(&args[1]);
    vm.run();
}
