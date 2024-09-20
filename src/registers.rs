#[derive(Debug)]
pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub psr: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
            // cpu start as supervisor mode by default
            psr: 0, // PSR[2] is N, PSR[1] is Z, and PSR[0] is P.
        }
    }

    pub fn set(&mut self, rg: u16, value: u16) {
        match rg {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            _ => panic!("write: invalid register"),
        }
    }

    pub fn get(&self, rg: u16) -> u16 {
        match rg {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            value => {
                dbg!(value);
                panic!("read: invalid register")
            }
        }
    }

    pub fn psr_is_negative(&self) -> bool {
        if self.psr & (1 << 2) == 1 {
            return true;
        }

        false
    }

    pub fn psr_is_zero(&self) -> bool {
        if self.psr & (1 << 1) == 1 {
            return true;
        }

        false
    }

    pub fn psr_is_positive(&self) -> bool {
        if self.psr & 1 == 1 {
            return true;
        }

        false
    }
}
