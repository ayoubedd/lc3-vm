use crate::vm::VM;

pub fn add(instr: u16, vm: &mut VM) {
    println!("add");
    let mode: bool = (instr & 1 << 5) != 0; // mode: 1 -> immediate, 0 -> register

    let dr = (instr & (7 << 9)) >> 9;
    let sr1 = (instr & (7 << 6)) >> 6;
    let value;

    match mode {
        true => { // imediate mode
            value = instr & 2_u16.pow(5) - 1;
        }
        false => { // register mode
            let sr2 = instr & 7;
            value = vm.registers.get(sr2);
        }
    }

    let result = vm.registers.get(sr1) + value;

    vm.registers.set(dr, result);
}
