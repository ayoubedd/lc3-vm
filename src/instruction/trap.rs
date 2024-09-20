use std::{io::Read, process::exit};

use crate::vm::VM;

pub fn trap(instr: u16, vm: &mut VM) {
    let trapvect = instr & (2_u16.pow(9) - 1);

    vm.registers.r7 = vm.registers.pc + 1;

    match trapvect {
        0x20 => {
            // GETC
            let mut stdin = std::io::stdin();
            let mut buff = [0_u8; 1];
            stdin.read_exact(&mut buff).unwrap();
            vm.registers.r0 = buff[0] as u16;
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
                let value = vm.memory.read(addr);
                if value == 0x0 {
                    break;
                }
                print!("{}", char::from(value as u8));
                addr += 1;
            }
        }
        0x23 => {
            // IN
        }
        0x24 => {
            // PUTSP
        }
        0x25 => {
            // HALT
            exit(1);
        }
        _ => {
            // unimplemented
            exit(1);
        }
    }
}
