pub const MEMORY_SIZE: usize = 1 << 4;

#[derive(Debug)]
pub struct Memory {
    pub mem: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_SIZE],
        }
    }

    pub fn zero(&mut self) {
        for ele in self.mem.as_mut() {
            *ele = 0;
        }
    }

    pub fn read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }
}
