use crate::vm::VM;

pub fn ldi(instr: u16, vm: &mut VM) {
    let dst_r = (instr & 7 << 9) >> 9;
    let pc_offset = instr & (2_u16.pow(9) - 1);

    let pc = vm.registers.pc + 1;
    let addr_addr = (pc + pc_offset) as usize;
    let addr = vm.memory.mem[addr_addr];

    let value = vm.memory.mem[addr as usize];

    vm.registers.set(dst_r, vm.memory.mem[addr as usize]);
    vm.setcc(value);
}
