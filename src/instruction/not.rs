use crate::vm::VM;

pub fn not(instr: u16, vm: &mut VM) {
    let dst_r = (instr & 7 << 9) >> 9;
    let src_r = (instr & 7 << 6) >> 6;

    let value = vm.registers.get(src_r);
    vm.registers.set(dst_r, !value);
    vm.setcc(value);
}
