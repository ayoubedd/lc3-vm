use crate::vm::VM;

pub fn lea(instr: u16, vm: &mut VM) {
    let dst_r = (instr & 7 << 9) >> 9;
    let pc_offset = instr & (2_u16.pow(9) - 1);

    let pc = vm.registers.pc + 1;
    let value = pc + pc_offset;

    vm.registers.set(dst_r, value);
    vm.setcc(value);
}
