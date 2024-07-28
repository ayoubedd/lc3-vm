use crate::vm::VM;

pub fn jmp(instr: u16, vm: &mut VM) {
    println!("jmp");

    let base_register = (instr & (7 << 6)) >> 6;
    vm.registers.pc = vm.registers.get(base_register);
}
