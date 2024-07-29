use crate::instruction;
use crate::{memory::Memory, registers::Registers};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::time::Duration;

pub struct VM {
    pub memory: Memory,
    pub registers: Registers,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    BR = 0,       /* branch */
    ADD,          /* add  */
    LD,           /* load */
    ST,           /* store */
    JSR,          /* jump register */
    AND,          /* bitwise and */
    LDR,          /* load register */
    STR,          /* store register */
    RTI,          /* unused */
    NOT,          /* bitwise not */
    LDI,          /* load indirect */
    STI,          /* store indirect */
    JMP,          /* jump */
    RES,          /* reserved (unused) */
    LEA,          /* load effective address */
    TRAP,         /* execute trap */
    INVLD = 1337, /* execute trap */
}

impl VM {
    pub const PC_START: u16 = 0x3000;

    pub fn new() -> VM {
        VM {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    pub fn load_program(&mut self, path: &str) {
        let file = std::fs::read(path).unwrap();
        let mut rdr = Cursor::new(&file[..]);
        let memory = &mut self.memory.mem;

        let base_address = rdr.read_u16::<BigEndian>().unwrap();
        self.registers.pc = base_address;
        let mut index: usize = self.registers.pc as usize;

        loop {
            let one = rdr.read_u16::<BigEndian>();
            match one {
                Ok(data) => {
                    memory[index] = data;
                    index += 1;
                }
                Err(_) => break,
            }
        }
    }

    pub fn run(&mut self) {
        let memory = self.memory.mem;
        let mut pc;

        loop {
            pc = self.registers.pc as usize;
            let instr = memory[pc];

            let opcode = Self::decode(instr);

            match opcode {
                Opcode::ADD => instruction::add(instr, self),
                Opcode::BR => instruction::br(instr, self),
                Opcode::LD => instruction::ld(instr, self),
                Opcode::ST => instruction::st(instr, self),
                Opcode::JSR => instruction::jsr(instr, self),
                Opcode::AND => instruction::and(instr, self),
                Opcode::LDR => instruction::ldr(instr, self),
                Opcode::STR => instruction::str(instr, self),
                Opcode::RTI => todo!(),
                Opcode::NOT => instruction::not(instr, self),
                Opcode::LDI => instruction::ldi(instr, self),
                Opcode::STI => instruction::sti(instr, self),
                Opcode::JMP => instruction::jmp(instr, self),
                Opcode::RES => todo!(),
                Opcode::LEA => instruction::lea(instr, self),
                Opcode::TRAP => instruction::trap(instr, self),
                Opcode::INVLD => todo!(),
            };

            if opcode != Opcode::JMP && opcode != Opcode::BR && opcode != Opcode::JSR {
                self.registers.pc += 1;
            }

            // dbg!(&self.registers);
            // dbg!(&self.memory);

            // println!("-------");
            std::thread::sleep(Duration::from_millis(1000));
        }
    }

    fn decode(instr: u16) -> Opcode {
        let opcode: u16 = instr >> 12;

        match opcode {
            0 => Opcode::BR,
            1 => Opcode::ADD,
            2 => Opcode::LD,
            3 => Opcode::ST,
            4 => Opcode::JSR,
            5 => Opcode::AND,
            6 => Opcode::LDR,
            7 => Opcode::STR,
            8 => Opcode::RTI,
            9 => Opcode::NOT,
            10 => Opcode::LDI,
            11 => Opcode::STI,
            12 => Opcode::JMP,
            13 => Opcode::RES,
            14 => Opcode::LEA,
            15 => Opcode::TRAP,
            _ => Opcode::INVLD,
        }
    }
}
