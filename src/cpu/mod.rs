use std::convert::TryInto;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hook::{self, Hooks};
use crate::syncunsafecell::SyncUnsafeCell;
use crate::{Address, PhyAddress, NUM_CPUS};

mod state;
pub use state::State;

// look at lock_api crate to see if I can figure out how to do cpu locking
// so I dont need to make everything unsafe

extern "C" {
    fn cpu_new(id: u32);
    fn cpu_delete(id: u32);

    fn cpu_loop(id: u32);

    fn cpu_set_mode(id: u32);

    fn cpu_get_pc(id: u32) -> u64;
    fn cpu_set_pc(id: u32, val: u64);
    fn cpu_set_sp(id: u32, val: u64);

    fn cpu_get_reg64(id: u32, reg: u32) -> u64;
    fn cpu_set_reg64(id: u32, reg: u32, val: u64);

    fn cpu_get_eflags(id: u32) -> u32;
    fn cpu_set_eflags(id: u32, eflags: u32);

    fn cpu_get_seg(
        id: u32,
        seg: u32,
        present: *mut u32,
        selector: *mut u16,
        base: *mut Address,
        limit: *mut u32,
        attr: *mut u16,
    );
    fn cpu_set_seg(
        id: u32,
        seg: u32,
        present: u32,
        selector: u16,
        base: Address,
        limit: u32,
        attr: u16,
    );
    fn cpu_get_ldtr(
        id: u32,
        present: *mut u32,
        selector: *mut u16,
        base: *mut Address,
        limit: *mut u32,
        attr: *mut u16,
    );
    fn cpu_set_ldtr(id: u32, present: u32, selector: u16, base: Address, limit: u32, attr: u16);
    fn cpu_get_tr(
        id: u32,
        present: *mut u32,
        selector: *mut u16,
        base: *mut Address,
        limit: *mut u32,
        attr: *mut u16,
    );
    fn cpu_set_tr(id: u32, present: u32, selector: u16, base: Address, limit: u32, attr: u16);
    fn cpu_get_gdtr(id: u32, base: *mut Address, limit: *mut u16);
    fn cpu_set_gdtr(id: u32, base: Address, limit: u16);
    fn cpu_get_idtr(id: u32, base: *mut Address, limit: *mut u16);
    fn cpu_set_idtr(id: u32, base: Address, limit: u16);

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
    fn cpu_get_cr8(id: u32) -> u32;
    fn cpu_set_cr8(id: u32, val: u32);
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
    fn cpu_get_tsc(id: u32) -> u64;
    fn cpu_set_tsc(id: u32, val: u64);
    fn cpu_get_tsc_aux(id: u32) -> u32;
    fn cpu_set_tsc_aux(id: u32, val: u32);
    fn cpu_get_apicbase(id: u32) -> PhyAddress;
    fn cpu_set_apicbase(id: u32, val: PhyAddress);
    fn cpu_get_pat(id: u32) -> u64;
    fn cpu_set_pat(id: u32, val: u64);

    fn cpu_get_zmm(id: u32, reg: u32, val: *mut u64);
    fn cpu_set_zmm(id: u32, reg: u32, val: *const u64);
    fn cpu_get_mxcsr(id: u32) -> u32;
    fn cpu_set_mxcsr(id: u32, val: u32);
    fn cpu_get_mxcsr_mask(id: u32) -> u32;
    fn cpu_set_mxcsr_mask(id: u32, val: u32);

    fn cpu_get_fp_cw(id: u32) -> u16;
    fn cpu_set_fp_cw(id: u32, val: u16);
    fn cpu_get_fp_sw(id: u32) -> u16;
    fn cpu_set_fp_sw(id: u32, val: u16);
    fn cpu_get_fp_tw(id: u32) -> u16;
    fn cpu_set_fp_tw(id: u32, val: u16);
    fn cpu_get_fp_op(id: u32) -> u16;
    fn cpu_set_fp_op(id: u32, val: u16);
    fn cpu_get_fp_st(id: u32, reg: u32) -> u64;
    fn cpu_set_fp_st(id: u32, reg: u32, val: u64);

    /// Bail out of the cpu eval loop
    ///
    /// This ffi's to longjmp, meaning some variables drop routines might be
    /// skipped, depending on the context and platform
    pub(crate) fn cpu_bail(id: u32) -> !;

    /// Return the current killbit status
    pub(crate) fn cpu_killbit(id: u32) -> u32;
    fn cpu_set_killbit(id: u32);
    fn cpu_clear_killbit(id: u32);
    fn cpu_exception(id: u32, vector: u32, error: u16);
}

enum GpRegs {
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
    // unused Rip = 16,
}

enum SegRegs {
    Es = 0,
    Cs = 1,
    Ss = 2,
    Ds = 3,
    Fs = 4,
    Gs = 5,
}

enum DRegs {
    Dr0 = 0,
    Dr1 = 1,
    Dr2 = 2,
    Dr3 = 3,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Zmm {
    pub q: [u64; 8],
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GlobalSeg {
    pub base: Address,
    pub limit: u16,
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Seg {
    pub present: bool,
    pub selector: u16,
    pub base: Address,
    pub limit: u32,
    pub attr: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RunState {
    Go,
    Stop,
    Bail,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct Tracking {
    seed: u64,
    pub(crate) state: RunState,
}

impl Default for Tracking {
    fn default() -> Self {
        Self {
            seed: 0,
            state: RunState::Stop,
        }
    }
}

#[ctor]
static CPU_TRACKING: SyncUnsafeCell<Vec<Tracking>> =
    { SyncUnsafeCell::new(vec![Tracking::default(); NUM_CPUS]) };

unsafe fn cpu_tracking(id: u32) -> &'static mut Tracking {
    &mut (*(CPU_TRACKING.0.get()))[id as usize]
}

pub(crate) unsafe fn run_state(id: u32) -> RunState {
    cpu_tracking(id).state
}

pub(crate) unsafe fn set_run_state(id: u32, rs: RunState) {
    cpu_tracking(id).state = rs;
}

unsafe fn seed(id: u32) -> u64 {
    cpu_tracking(id).seed
}

unsafe fn set_seed(id: u32, seed: u64) {
    cpu_tracking(id).seed = seed;
}

#[no_mangle]
extern "C" fn bochscpu_rand(id: u32) -> u64 {
    let seed = unsafe { seed(id) };
    let hash = blake3::hash(&seed.to_le_bytes());

    // set the seed from the low 64 bits
    let new_seed = u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap());
    unsafe { set_seed(id, new_seed) };

    // return the next 64 bits as entropy
    u64::from_le_bytes(hash.as_bytes()[8..16].try_into().unwrap())
}

pub struct CpuRun<'a> {
    cpu: &'a Cpu
}

impl<'a> CpuRun<'a> {
    pub fn new(cpu: &'a Cpu) -> Self {
        Self { cpu }
    }

    pub unsafe fn run(self) -> RunState {
        self.cpu.set_run_state(RunState::Go);

        while cpu_killbit(self.cpu.handle) == 0 {
            match run_state(self.cpu.handle) {
                RunState::Stop => break,
                _ => cpu_loop(self.cpu.handle),
            }
        }

        self.cpu.run_state()
    }

    pub unsafe fn register(self, hook: &mut dyn Hooks) -> Self {
        hook::register(hook);

        self
    }
}

impl<'a> Drop for CpuRun<'a> {
    fn drop(&mut self) {
        unsafe { hook::clear() };
    }
}

pub struct Cpu {
    handle: u32,
}

impl Cpu {
    pub unsafe fn new(id: u32) -> Self {
        cpu_new(id);

        Self { handle: id }
    }

    pub unsafe fn new_with_seed(id: u32, seed: u64) -> Self {
        let c = Self::new(id);
        c.set_seed(seed);

        c
    }

    pub unsafe fn new_with_state(id: u32, s: &State) -> Self {
        let c = Self::new(id);
        c.set_state(s);

        c
    }

    pub fn from(id: u32) -> Self {
        Self { handle: id }
    }

    pub fn id(&self) -> u32 {
        self.handle
    }

    pub unsafe fn delete(self) {
        cpu_delete(self.handle);
    }

    pub unsafe fn prepare(&self) -> CpuRun {
        CpuRun::new(self)
    }

    pub unsafe fn run_state(&self) -> RunState {
        run_state(self.handle)
    }

    pub unsafe fn set_run_state(&self, rs: RunState) {
        set_run_state(self.handle, rs);

        match rs {
            RunState::Stop => cpu_set_killbit(self.handle),
            _ => cpu_clear_killbit(self.handle),
        };
    }

    pub unsafe fn seed(&self) -> u64 {
        seed(self.handle)
    }

    pub unsafe fn set_seed(&self, new_seed: u64) {
        set_seed(self.handle, new_seed)
    }

    // rax=0000000000000000 rbx=00000202e01c5080 rcx=00000202e01b5c88
    // rdx=000000b69e6ef750 rsi=0000000000000000 rdi=000000b69e6ff750
    // rip=00007ffa91c37870 rsp=000000b69e6ef6b8 rbp=0000000000000000
    //  r8=0000000000010000  r9=000000b69e6ef704 r10=00000202e01c9a40
    // r11=00000202e01e42a0 r12=0000000000000000 r13=0000000000000000
    // r14=0000000000000000 r15=0000000000000000
    pub unsafe fn print_gprs(&self) {
        println!(
            "rax={:016x} rbx={:016x} rcx={:016x}",
            self.rax(),
            self.rbx(),
            self.rcx()
        );
        println!(
            "rdx={:016x} rsi={:016x} rdi={:016x}",
            self.rdx(),
            self.rsi(),
            self.rdi()
        );
        println!(
            "rip={:016x} rsp={:016x} rbp={:016x}",
            self.rip(),
            self.rsp(),
            self.rbp()
        );
        println!(
            " r8={:016x}  r9={:016x} r10={:016x}",
            self.r8(),
            self.r9(),
            self.r10()
        );
        println!(
            "r11={:016x} r12={:016x} r13={:016x}",
            self.r11(),
            self.r12(),
            self.r13()
        );
        println!("r14={:016x} r15={:016x}", self.r14(), self.r15());
    }

    pub unsafe fn state(&self) -> State {
        State {
            bochscpu_seed: seed(self.handle),

            rip: self.rip(),

            rax: self.rax(),
            rcx: self.rcx(),
            rdx: self.rdx(),
            rbx: self.rbx(),
            rsp: self.rsp(),
            rbp: self.rbp(),
            rsi: self.rsi(),
            rdi: self.rdi(),
            r8: self.r8(),
            r9: self.r9(),
            r10: self.r10(),
            r11: self.r11(),
            r12: self.r12(),
            r13: self.r13(),
            r14: self.r14(),
            r15: self.r15(),
            rflags: self.rflags(),

            es: self.es(),
            cs: self.cs(),
            ss: self.ss(),
            ds: self.ds(),
            fs: self.fs(),
            gs: self.gs(),

            ldtr: self.ldtr(),
            tr: self.tr(),
            gdtr: self.gdtr(),
            idtr: self.idtr(),

            dr0: self.dr0(),
            dr1: self.dr1(),
            dr2: self.dr2(),
            dr3: self.dr3(),
            dr6: self.dr6(),
            dr7: self.dr7(),

            cr0: self.cr0(),
            cr2: self.cr2(),
            cr3: self.cr3(),
            cr4: self.cr4(),
            cr8: self.cr8(),
            xcr0: self.xcr0(),

            kernel_gs_base: self.kernel_gs_base(),
            sysenter_cs: self.sysenter_cs(),
            sysenter_eip: self.sysenter_eip(),
            sysenter_esp: self.sysenter_esp(),
            star: self.star(),
            lstar: self.lstar(),
            cstar: self.cstar(),
            efer: self.efer(),
            sfmask: self.sfmask(),
            tsc: self.tsc(),
            apic_base: self.apic_base(),
            pat: self.pat(),
            tsc_aux: self.tsc_aux(),

            zmm: [
                self.zmm(0),
                self.zmm(1),
                self.zmm(2),
                self.zmm(3),
                self.zmm(4),
                self.zmm(5),
                self.zmm(6),
                self.zmm(7),
                self.zmm(8),
                self.zmm(9),
                self.zmm(10),
                self.zmm(11),
                self.zmm(12),
                self.zmm(13),
                self.zmm(14),
                self.zmm(15),
                self.zmm(16),
                self.zmm(17),
                self.zmm(18),
                self.zmm(19),
                self.zmm(20),
                self.zmm(21),
                self.zmm(22),
                self.zmm(23),
                self.zmm(24),
                self.zmm(25),
                self.zmm(26),
                self.zmm(27),
                self.zmm(28),
                self.zmm(29),
                self.zmm(30),
                self.zmm(31),
            ],
            mxcsr: self.mxcsr(),
            mxcsr_mask: self.mxcsr_mask(),

            fpcw: self.fp_cw(),
            fpsw: self.fp_sw(),
            fptw: self.fp_tw(),
            fpop: self.fp_op(),
            fpst: [
                self.fp_st(0),
                self.fp_st(1),
                self.fp_st(2),
                self.fp_st(3),
                self.fp_st(4),
                self.fp_st(5),
                self.fp_st(6),
                self.fp_st(7),
            ],
        }
    }

    pub unsafe fn set_state(&self, s: &State) {
        self.set_seed(s.bochscpu_seed);

        self.set_rip(s.rip);

        self.set_rax(s.rax);
        self.set_rcx(s.rcx);
        self.set_rdx(s.rdx);
        self.set_rbx(s.rbx);
        self.set_rsp(s.rsp);
        self.set_rbp(s.rbp);
        self.set_rsi(s.rsi);
        self.set_rdi(s.rdi);
        self.set_r8(s.r8);
        self.set_r9(s.r9);
        self.set_r10(s.r10);
        self.set_r11(s.r11);
        self.set_r12(s.r12);
        self.set_r13(s.r13);
        self.set_r14(s.r14);
        self.set_r15(s.r15);
        self.set_rflags(s.rflags);

        self.set_es(s.es);
        self.set_cs_raw(s.cs);
        self.set_ss(s.ss);
        self.set_ds(s.ds);
        self.set_fs(s.fs);
        self.set_gs(s.gs);

        self.set_ldtr(s.ldtr);
        self.set_tr(s.tr);
        self.set_gdtr(s.gdtr);
        self.set_idtr(s.idtr);

        self.set_dr0(s.dr0);
        self.set_dr1(s.dr1);
        self.set_dr2(s.dr2);
        self.set_dr3(s.dr3);
        self.set_dr6(s.dr6);
        self.set_dr7(s.dr7);

        self.set_cr0(s.cr0);
        self.set_cr2(s.cr2);
        self.set_cr3(s.cr3);
        self.set_cr4(s.cr4);
        self.set_cr8(s.cr8);
        self.set_xcr0(s.xcr0);

        self.set_kernel_gs_base(s.kernel_gs_base);
        self.set_sysenter_cs(s.sysenter_cs);
        self.set_sysenter_esp(s.sysenter_esp);
        self.set_sysenter_eip(s.sysenter_eip);
        self.set_efer(s.efer);
        self.set_star(s.star);
        self.set_lstar(s.lstar);
        self.set_cstar(s.cstar);
        self.set_sfmask(s.sfmask);
        self.set_tsc(s.tsc);
        self.set_apic_base(s.apic_base);
        self.set_pat(s.pat);
        self.set_tsc_aux(s.tsc_aux);

        for (ii, z) in (&s.zmm).iter().enumerate() {
            self.set_zmm(ii, *z);
        }
        self.set_mxcsr(s.mxcsr);
        self.set_mxcsr_mask(s.mxcsr);

        self.set_fp_cw(s.fpcw);
        self.set_fp_sw(s.fpsw);
        self.set_fp_tw(s.fptw);
        self.set_fp_op(s.fpop);

        for (ii, f) in (&s.fpst).iter().enumerate() {
            self.set_fp_st(ii, *f);
        }

        // because we used set_cs_raw we need to update the state manually.
        self.set_mode();
    }

    pub unsafe fn exception(&self, vector: u32, error: u16) {
        cpu_exception(self.handle, vector, error)
    }

    //
    // regs below here
    //

    // gp regs

    pub unsafe fn rip(&self) -> u64 {
        cpu_get_pc(self.handle)
    }

    pub unsafe fn set_rip(&self, v: u64) {
        cpu_set_pc(self.handle, v);

        self.set_run_state(RunState::Bail);
    }

    pub unsafe fn rax(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rax as _)
    }

    pub unsafe fn set_rax(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rax as _, v)
    }

    pub unsafe fn rcx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rcx as _)
    }

    pub unsafe fn set_rcx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rcx as _, v)
    }

    pub unsafe fn rdx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rdx as _)
    }

    pub unsafe fn set_rdx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rdx as _, v)
    }

    pub unsafe fn rbx(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rbx as _)
    }

    pub unsafe fn set_rbx(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rbx as _, v)
    }

    pub unsafe fn rsp(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rsp as _)
    }

    pub unsafe fn set_rsp(&self, v: u64) {
        cpu_set_sp(self.handle, v)
    }

    pub unsafe fn rbp(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rbp as _)
    }

    pub unsafe fn set_rbp(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rbp as _, v)
    }

    pub unsafe fn rsi(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rsi as _)
    }

    pub unsafe fn set_rsi(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rsi as _, v)
    }

    pub unsafe fn rdi(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::Rdi as _)
    }

    pub unsafe fn set_rdi(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::Rdi as _, v)
    }

    pub unsafe fn r8(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R8 as _)
    }

    pub unsafe fn set_r8(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R8 as _, v)
    }

    pub unsafe fn r9(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R9 as _)
    }

    pub unsafe fn set_r9(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R9 as _, v)
    }

    pub unsafe fn r10(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R10 as _)
    }

    pub unsafe fn set_r10(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R10 as _, v)
    }

    pub unsafe fn r11(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R11 as _)
    }

    pub unsafe fn set_r11(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R11 as _, v)
    }

    pub unsafe fn r12(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R12 as _)
    }

    pub unsafe fn set_r12(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R12 as _, v)
    }

    pub unsafe fn r13(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R13 as _)
    }

    pub unsafe fn set_r13(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R13 as _, v)
    }

    pub unsafe fn r14(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R14 as _)
    }

    pub unsafe fn set_r14(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R14 as _, v)
    }

    pub unsafe fn r15(&self) -> u64 {
        cpu_get_reg64(self.handle, GpRegs::R15 as _)
    }

    pub unsafe fn set_r15(&self, v: u64) {
        cpu_set_reg64(self.handle, GpRegs::R15 as _, v)
    }

    pub unsafe fn rflags(&self) -> u64 {
        cpu_get_eflags(self.handle) as _
    }

    pub unsafe fn set_rflags(&self, v: u64) {
        cpu_set_eflags(self.handle, v as _)
    }

    // segment registers

    unsafe fn seg(&self, s: SegRegs) -> Seg {
        let mut present = 0;
        let mut selector = 0;
        let mut base = 0;
        let mut limit = 0;
        let mut attr = 0;

        cpu_get_seg(
            self.handle,
            s as _,
            &mut present,
            &mut selector,
            &mut base,
            &mut limit,
            &mut attr,
        );

        Seg {
            present: present != 0,
            selector,
            base,
            limit,
            attr,
        }
    }

    unsafe fn set_seg(&self, s: SegRegs, v: Seg) {
        cpu_set_seg(
            self.handle,
            s as _,
            v.present as _,
            v.selector,
            v.base,
            v.limit,
            v.attr,
        )
    }

    pub unsafe fn es(&self) -> Seg {
        self.seg(SegRegs::Es)
    }

    pub unsafe fn set_es(&self, v: Seg) {
        self.set_seg(SegRegs::Es, v);
    }

    pub unsafe fn cs(&self) -> Seg {
        self.seg(SegRegs::Cs)
    }

    pub unsafe fn set_cs(&self, v: Seg) {
        self.set_seg(SegRegs::Cs, v);
        self.set_mode();
    }

    /// This function does not update the cpu mode after setting CS. This is
    /// needed as during cpu init, if efer isn't yet populated the cpu will not
    /// know that long mode might be active and change the state.
    pub unsafe fn set_cs_raw(&self, v: Seg) {
        self.set_seg(SegRegs::Cs, v);
    }

    pub unsafe fn ss(&self) -> Seg {
        self.seg(SegRegs::Ss)
    }

    pub unsafe fn set_ss(&self, v: Seg) {
        self.set_seg(SegRegs::Ss, v);
    }

    pub unsafe fn ds(&self) -> Seg {
        self.seg(SegRegs::Ds)
    }

    pub unsafe fn set_ds(&self, v: Seg) {
        self.set_seg(SegRegs::Ds, v);
    }

    pub unsafe fn fs(&self) -> Seg {
        self.seg(SegRegs::Fs)
    }

    pub unsafe fn set_fs(&self, v: Seg) {
        self.set_seg(SegRegs::Fs, v);
    }

    pub unsafe fn gs(&self) -> Seg {
        self.seg(SegRegs::Gs)
    }

    pub unsafe fn set_gs(&self, v: Seg) {
        self.set_seg(SegRegs::Gs, v);
    }

    pub unsafe fn ldtr(&self) -> Seg {
        let mut present = 0;
        let mut selector = 0;
        let mut base = 0;
        let mut limit = 0;
        let mut attr = 0;

        cpu_get_ldtr(
            self.handle,
            &mut present,
            &mut selector,
            &mut base,
            &mut limit,
            &mut attr,
        );

        Seg {
            present: present != 0,
            selector,
            base,
            limit,
            attr,
        }
    }

    pub unsafe fn set_ldtr(&self, v: Seg) {
        cpu_set_ldtr(
            self.handle,
            v.present as _,
            v.selector,
            v.base,
            v.limit,
            v.attr,
        )
    }

    pub unsafe fn tr(&self) -> Seg {
        let mut present = 0;
        let mut selector = 0;
        let mut base = 0;
        let mut limit = 0;
        let mut attr = 0;

        cpu_get_tr(
            self.handle,
            &mut present,
            &mut selector,
            &mut base,
            &mut limit,
            &mut attr,
        );

        Seg {
            present: present != 0,
            selector,
            base,
            limit,
            attr,
        }
    }

    pub unsafe fn set_tr(&self, v: Seg) {
        cpu_set_tr(
            self.handle,
            v.present as _,
            v.selector,
            v.base,
            v.limit,
            v.attr,
        )
    }

    pub unsafe fn gdtr(&self) -> GlobalSeg {
        let mut base = 0;
        let mut limit = 0;

        cpu_get_gdtr(self.handle, &mut base, &mut limit);

        GlobalSeg { base, limit }
    }

    pub unsafe fn set_gdtr(&self, gdtr: GlobalSeg) {
        cpu_set_gdtr(self.handle, gdtr.base, gdtr.limit)
    }

    pub unsafe fn idtr(&self) -> GlobalSeg {
        let mut base = 0;
        let mut limit = 0;

        cpu_get_idtr(self.handle, &mut base, &mut limit);

        GlobalSeg { base, limit }
    }

    pub unsafe fn set_idtr(&self, idtr: GlobalSeg) {
        cpu_set_idtr(self.handle, idtr.base, idtr.limit)
    }

    // debug registers

    pub unsafe fn dr0(&self) -> Address {
        cpu_get_dr(self.handle, DRegs::Dr0 as _)
    }

    pub unsafe fn set_dr0(&self, v: Address) {
        cpu_set_dr(self.handle, DRegs::Dr0 as _, v)
    }

    pub unsafe fn dr1(&self) -> Address {
        cpu_get_dr(self.handle, DRegs::Dr1 as _)
    }

    pub unsafe fn set_dr1(&self, v: Address) {
        cpu_set_dr(self.handle, DRegs::Dr1 as _, v)
    }

    pub unsafe fn dr2(&self) -> Address {
        cpu_get_dr(self.handle, DRegs::Dr2 as _)
    }

    pub unsafe fn set_dr2(&self, v: Address) {
        cpu_set_dr(self.handle, DRegs::Dr2 as _, v)
    }

    pub unsafe fn dr3(&self) -> Address {
        cpu_get_dr(self.handle, DRegs::Dr3 as _)
    }

    pub unsafe fn set_dr3(&self, v: Address) {
        cpu_set_dr(self.handle, DRegs::Dr3 as _, v)
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
        cpu_set_cr0(self.handle, v);
        self.set_mode();
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

    pub unsafe fn cr8(&self) -> u64 {
        cpu_get_cr8(self.handle) as _
    }

    pub unsafe fn set_cr8(&self, v: u64) {
        cpu_set_cr8(self.handle, v as _)
    }

    pub unsafe fn efer(&self) -> u32 {
        cpu_get_efer(self.handle)
    }

    pub unsafe fn set_efer(&self, v: u32) {
        cpu_set_efer(self.handle, v);
        self.set_mode();
    }

    /// Update the internal bochs cpu mode
    ///
    /// This corresponds to the bochs function `handleCpuModeChange()`, and is
    /// implicitly called via `set_cs`, `set_cr0`, or `set_efer`. By my
    /// understanding, these are the only functions which can result in a cpu
    /// mode change, but the function is exposed in case I've missed some. If
    /// you find some context where you need to call it, please file a bug.
    pub unsafe fn set_mode(&self) {
        cpu_set_mode(self.handle)
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

    pub unsafe fn sysenter_cs(&self) -> u64 {
        cpu_get_sysenter_cs(self.handle) as _
    }

    pub unsafe fn set_sysenter_cs(&self, v: u64) {
        cpu_set_sysenter_cs(self.handle, v as _)
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

    pub unsafe fn sfmask(&self) -> u64 {
        cpu_get_fmask(self.handle) as _
    }

    pub unsafe fn set_sfmask(&self, v: u64) {
        cpu_set_fmask(self.handle, v as _)
    }

    pub unsafe fn tsc(&self) -> u64 {
        cpu_get_tsc(self.handle)
    }

    pub unsafe fn set_tsc(&self, v: u64) {
        cpu_set_tsc(self.handle, v)
    }

    pub unsafe fn apic_base(&self) -> PhyAddress {
        cpu_get_apicbase(self.handle)
    }

    pub unsafe fn set_apic_base(&self, v: PhyAddress) {
        cpu_set_apicbase(self.handle, v)
    }

    pub unsafe fn pat(&self) -> u64 {
        cpu_get_pat(self.handle)
    }

    pub unsafe fn set_pat(&self, v: u64) {
        cpu_set_pat(self.handle, v)
    }

    pub unsafe fn tsc_aux(&self) -> u64 {
        cpu_get_tsc_aux(self.handle) as _
    }

    pub unsafe fn set_tsc_aux(&self, v: u64) {
        cpu_set_tsc_aux(self.handle, v as _)
    }

    // TODO mtrrphys?

    // zmm

    pub unsafe fn zmm(&self, idx: usize) -> Zmm {
        assert!(idx < 32);
        let mut v = Zmm { q: [0; 8] };
        cpu_get_zmm(self.handle, idx as _, &mut v.q as *mut _ as *mut u64);

        v
    }

    pub unsafe fn set_zmm(&self, idx: usize, v: Zmm) {
        assert!(idx < 32);
        cpu_set_zmm(self.handle, idx as _, &v.q as *const _ as *const u64)
    }

    pub unsafe fn mxcsr(&self) -> u32 {
        cpu_get_mxcsr(self.handle)
    }

    pub unsafe fn set_mxcsr(&self, v: u32) {
        cpu_set_mxcsr(self.handle, v)
    }

    pub unsafe fn mxcsr_mask(&self) -> u32 {
        cpu_get_mxcsr_mask(self.handle)
    }

    pub unsafe fn set_mxcsr_mask(&self, v: u32) {
        cpu_set_mxcsr_mask(self.handle, v)
    }

    // FP

    pub unsafe fn fp_cw(&self) -> u16 {
        cpu_get_fp_cw(self.handle)
    }

    pub unsafe fn set_fp_cw(&self, v: u16) {
        cpu_set_fp_cw(self.handle, v)
    }

    pub unsafe fn fp_sw(&self) -> u16 {
        cpu_get_fp_sw(self.handle)
    }

    pub unsafe fn set_fp_sw(&self, v: u16) {
        cpu_set_fp_sw(self.handle, v)
    }

    pub unsafe fn fp_tw(&self) -> u16 {
        cpu_get_fp_tw(self.handle)
    }

    pub unsafe fn set_fp_tw(&self, v: u16) {
        cpu_set_fp_tw(self.handle, v)
    }

    pub unsafe fn fp_op(&self) -> u16 {
        cpu_get_fp_op(self.handle)
    }

    pub unsafe fn set_fp_op(&self, v: u16) {
        cpu_set_fp_op(self.handle, v)
    }

    pub unsafe fn fp_st(&self, idx: usize) -> u64 {
        assert!(idx < 8);
        cpu_get_fp_st(self.handle, idx as _)
    }

    pub unsafe fn set_fp_st(&self, idx: usize, v: u64) {
        assert!(idx < 8);
        cpu_set_fp_st(self.handle, idx as _, v)
    }
}
