use crate::VM;

pub fn st(instr: u16, vm: &mut VM) {
    let sr = (instr & (7 << 9)) >> 9;
    let offset = instr & 2_u16.pow(9) - 1;
    let target_address = (vm.registers.pc + offset) as usize;

    vm.memory.mem[target_address] = vm.registers.get(sr);
}
