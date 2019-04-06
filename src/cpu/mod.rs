use crate::syncunsafecell::SyncUnsafeCell;

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StopReason {
    None,
    Timeout,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RunState {
    Go,
    Stop(StopReason),
}

// this needs to be global visible so callbacks can tell us to stop emulation
// despite the intuitive race condition where different cpus modify their state
// concurrently, bochs serializes cpu execution even with multiple cpus, so
// this should be ok. I hope.
#[ctor]
pub static CPU_RUN_STATES: SyncUnsafeCell<Vec<RunState>> = {
    SyncUnsafeCell::new(vec![RunState::Stop(StopReason::None); 255])
};

pub unsafe fn run_state(id: u32) -> RunState {
    (*(CPU_RUN_STATES.0.get()))[id as usize]
}

pub unsafe fn set_run_state(id: u32, rs: RunState) {
    (*(CPU_RUN_STATES.0.get()))[id as usize] = rs;
}

pub struct Cpu {
    handle: u32
}

impl Cpu {
    pub unsafe fn new(id: u32) -> Self {
        cpu_new(id);

        Self { handle: id }
    }

    pub unsafe fn new_with_state(id: u32, s: State) -> Self {
        let c = Self::new(id);
        c.set_state(s);

        c
    }

    pub fn from(id: u32) -> Self {
        Self { handle: id }
    }

    pub unsafe fn from_with_state(id: u32, s: State) -> Self {
        let c = Self::from(id);
        c.set_state(s);

        c
    }

    pub fn id(&self) -> u32 {
        self.handle
    }

    pub unsafe fn delete(&mut self) {
        cpu_delete(self.handle);
    }

    pub unsafe fn run(&self) -> RunState {
        set_run_state(self.handle, RunState::Go);

        loop {
            match run_state(self.handle) {
                RunState::Stop(_) => break,
                RunState::Go => cpu_loop(self.handle),
            }
        }

        run_state(self.handle)
    }

    pub unsafe fn pc(&self) -> u64 {
        cpu_get_pc(self.handle)
    }

    pub unsafe fn reg(&self, r: GpReg) -> u64 {
        cpu_get_reg64(self.handle, r as u32)
    }

    pub unsafe fn set_reg(&self, r: GpReg, v: u64) {
        cpu_set_reg64(self.handle, r as u32, v)
    }

    /*
    pub fn state(&self) -> State {

    }
    */

    pub unsafe fn set_state(&self, s: State) {
        cpu_set_state(self.handle)
    }

    pub unsafe fn run_state(&self) -> RunState {
        run_state(self.handle)
    }

    pub unsafe fn set_run_state(&self, rs: RunState) {
        set_run_state(self.handle, rs)
    }
}
