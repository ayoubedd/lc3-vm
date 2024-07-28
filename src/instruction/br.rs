use crate::VM;

pub fn br(instr: u16, vm: &mut VM) {
    println!("br");

    let negative = (instr & (1 << 11)) >> 11 == 1;
    let zero = (instr & (1 << 10)) >> 10 == 1;
    let positive = (instr & (1 << 9)) >> 9 == 1;

    let offset = instr & 2_u16.pow(9) - 1;

    println!("zero: {zero}");
    println!("negative: {negative}");
    println!("positive: {positive}");
    println!("offset: {}", offset);

    if negative && vm.registers.is_negative()
        || positive && vm.registers.is_positive()
        || zero && vm.registers.is_zero()
    {
        vm.registers.pc += offset;
    }
}
