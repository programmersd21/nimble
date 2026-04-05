use super::bytecode::Reg;

pub struct RegisterAllocator {
    next: u8,
    max: u8,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self { next: 0, max: 0 }
    }

    pub fn alloc(&mut self) -> Reg {
        let r = Reg(self.next);
        self.next += 1;
        if self.next > self.max {
            self.max = self.next;
        }
        r
    }

    pub fn free(&mut self) {
        if self.next > 0 {
            self.next -= 1;
        }
    }

    pub fn reset(&mut self, next: u8) {
        self.next = next;
    }

    pub fn current(&self) -> u8 {
        self.next
    }

    pub fn max_registers(&self) -> u8 {
        self.max
    }
}
