use crate::VM;

pub fn str(instr: u16, vm: &mut VM) {
    let src_r = (instr & 7 << 9) >> 9;
    let base_r = (instr & 7 << 6) >> 6;
    let offset = instr & (2_u16.pow(6) - 1);

    let dst_addr = vm.registers.get(base_r) + offset;

    vm.memory.write(dst_addr, vm.registers.get(src_r));
}
