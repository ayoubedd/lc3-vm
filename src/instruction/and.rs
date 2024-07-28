use crate::VM;

pub fn and(instr: u16, vm: &mut VM) {
    let mode: bool = (instr & 1 << 5) != 0; // mode: 1 -> immediate, 0 -> register

    let dr = (instr & (7 << 9)) >> 9;
    let sr1 = (instr & (7 << 6)) >> 6;
    let r_value;

    match mode {
        true => { // imediate mode
            r_value = instr & 2_u16.pow(5) - 1;
        }
        false => { // register mode
            let sr2 = instr & 7;
            r_value = vm.registers.get(sr2);
        }
    }
    
    let result = vm.registers.get(sr1) & r_value;

    vm.registers.set(dr, result);
}
