use crate::vm::VM;

pub fn jsr(instr: u16, vm: &mut VM) {
    let mode: bool = (instr & 1 << 11) != 0; // mode: 1 -> immediate, 0 -> register

    vm.registers.pc += 1;
    vm.registers.r7 = vm.registers.pc;

    let mut addr;

    match mode {
        true => {
            // immediate mode
            addr = instr & 2_u16.pow(10) - 1;
            addr += vm.registers.pc;
        }
        false => {
            // register
            let rg = (instr & 7 << 6) >> 6;
            addr = vm.registers.get(rg);
        }
    }

    vm.registers.pc = addr;
}
