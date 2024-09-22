use crate::{utilities::sign_extend, vm::VM};

pub fn add(instr: u16, vm: &mut VM) {
    let mode: bool = (instr & 1 << 5) != 0; // mode: 1 -> immediate, 0 -> register

    let dr = (instr & (7 << 9)) >> 9;
    let sr1 = (instr & (7 << 6)) >> 6;
    let value;

    match mode {
        true => {
            // imediate mode
            value = instr & 2_u16.pow(5) - 1;
        }
        false => {
            // register mode
            let sr2 = instr & 7;
            value = vm.registers.get(sr2);
        }
    }

    let sign_extended = sign_extend(value, 16 - 5);
    let result = vm.registers.get(sr1) + sign_extended;

    vm.registers.set(dr, result);
    vm.setcc(result);
}
