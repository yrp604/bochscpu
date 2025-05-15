use std::ffi::c_void;
use std::hint::unreachable_unchecked;
use std::mem;
use std::slice;

use crate::NUM_CPUS;
use crate::cpu::{cpu_bail, cpu_exception};
use crate::syncunsafecell::{SyncUnsafeCell, ptr_to_ref_mut};
use crate::{Address, PhyAddress};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum HookEvent {
    Stop,
    SetPc,
    Exception(u32, Option<u16>),
}

#[ctor]
static HOOK_EVENTS: SyncUnsafeCell<Vec<Option<HookEvent>>> =
    unsafe { SyncUnsafeCell::new(vec![None; NUM_CPUS]) };

pub(crate) unsafe fn hook_event(id: u32) -> &'static mut Option<HookEvent> {
    unsafe { &mut ptr_to_ref_mut(HOOK_EVENTS.0.get())[id as usize] }
}

pub(crate) unsafe fn set_hook_event(id: u32, he: Option<HookEvent>) {
    unsafe {
        *hook_event(id) = he;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum ResetSource {
    Software = 10,
    Hardware = 11,
}

impl From<u32> for ResetSource {
    fn from(i: u32) -> Self {
        match i {
            10 => ResetSource::Software,
            11 => ResetSource::Hardware,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum Branch {
    Jmp = 10,
    JmpIndirect = 11,
    Call = 12,
    CallIndirect = 13,
    Ret = 14,
    Iret = 15,
    Int = 16,
    Syscall = 17,
    Sysret = 18,
    Sysenter = 19,
    Sysexit = 20,
}

impl From<u32> for Branch {
    fn from(i: u32) -> Self {
        match i {
            10 => Branch::Jmp,
            11 => Branch::JmpIndirect,
            12 => Branch::Call,
            13 => Branch::CallIndirect,
            14 => Branch::Ret,
            15 => Branch::Iret,
            16 => Branch::Int,
            17 => Branch::Syscall,
            18 => Branch::Sysret,
            19 => Branch::Sysenter,
            20 => Branch::Sysexit,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum TlbCntrl {
    MovCr0 = 10,
    MovCr3 = 11,
    MovCr4 = 12,
    TaskSwitch = 13,
    ContextSwitch = 14,
    InvLpg = 15,
    InvEpt = 16,
    InvVpid = 17,
    InvPcid = 18,
}

impl From<u32> for TlbCntrl {
    fn from(i: u32) -> Self {
        match i {
            10 => TlbCntrl::MovCr0,
            11 => TlbCntrl::MovCr3,
            12 => TlbCntrl::MovCr4,
            13 => TlbCntrl::TaskSwitch,
            14 => TlbCntrl::ContextSwitch,
            15 => TlbCntrl::InvLpg,
            16 => TlbCntrl::InvEpt,
            17 => TlbCntrl::InvVpid,
            18 => TlbCntrl::InvPcid,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum CacheCntrl {
    Invd = 10,
    Wbind = 11,
}

impl From<u32> for CacheCntrl {
    fn from(i: u32) -> Self {
        match i {
            10 => CacheCntrl::Invd,
            11 => CacheCntrl::Wbind,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum PrefetchHint {
    Nta = 0,
    T0 = 1,
    T1 = 2,
    T2 = 3,
}

impl From<u32> for PrefetchHint {
    fn from(i: u32) -> Self {
        match i {
            0 => PrefetchHint::Nta,
            1 => PrefetchHint::T0,
            2 => PrefetchHint::T1,
            3 => PrefetchHint::T2,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum MemAccess {
    Read = 0,
    Write = 1,
    Execute = 2,
    RW = 3,
}

impl From<u32> for MemAccess {
    fn from(i: u32) -> Self {
        match i {
            0 => MemAccess::Read,
            1 => MemAccess::Write,
            2 => MemAccess::Execute,
            3 => MemAccess::RW,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum MemType {
    Uc = 0,
    Wc = 1,
    Reserved2 = 2,
    Reserved3 = 3,
    Wt = 4,
    Wp = 5,
    Wb = 6,
    UcWeak = 7,
    Invalid = 8,
}

impl From<u32> for MemType {
    fn from(i: u32) -> Self {
        match i {
            0 => MemType::Uc,
            1 => MemType::Wc,
            2 => MemType::Reserved2,
            3 => MemType::Reserved3,
            4 => MemType::Wt,
            5 => MemType::Wp,
            6 => MemType::Wb,
            7 => MemType::UcWeak,
            8 => MemType::Invalid,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

pub trait Hooks {
    fn reset(&mut self, _id: u32, _ty: ResetSource) {}
    fn hlt(&mut self, _id: u32) {}
    fn mwait(&mut self, _id: u32, _addr: PhyAddress, _len: usize, _flags: u32) {}

    fn cnear_branch_taken(&mut self, _id: u32, _branch_pc: Address, _new_pc: Address) {}
    fn cnear_branch_not_taken(&mut self, _id: u32, _pc: Address, _new_pc: Address) {}
    fn ucnear_branch(&mut self, _id: u32, _what: Branch, _branch_pc: Address, _new_pc: Address) {}
    fn far_branch(
        &mut self,
        _id: u32,
        _what: Branch,
        _branch_pc: (u16, Address),
        _new_pc: (u16, Address),
    ) {
    }

    fn opcode(
        &mut self,
        _id: u32,
        _ins: *const c_void,
        _opcode: &[u8],
        _is_32: bool,
        _is_64: bool,
    ) {
    }
    fn interrupt(&mut self, _id: u32, _vector: u32) {}
    fn exception(&mut self, _id: u32, _vector: u32, _error_code: u32) {}
    fn hw_interrupt(&mut self, _id: u32, _vector: u32, _pc: (u16, Address)) {}

    fn tlb_cntrl(&mut self, _id: u32, _what: TlbCntrl, _new_cr: Option<PhyAddress>) {}
    fn cache_cntrl(&mut self, _id: u32, _what: CacheCntrl) {}
    fn prefetch_hint(&mut self, _id: u32, _what: PrefetchHint, _seg: u32, _off: Address) {}
    fn clflush(&mut self, _id: u32, _vaddr: Address, _paddr: PhyAddress) {}

    fn before_execution(&mut self, _id: u32, _ins: *mut c_void) {}
    fn after_execution(&mut self, _id: u32, _ins: *mut c_void) {}
    fn repeat_iteration(&mut self, _id: u32, _ins: *mut c_void) {}

    fn inp(&mut self, _addr: u16, _len: usize) {}
    fn inp2(&mut self, _addr: u16, _len: usize, _val: u32) {}
    fn outp(&mut self, _addr: u16, _len: usize, _val: u32) {}

    fn lin_access(
        &mut self,
        _id: u32,
        _vaddr: Address,
        _paddr: Address,
        _len: usize,
        _memty: MemType,
        _rw: MemAccess,
    ) {
    }
    fn phy_access(
        &mut self,
        _id: u32,
        _paddr: PhyAddress,
        _len: usize,
        _memty: MemType,
        _rw: MemAccess,
    ) {
    }

    fn cpuid(&mut self, _id: u32) {}

    fn wrmsr(&mut self, _id: u32, _msr: u32, _val: u64) {}

    fn vmexit(&mut self, _id: u32, _reason: u32, _qualification: u64) {}
}

static HOOKS: SyncUnsafeCell<Vec<&mut dyn Hooks>> = SyncUnsafeCell::new(Vec::new());

unsafe fn hooks() -> &'static mut Vec<&'static mut dyn Hooks> {
    unsafe { ptr_to_ref_mut(HOOKS.0.get()) }
}

pub(crate) unsafe fn register<'a>(h: &'a mut dyn Hooks) {
    unsafe {
        // we need to extend the lifetime of this hook object to 'static so we can insert it
        let hook = mem::transmute::<&'a mut dyn Hooks, &'static mut dyn Hooks>(h);
        hooks().push(hook);
    }
}

pub(crate) unsafe fn clear() {
    unsafe {
        hooks().clear();
    }
}

// these should not be callable from the main cpu, thus shouldnt be hitable...
#[unsafe(no_mangle)]
extern "C" fn bx_instr_init_env() {}
#[unsafe(no_mangle)]
extern "C" fn bx_instr_exit_env() {}
#[unsafe(no_mangle)]
extern "C" fn bx_instr_initialize(_: u32) {}
#[unsafe(no_mangle)]
extern "C" fn bx_instr_exit(_: u32) {}

//

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_reset(cpu: u32, ty: u32) {
    unsafe {
        let src: ResetSource = ty.into();

        // this is kind of awkward -- we call reset during cpu init to initialize
        // parts of it, but the cpu is half-initialized when this callback gets
        // fired, so this crashes. We're going to assume that we're the only ones
        // that can do hardware resets, and only call hooks for software resets
        if src == ResetSource::Hardware {
            return;
        }

        hooks().iter_mut().for_each(|x| x.reset(cpu, src));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_hlt(cpu: u32) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.hlt(cpu));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_mwait(cpu: u32, addr: PhyAddress, len: u32, flags: u32) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.mwait(cpu, addr, len as usize, flags));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_cnear_branch_taken(
    cpu: u32,
    branch_eip: Address,
    new_eip: Address,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.cnear_branch_taken(cpu, branch_eip, new_eip));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_cnear_branch_not_taken(
    cpu: u32,
    branch_eip: Address,
    new_eip: Address,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.cnear_branch_not_taken(cpu, branch_eip, new_eip));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_ucnear_branch(
    cpu: u32,
    what: u32,
    branch_eip: Address,
    new_eip: Address,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.ucnear_branch(cpu, what.into(), branch_eip, new_eip));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_far_branch(
    cpu: u32,
    what: u32,
    prev_cs: u16,
    prev_eip: Address,
    new_cs: u16,
    new_eip: Address,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.far_branch(cpu, what.into(), (prev_cs, prev_eip), (new_cs, new_eip)));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_opcode(
    cpu: u32,
    i: *mut c_void,
    opcode: *const u8,
    len: u32,
    is32: u32,
    is64: u32,
) {
    unsafe {
        hooks().iter_mut().for_each(|x| {
            x.opcode(
                cpu,
                i,
                slice::from_raw_parts(opcode, len as usize),
                is32 != 0,
                is64 != 0,
            )
        });

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_interrupt(cpu: u32, vector: u32) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.interrupt(cpu, vector));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_exception(cpu: u32, vector: u32, error_code: u32) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.exception(cpu, vector, error_code));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_hwinterrupt(cpu: u32, vector: u32, cs: u16, eip: Address) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.hw_interrupt(cpu, vector, (cs, eip)));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn bx_instr_tlb_cntrl(cpu: u32, what: u32, new_cr3: PhyAddress) {
    let ty = what.into();
    let maybe_cr3 = match ty {
        TlbCntrl::MovCr0 => Some(new_cr3),
        TlbCntrl::MovCr3 => Some(new_cr3),
        TlbCntrl::MovCr4 => Some(new_cr3),
        TlbCntrl::TaskSwitch => Some(new_cr3),
        _ => None,
    };

    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.tlb_cntrl(cpu, ty, maybe_cr3));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_cache_cntrl(cpu: u32, what: u32) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.cache_cntrl(cpu, what.into()));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_prefetch_hint(cpu: u32, what: u32, seg: u32, offset: Address) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.prefetch_hint(cpu, what.into(), seg, offset));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_clflush(cpu: u32, laddr: Address, paddr: PhyAddress) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.clflush(cpu, laddr, paddr));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_before_execution(cpu: u32, i: *mut c_void) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.before_execution(cpu, i));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_after_execution(cpu: u32, i: *mut c_void) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.after_execution(cpu, i));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_repeat_iteration(cpu: u32, i: *mut c_void) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.repeat_iteration(cpu, i));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_lin_access(
    cpu: u32,
    lin: Address,
    phy: Address,
    len: u32,
    memtype: u32,
    rw: u32,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.lin_access(cpu, lin, phy, len as usize, memtype.into(), rw.into()));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_phy_access(
    cpu: u32,
    phy: Address,
    len: u32,
    memtype: u32,
    rw: u32,
) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.phy_access(cpu, phy, len as usize, memtype.into(), rw.into()));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_inp(addr: u16, len: u32) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.inp(addr, len as usize));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_inp2(addr: u16, len: u32, val: u32) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.inp2(addr, len as usize, val));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_outp(addr: u16, len: u32, val: u32) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.outp(addr, len as usize, val));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_cpuid(cpu: u32) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.cpuid(cpu));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_wrmsr(cpu: u32, addr: u32, value: u64) {
    unsafe {
        hooks().iter_mut().for_each(|x| x.wrmsr(cpu, addr, value));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn bx_instr_vmexit(cpu: u32, reason: u32, qualification: u64) {
    unsafe {
        hooks()
            .iter_mut()
            .for_each(|x| x.vmexit(cpu, reason, qualification));

        if let Some(e) = hook_event(cpu).take() {
            match e {
                HookEvent::Stop | HookEvent::SetPc => cpu_bail(cpu),
                HookEvent::Exception(vector, error) => {
                    cpu_exception(cpu, vector, error.unwrap_or(0))
                }
            }
        }
    }
}
