use crate::vm::VM;

pub fn jmp(instr: u16, vm: &mut VM) {
    let base_register = (instr & (7 << 6)) >> 6;

    if base_register == 0b111 {
        println!("ret");
        vm.registers.pc = vm.registers.r7;
    } else {
        println!("jmp");
        vm.registers.pc = vm.registers.get(base_register);
    }
}
