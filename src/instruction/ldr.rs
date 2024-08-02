use crate::VM;

pub fn ldr(instr: u16, vm: &mut VM) {
    let dst_r = (instr & 7 << 9) >> 9;
    let base_r = (instr & 7 << 6) >> 6;
    let offset = instr & (2_u16.pow(6) - 1);

    let dst_addr = vm.registers.get(base_r) + offset;
    let value = vm.memory.mem[dst_addr as usize];

    vm.registers.set(dst_r, value);
    vm.setcc(value);
}
