use crate::{Address, NUM_CPUS};
use crate::syncunsafecell::SyncUnsafeCell;

pub mod state;
use state::State;

extern "C" {
    fn cpu_new(id: u32);
    fn cpu_delete(id: u32);

    fn cpu_loop(id: u32);
    fn cpu_get_pc(id: u32) -> u64;
    fn cpu_set_state(id: u32);

    fn cpu_get_reg64(id: u32, reg: u32) -> u64;
    fn cpu_set_reg64(id: u32, reg: u32, val: u64);

    fn cpu_get_eflags(id: u32) -> u32;
    fn cpu_set_eflags(id: u32, eflags: u32);

    fn cpu_get_dr(id: u32, reg: u32) -> Address;
    fn cpu_set_dr(id: u32, reg: u32, val: Address);
    fn cpu_get_dr6(id: u32) -> u32;
    fn cpu_set_dr6(id: u32, val: u32);
    fn cpu_get_dr7(id: u32) -> u32;
    fn cpu_set_dr7(id: u32, val: u32);

    fn cpu_get_cr0(id: u32) -> u32;
    fn cpu_set_cr0(id: u32, val: u32);
    fn cpu_get_cr2(id: u32) -> Address;
    fn cpu_set_cr2(id: u32, val: Address);
    fn cpu_get_cr3(id: u32) -> Address;
    fn cpu_set_cr3(id: u32, val: Address);
    fn cpu_get_cr4(id: u32) -> u32;
    fn cpu_set_cr4(id: u32, val: u32);
    fn cpu_get_efer(id: u32) -> u32;
    fn cpu_set_efer(id: u32, val: u32);
    fn cpu_get_xcr0(id: u32) -> u32;
    fn cpu_set_xcr0(id: u32, val: u32);

    fn cpu_get_kernel_gs_base(id: u32) -> u64;
    fn cpu_set_kernel_gs_base(id: u32, val: u64);
    fn cpu_get_sysenter_cs(id: u32) -> u32;
    fn cpu_set_sysenter_cs(id: u32, val: u32);
    fn cpu_get_sysenter_eip(id: u32) -> Address;
    fn cpu_set_sysenter_eip(id: u32, val: Address);
    fn cpu_get_sysenter_esp(id: u32) -> Address;
    fn cpu_set_sysenter_esp(id: u32, val: Address);
    fn cpu_get_star(id: u32) -> u64;
    fn cpu_set_star(id: u32, val: u64);
    fn cpu_get_lstar(id: u32) -> u64;
    fn cpu_set_lstar(id: u32, val: u64);
    fn cpu_get_cstar(id: u32) -> u64;
    fn cpu_set_cstar(id: u32, val: u64);
    fn cpu_get_fmask(id: u32) -> u32;
    fn cpu_set_fmask(id: u32, val: u32);
}

enum GpReg {
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

enum DrReg {
    Dr0 = 0,
    Dr1 = 1,
    Dr2 = 2,
    Dr3 = 3,
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
    SyncUnsafeCell::new(vec![RunState::Stop(StopReason::None); NUM_CPUS])
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

    pub unsafe fn run_state(&self) -> RunState {
        run_state(self.handle)
    }

    pub unsafe fn set_run_state(&self, rs: RunState) {
        set_run_state(self.handle, rs)
    }

    /*
    pub fn state(&self) -> State {

    }
    */

    pub unsafe fn set_state(&self, s: State) {
        cpu_set_state(self.handle)
    }


    //
    // regs below here
    //

    // gp regs

    pub unsafe fn rip(&self) -> u64 {
        cpu_get_pc(self.handle)
    }

    pub unsafe fn set_rip(&self, v: u64) {
        unimplemented!()
    }

    pub unsafe fn rax(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rax as _)
    }

    pub unsafe fn set_rax(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rax as _, v)
    }

    pub unsafe fn rcx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rcx as _)
    }

    pub unsafe fn set_rcx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rcx as _, v)
    }

    pub unsafe fn rdx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rdx as _)
    }

    pub unsafe fn set_rdx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rdx as _, v)
    }

    pub unsafe fn rbx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rbx as _)
    }

    pub unsafe fn set_rbx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rbx as _, v)
    }

    pub unsafe fn rsp(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rsp as _)
    }

    pub unsafe fn set_rsp(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rsp as _, v)
    }

    pub unsafe fn rbp(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rbp as _)
    }

    pub unsafe fn set_rbp(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rbp as _, v)
    }

    pub unsafe fn rsi(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rsi as _)
    }

    pub unsafe fn set_rsi(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rsi as _, v)
    }

    pub unsafe fn rdi(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::Rdi as _)
    }

    pub unsafe fn set_rdi(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::Rdi as _, v)
    }

    pub unsafe fn r8(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R8 as _)
    }

    pub unsafe fn set_r8(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R8 as _, v)
    }

    pub unsafe fn r9(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R9 as _)
    }

    pub unsafe fn set_r9(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R9 as _, v)
    }

    pub unsafe fn r10(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R10 as _)
    }

    pub unsafe fn set_r10(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R10 as _, v)
    }

    pub unsafe fn r11(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R11 as _)
    }

    pub unsafe fn set_r11(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R11 as _, v)
    }

    pub unsafe fn r12(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R12 as _)
    }

    pub unsafe fn set_r12(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R12 as _, v)
    }

    pub unsafe fn r13(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R13 as _)
    }

    pub unsafe fn set_r13(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R13 as _, v)
    }

    pub unsafe fn r14(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R14 as _)
    }

    pub unsafe fn set_r14(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R14 as _, v)
    }

    pub unsafe fn r15(&self) -> u64 {
        cpu_get_reg64(self.handle, GpReg::R15 as _)
    }

    pub unsafe fn set_r15(&self, v: u64) {
        cpu_set_reg64(self.handle, GpReg::R15 as _, v)
    }

    pub unsafe fn rflags(&self) -> u32 {
        cpu_get_eflags(self.handle)
    }

    pub unsafe fn set_rflags(&self, v: u32) {
        cpu_set_eflags(self.handle, v)
    }

    // segment registers
    // TODO

    // debug registers

    pub unsafe fn dr0(&self) -> Address {
        cpu_get_dr(self.handle, DrReg::Dr0 as _)
    }

    pub unsafe fn set_dr0(&self, v: Address) {
        cpu_set_dr(self.handle, DrReg::Dr0 as _, v)
    }

    pub unsafe fn dr1(&self) -> Address {
        cpu_get_dr(self.handle, DrReg::Dr1 as _)
    }

    pub unsafe fn set_dr1(&self, v: Address) {
        cpu_set_dr(self.handle, DrReg::Dr1 as _, v)
    }

    pub unsafe fn dr2(&self) -> Address {
        cpu_get_dr(self.handle, DrReg::Dr2 as _)
    }

    pub unsafe fn set_dr2(&self, v: Address) {
        cpu_set_dr(self.handle, DrReg::Dr2 as _, v)
    }

    pub unsafe fn dr3(&self) -> Address {
        cpu_get_dr(self.handle, DrReg::Dr3 as _)
    }

    pub unsafe fn set_dr3(&self, v: Address) {
        cpu_set_dr(self.handle, DrReg::Dr3 as _, v)
    }

    pub unsafe fn dr6(&self) -> u32 {
        cpu_get_dr6(self.handle)
    }

    pub unsafe fn set_dr6(&self, v: u32) {
        cpu_set_dr6(self.handle, v)
    }

    pub unsafe fn dr7(&self) -> u32 {
        cpu_get_dr7(self.handle)
    }

    pub unsafe fn set_dr7(&self, v: u32) {
        cpu_set_dr7(self.handle, v)
    }

    // control registers

    pub unsafe fn cr0(&self) -> u32 {
        cpu_get_cr0(self.handle)
    }

    pub unsafe fn set_cr0(&self, v: u32) {
        cpu_set_cr0(self.handle, v)
    }

    pub unsafe fn cr2(&self) -> Address {
        cpu_get_cr2(self.handle)
    }

    pub unsafe fn set_cr2(&self, v: Address) {
        cpu_set_cr2(self.handle, v)
    }

    pub unsafe fn cr3(&self) -> Address {
        cpu_get_cr3(self.handle)
    }

    pub unsafe fn set_cr3(&self, v: Address) {
        cpu_set_cr3(self.handle, v)
    }

    pub unsafe fn cr4(&self) -> u32 {
        cpu_get_cr4(self.handle)
    }

    pub unsafe fn set_cr4(&self, v: u32) {
        cpu_set_cr4(self.handle, v)
    }

    pub unsafe fn efer(&self) -> u32 {
        cpu_get_efer(self.handle)
    }

    pub unsafe fn set_efer(&self, v: u32) {
        cpu_set_efer(self.handle, v)
    }

    pub unsafe fn xcr0(&self) -> u32 {
        cpu_get_xcr0(self.handle)
    }

    pub unsafe fn set_xcr0(&self, v: u32) {
        cpu_set_xcr0(self.handle, v)
    }

    // msrs

    pub unsafe fn kernel_gs_base(&self) -> u64 {
        cpu_get_kernel_gs_base(self.handle)
    }

    pub unsafe fn set_kernel_gs_base(&self, v: u64) {
        cpu_set_kernel_gs_base(self.handle, v)
    }

    pub unsafe fn sysenter_cs(&self) -> u32 {
        cpu_get_sysenter_cs(self.handle)
    }

    pub unsafe fn set_sysenter_cs(&self, v: u32) {
        cpu_set_sysenter_cs(self.handle, v)
    }

    pub unsafe fn sysenter_eip(&self) -> Address {
        cpu_get_sysenter_eip(self.handle)
    }

    pub unsafe fn set_sysenter_eip(&self, v: Address) {
        cpu_set_sysenter_eip(self.handle, v)
    }

    pub unsafe fn sysenter_esp(&self) -> Address {
        cpu_get_sysenter_esp(self.handle)
    }

    pub unsafe fn set_sysenter_esp(&self, v: Address) {
        cpu_set_sysenter_esp(self.handle, v)
    }

    pub unsafe fn star(&self) -> u64 {
        cpu_get_star(self.handle)
    }

    pub unsafe fn set_star(&self, v: u64) {
        cpu_set_star(self.handle, v)
    }

    pub unsafe fn lstar(&self) -> u64 {
        cpu_get_lstar(self.handle)
    }

    pub unsafe fn set_lstar(&self, v: u64) {
        cpu_set_lstar(self.handle, v)
    }

    pub unsafe fn cstar(&self) -> u64 {
        cpu_get_cstar(self.handle)
    }

    pub unsafe fn set_cstar(&self, v: u64) {
        cpu_set_cstar(self.handle, v)
    }

    pub unsafe fn fmask(&self) -> u32 {
        cpu_get_fmask(self.handle)
    }

    pub unsafe fn set_fmask(&self, v: u32) {
        cpu_set_fmask(self.handle, v)
    }


    // TODO pat
    // TODO mtrrphys?

}
