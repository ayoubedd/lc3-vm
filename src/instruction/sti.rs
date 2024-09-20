use crate::VM;

pub fn sti(instr: u16, vm: &mut VM) {
    let src_r = (instr & 7 << 9) >> 9;
    let pc_offset = instr & (2_u16.pow(10) - 1);

    let pc = vm.registers.pc + 1;
    let dst_addr = pc + pc_offset;

    vm.memory.write(dst_addr, vm.registers.get(src_r));
}
