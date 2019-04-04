pub mod state;
use state::State;

extern "C" {
    fn cpu_new(id: u32);
    fn cpu_delete(id: u32);

    fn cpu_loop(id: u32);
    fn cpu_get_pc(id: u32) -> u64;
    fn cpu_get_reg64(id: u32, reg: u32) -> u64;
    fn cpu_set_reg64(id: u32, reg: u32, val: u64);

    fn cpu_set_state(id: u32);
}

pub enum GpReg {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rbx = 3,
    Rsp = 4,
    Rbp = 5,
    Rsi = 6,
    Rdi = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,

}

pub struct Cpu {
    handle: u32
}

impl Cpu {
    pub fn new(id: u32) -> Self {
        unsafe { cpu_new(id) };

        Self { handle: id }
    }

    pub fn from(id: u32, s: State) -> Self {
        let c = new(id);
        c.set_state(s);

        c
    }

    pub fn id(&self) -> u32 {
        self.handle
    }

    pub fn delete(&mut self) {
        unsafe { cpu_delete(self.handle)  };
    }

    pub fn run(&self) {
        unsafe { cpu_loop(self.handle) };
    }

    pub fn pc(&self) -> u64 {
        unsafe { cpu_get_pc(self.handle) }
    }

    pub fn reg(&self, r: Reg) -> u64 {
        unsafe { cpu_get_reg64(self.handle, r as u32) }
    }

    pub fn set_reg(&self, r: Reg, v: u64) {
        unsafe { cpu_set_reg64(self.handle, r as u32, v) }
    }

    pub fn state(&self) -> State {

    }

    pub fn set_state(&self, s: State) {
        unsafe { cpu_set_state(self.id) }
    }
}
