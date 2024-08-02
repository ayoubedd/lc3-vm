use crate::VM;

pub fn br(instr: u16, vm: &mut VM) {
    let negative = (instr & (1 << 11)) >> 11 == 1;
    let zero = (instr & (1 << 10)) >> 10 == 1;
    let positive = (instr & (1 << 9)) >> 9 == 1;

    dbg!(negative, negative, positive);

    let offset = instr & 2_u16.pow(10) - 1;

    if negative && vm.registers.psr_is_negative()
        || positive && vm.registers.psr_is_positive()
        || zero && vm.registers.psr_is_zero()
    {
        vm.registers.pc += offset;
        vm.branched = true;
    } else {
        vm.branched = false;
    }
}
