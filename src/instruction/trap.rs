use std::process::exit;

use crate::vm::VM;

pub fn trap(instr: u16, vm: &mut VM) {
    let trapvect = instr & (2_u16.pow(9) - 1);

    vm.registers.r7 = vm.registers.pc + 1;

    match trapvect {
        0x20 => {
            // GETC
            // println!("getc");
        }
        0x21 => {
            // OUT

            let value: u8 = (vm.registers.r0 & 2u16.pow(9) - 1) as u8;
            print!("{}", char::from(value));
        }
        0x22 => {
            // PUTS

            let mut addr = vm.registers.r0;
            loop {
                let value = vm.memory.mem[addr as usize];
                if value == 0x0 {
                    break;
                }
                print!("{}", char::from(value as u8));
                addr += 1;
            }
        }
        0x23 => {
            // IN
            // println!("in");
        }
        0x24 => {
            // PUTSP
            // println!("putsp");
        }
        0x25 => {
            // HALT
            println!("\n\nHALT!");
            exit(1);
        }
        _ => {
            // unimplemented
            println!("UNIMPLEMENTED TRAP!");
            exit(1);
        }
    }
}
