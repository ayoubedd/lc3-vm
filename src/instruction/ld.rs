use crate::VM;

pub fn ld(instr: u16, vm: &mut VM) {
    println!("ld");
    let dr = (instr & (7 << 9)) >> 9;
    let offset = instr & 2_u16.pow(9) - 1;

    let target = vm.registers.pc + offset;
    let value = vm.memory.mem[target as usize];

    vm.registers.set(dr, value);
}
