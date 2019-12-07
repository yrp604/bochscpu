use std::ffi::c_void;
use std::slice;
use std::hint::unreachable_unchecked;

use crate::{Address, PhyAddress};
use crate::cpu::{cpu_bail, cpu_killbit};

// If static mut gets the axe:
//
// use std::cell::UnsafeCell;
//
// static INIT_ENV_HOOKS: UnsafeCell<Vec<Box<InitEnvHook>>> = UnsafeCell::new(Vec::new());
// ...
// pub unsafe fn init_env<T: InitEnvHook + 'static>(h: T) {
//     (*(INIT_ENV_HOOKS.get())).push(Box::new(h))
// }
//
// #[no_mangle]
// extern "C" fn bx_instr_init_env() {
//     unsafe { INIT_ENV_HOOKS.get().iter_mut().for_each(|x| x()) }
// }

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

static mut RESET_HOOKS: Vec<Box<dyn FnMut(u32, u32)>> = Vec::new();
static mut HLT_HOOKS: Vec<Box<dyn FnMut(u32)>> = Vec::new();
static mut MWAIT_HOOKS: Vec<Box<dyn FnMut(u32, PhyAddress, usize, u32)>> = Vec::new();

static mut CNEAR_BRANCH_TAKEN_HOOKS: Vec<Box<dyn FnMut(u32, Address, Address)>> = Vec::new();
static mut CNEAR_BRANCH_NOT_TAKEN_HOOKS: Vec<Box<dyn FnMut(u32, Address)>> = Vec::new();
static mut UCNEAR_BRANCH_HOOKS: Vec<Box<dyn FnMut(u32, Branch, Address, Address)>> = Vec::new();
static mut FAR_BRANCH_HOOKS: Vec<Box<dyn FnMut(u32, Branch, (u16, Address), (u16, Address))>> = Vec::new();

static mut OPCODE_HOOKS: Vec<Box<dyn FnMut(u32, *mut c_void, &[u8], bool, bool)>> = Vec::new();
static mut INTERRUPT_HOOKS: Vec<Box<dyn FnMut(u32, u32)>> = Vec::new();
static mut EXCEPTION_HOOKS: Vec<Box<dyn FnMut(u32, u32, u32)>> = Vec::new();
static mut HW_INTERRUPT_HOOKS: Vec<Box<dyn FnMut(u32, u32, (u16, Address))>> = Vec::new();

static mut TLB_CNTRL_HOOKS: Vec<Box<dyn FnMut(u32, TlbCntrl, Option<PhyAddress>)>> = Vec::new();
static mut CACHE_CNTRL_HOOKS: Vec<Box<dyn FnMut(u32, CacheCntrl)>> = Vec::new();
static mut PREFETCH_HINT_HOOKS: Vec<Box<dyn FnMut(u32, PrefetchHint, u32, Address)>> = Vec::new();
static mut CLFLUSH_HOOKS: Vec<Box<dyn FnMut(u32, Address, PhyAddress)>> = Vec::new();

static mut BEFORE_EXECUTION_HOOKS: Vec<Box<dyn FnMut(u32, *mut c_void)>> = Vec::new();
static mut AFTER_EXECUTION_HOOKS: Vec<Box<dyn FnMut(u32, *mut c_void)>> = Vec::new();
static mut REPEAT_ITERATION_HOOKS: Vec<Box<dyn FnMut(u32, *mut c_void)>> = Vec::new();

static mut INP_HOOKS: Vec<Box<dyn FnMut(u16, usize)>> = Vec::new();
static mut INP2_HOOKS: Vec<Box<dyn FnMut(u16, usize, u32)>> = Vec::new();
static mut OUTP_HOOKS: Vec<Box<dyn FnMut(u16, usize, u32)>> = Vec::new();

static mut LIN_ACCESS_HOOKS: Vec<Box<dyn FnMut(u32, Address, Address, usize, u32, MemAccess)>> = Vec::new();
static mut PHY_ACCESS_HOOKS: Vec<Box<dyn FnMut(u32, Address, usize, u32, MemAccess)>> = Vec::new();

static mut WRMSR_HOOKS: Vec<Box<dyn FnMut(u32, u32, u64)>> = Vec::new();

static mut VMEXIT_HOOKS: Vec<Box<dyn FnMut(u32, u32, u64)>> = Vec::new();

// these should not be callable from the main cpu, thus shouldnt be hitable...
#[no_mangle]
extern "C" fn bx_instr_init_env() {}
#[no_mangle]
extern "C" fn bx_instr_exit_env() {}
#[no_mangle]
extern "C" fn bx_instr_initialize(_: u32) {}
#[no_mangle]
extern "C" fn bx_instr_exit(_: u32) {}

//

pub unsafe fn reset<T: FnMut(u32, u32) + 'static>(h: T) {
    RESET_HOOKS.push(Box::new(h))
}

pub unsafe fn reset_clear() {
    RESET_HOOKS.clear()
}

pub unsafe fn hlt<T: FnMut(u32) + 'static>(h: T) {
    HLT_HOOKS.push(Box::new(h))
}

pub unsafe fn hlt_clear() {
    HLT_HOOKS.clear()
}

pub unsafe fn mwait<T: FnMut(u32, PhyAddress, usize, u32) + 'static>(h: T) {
    MWAIT_HOOKS.push(Box::new(h))
}

pub unsafe fn mwait_clear() {
    MWAIT_HOOKS.clear()
}
#[no_mangle]
extern "C" fn bx_instr_reset(cpu: u32, ty: u32) {
    unsafe {
        RESET_HOOKS.iter_mut().for_each(|x| x(cpu, ty));

        // avoid the overhead of calling Cpu::from and just check the raw flags
        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_hlt(cpu: u32) {
    unsafe {
        HLT_HOOKS.iter_mut().for_each(|x| x(cpu));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_mwait(cpu: u32, addr: PhyAddress, len: u32, flags: u32) {
    unsafe {
        MWAIT_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, addr, len as usize, flags));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn cnear_branch_taken<T: FnMut(u32, Address, Address) + 'static>(h: T) {
    CNEAR_BRANCH_TAKEN_HOOKS.push(Box::new(h))
}

pub unsafe fn cnear_branch_taken_clear() {
    CNEAR_BRANCH_TAKEN_HOOKS.clear()
}

pub unsafe fn cnear_branch_not_taken<T: FnMut(u32, Address) + 'static>(h: T) {
    CNEAR_BRANCH_NOT_TAKEN_HOOKS.push(Box::new(h))
}

pub unsafe fn cnear_branch_not_taken_clear() {
    CNEAR_BRANCH_NOT_TAKEN_HOOKS.clear()
}

pub unsafe fn ucnear_branch<T: FnMut(u32, Branch, Address, Address) + 'static>(h: T) {
    UCNEAR_BRANCH_HOOKS.push(Box::new(h))
}

pub unsafe fn ucnear_branch_clear() {
    UCNEAR_BRANCH_HOOKS.clear()
}

pub unsafe fn far_branch<T: FnMut(u32, Branch, (u16, Address), (u16, Address)) + 'static>(h: T) {
    FAR_BRANCH_HOOKS.push(Box::new(h))
}

pub unsafe fn far_branch_clear() {
    FAR_BRANCH_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_cnear_branch_taken(cpu: u32, branch_eip: Address, new_eip: Address) {
    unsafe {
        CNEAR_BRANCH_TAKEN_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, branch_eip, new_eip));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_cnear_branch_not_taken(cpu: u32, branch_eip: Address) {
    unsafe {
        CNEAR_BRANCH_NOT_TAKEN_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, branch_eip));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_ucnear_branch(cpu: u32, what: u32, branch_eip: Address, new_eip: Address) {
    unsafe {
        UCNEAR_BRANCH_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, what.into(), branch_eip, new_eip));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_far_branch(
    cpu: u32,
    what: u32,
    prev_cs: u16,
    prev_eip: Address,
    new_cs: u16,
    new_eip: Address,
) {
    unsafe {
        FAR_BRANCH_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, what.into(), (prev_cs, prev_eip), (new_cs, new_eip)));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

pub unsafe fn opcode<T: FnMut(u32, *mut c_void, &[u8], bool, bool) + 'static>(h: T) {
    OPCODE_HOOKS.push(Box::new(h))
}

pub unsafe fn opcode_clear() {
    OPCODE_HOOKS.clear()
}

pub unsafe fn interrupt<T: FnMut(u32, u32) + 'static>(h: T) {
    INTERRUPT_HOOKS.push(Box::new(h))
}

pub unsafe fn interrupt_clear() {
    INTERRUPT_HOOKS.clear()
}

pub unsafe fn exception<T: FnMut(u32, u32, u32) + 'static>(h: T) {
    EXCEPTION_HOOKS.push(Box::new(h))
}

pub unsafe fn exception_clear() {
    EXCEPTION_HOOKS.clear()
}

pub unsafe fn hw_interrupt<T: FnMut(u32, u32, (u16, Address)) + 'static>(h: T) {
    HW_INTERRUPT_HOOKS.push(Box::new(h))
}

pub unsafe fn hw_interrupt_clear() {
    HW_INTERRUPT_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_opcode(
    cpu: u32,
    i: *mut c_void,
    opcode: *const u8,
    len: u32,
    is32: u32,
    is64: u32,
) {
    unsafe {
        OPCODE_HOOKS.iter_mut().for_each(|x| {
            x(
                cpu,
                i as *mut _ as *mut c_void,
                slice::from_raw_parts(opcode, len as usize),
                is32 != 0,
                is64 != 0,
            )
        });

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_interrupt(cpu: u32, vector: u32) {
    unsafe {
        INTERRUPT_HOOKS.iter_mut().for_each(|x| x(cpu, vector));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

#[no_mangle]
extern "C" fn bx_instr_exception(cpu: u32, vector: u32, error_code: u32) {
    unsafe {
        EXCEPTION_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, vector, error_code));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_hwinterrupt(cpu: u32, vector: u32, cs: u16, eip: Address) {
    unsafe {
        HW_INTERRUPT_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, vector, (cs, eip)));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn tlb_cntrl<T: FnMut(u32, TlbCntrl, Option<PhyAddress>) + 'static>(h: T) {
    TLB_CNTRL_HOOKS.push(Box::new(h))
}

pub unsafe fn tlb_cntrl_clear() {
    TLB_CNTRL_HOOKS.clear()
}

pub unsafe fn cache_cntrl<T: FnMut(u32, CacheCntrl) + 'static>(h: T) {
    CACHE_CNTRL_HOOKS.push(Box::new(h))
}

pub unsafe fn cache_cntrl_clear() {
    CACHE_CNTRL_HOOKS.clear()
}

pub unsafe fn prefetch_hint<T: FnMut(u32, PrefetchHint, u32, Address) + 'static>(h: T) {
    PREFETCH_HINT_HOOKS.push(Box::new(h))
}

pub unsafe fn prefetch_hint_clear() {
    PREFETCH_HINT_HOOKS.clear()
}

pub unsafe fn clflush<T: FnMut(u32, Address, PhyAddress) + 'static>(h: T) {
    CLFLUSH_HOOKS.push(Box::new(h))
}

pub unsafe fn clflush_clear() {
    CLFLUSH_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_tlb_cntrl(cpu: u32, what: u32, new_cr3: PhyAddress) {
    let ty = what.into();
    let maybe_cr3 = match ty {
        TlbCntrl::MovCr0 => Some(new_cr3),
        TlbCntrl::MovCr3 => Some(new_cr3),
        TlbCntrl::MovCr4 => Some(new_cr3),
        TlbCntrl::TaskSwitch => Some(new_cr3),
        _ => None,
    };

    unsafe {
        TLB_CNTRL_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, ty, maybe_cr3));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_cache_cntrl(cpu: u32, what: u32) {
    unsafe {
        CACHE_CNTRL_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, what.into()));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_prefetch_hint(cpu: u32, what: u32, seg: u32, offset: Address) {
    unsafe {
        PREFETCH_HINT_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, what.into(), seg, offset));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_clflush(cpu: u32, laddr: Address, paddr: PhyAddress) {
    unsafe {
        CLFLUSH_HOOKS.iter_mut().for_each(|x| x(cpu, laddr, paddr));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

/// Hook before instruction execution
///
/// # Note
///
/// This hook can be executed multiple times for a single instruction,
/// consider `push rax`, where the stack is not present in the current page
/// table. In that case, this hook execute then will generate a #PF. The
/// emulator could service that #PF, and then return to the `push` and execute
/// the hook again.
pub unsafe fn before_execution<T: FnMut(u32, *mut c_void) + 'static>(h: T) {
    BEFORE_EXECUTION_HOOKS.push(Box::new(h))
}

pub unsafe fn before_execution_clear() {
    BEFORE_EXECUTION_HOOKS.clear()
}

pub unsafe fn after_execution<T: FnMut(u32, *mut c_void) + 'static>(h: T) {
    AFTER_EXECUTION_HOOKS.push(Box::new(h))
}

pub unsafe fn after_execution_clear() {
    AFTER_EXECUTION_HOOKS.clear()
}

pub unsafe fn repeat_iteration<T: FnMut(u32, *mut c_void) + 'static>(h: T) {
    REPEAT_ITERATION_HOOKS.push(Box::new(h))
}

pub unsafe fn repeat_iteration_clear() {
    REPEAT_ITERATION_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_before_execution(cpu: u32, i: *mut c_void) {
    unsafe {
        BEFORE_EXECUTION_HOOKS.iter_mut().for_each(|x| x(cpu, i));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_after_execution(cpu: u32, i: *mut c_void) {
    unsafe {
        AFTER_EXECUTION_HOOKS.iter_mut().for_each(|x| x(cpu, i));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}
#[no_mangle]
extern "C" fn bx_instr_repeat_iteration(cpu: u32, i: *mut c_void) {
    unsafe {
        REPEAT_ITERATION_HOOKS.iter_mut().for_each(|x| x(cpu, i));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn inp<T: FnMut(u16, usize) + 'static>(h: T) {
    INP_HOOKS.push(Box::new(h))
}

pub unsafe fn inp_clear() {
    INP_HOOKS.clear()
}

pub unsafe fn inp2<T: FnMut(u16, usize, u32) + 'static>(h: T) {
    INP2_HOOKS.push(Box::new(h))
}

pub unsafe fn inp2_clear() {
    INP2_HOOKS.clear()
}

pub unsafe fn outp<T: FnMut(u16, usize, u32) + 'static>(h: T) {
    OUTP_HOOKS.push(Box::new(h))
}

pub unsafe fn outp_clear() {
    OUTP_HOOKS.clear()
}

// XXX these functions don't have cpuid's passed to them, so we cant check the
// kill bit easily...

#[no_mangle]
extern "C" fn bx_instr_inp(addr: u16, len: u32) {
    unsafe {
        INP_HOOKS.iter_mut().for_each(|x| x(addr, len as usize));
    }
}
#[no_mangle]
extern "C" fn bx_instr_inp2(addr: u16, len: u32, val: u32) {
    unsafe {
        INP2_HOOKS
            .iter_mut()
            .for_each(|x| x(addr, len as usize, val));
    }
}
#[no_mangle]
extern "C" fn bx_instr_outp(addr: u16, len: u32, val: u32) {
    unsafe {
        OUTP_HOOKS
            .iter_mut()
            .for_each(|x| x(addr, len as usize, val));
    }
}

//

pub unsafe fn lin_access<T: FnMut(u32, Address, Address, usize, u32, MemAccess) + 'static>(h: T) {
    LIN_ACCESS_HOOKS.push(Box::new(h))
}

pub unsafe fn lin_access_clear() {
    LIN_ACCESS_HOOKS.clear()
}

pub unsafe fn phy_access<T: FnMut(u32, Address, usize, u32, MemAccess) + 'static>(h: T) {
    PHY_ACCESS_HOOKS.push(Box::new(h))
}

pub unsafe fn phy_access_clear() {
    PHY_ACCESS_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_lin_access(
    cpu: u32,
    lin: Address,
    phy: Address,
    len: u32,
    memtype: u32,
    rw: u32,
) {
    unsafe {
        LIN_ACCESS_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, lin, phy, len as usize, memtype, rw.into()));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

#[no_mangle]
extern "C" fn bx_instr_phy_access(cpu: u32, phy: Address, len: u32, memtype: u32, rw: u32) {
    unsafe {
        PHY_ACCESS_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, phy, len as usize, memtype, rw.into()));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn wrmsr<T: FnMut(u32, u32, u64) + 'static>(h: T) {
    WRMSR_HOOKS.push(Box::new(h))
}

pub unsafe fn wrmsr_clear() {
    WRMSR_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_wrmsr(cpu: u32, addr: u32, value: u64) {
    unsafe {
        WRMSR_HOOKS.iter_mut().for_each(|x| x(cpu, addr, value));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn vmexit<T: FnMut(u32, u32, u64) + 'static>(h: T) {
    VMEXIT_HOOKS.push(Box::new(h))
}

pub unsafe fn vmexit_clear() {
    VMEXIT_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_vmexit(cpu: u32, reason: u32, qualification: u64) {
    unsafe {
        VMEXIT_HOOKS
            .iter_mut()
            .for_each(|x| x(cpu, reason, qualification));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

pub unsafe fn clear() {
    reset_clear();
    hlt_clear();
    mwait_clear();

    cnear_branch_taken_clear();
    cnear_branch_not_taken_clear();
    ucnear_branch_clear();
    far_branch_clear();

    opcode_clear();
    interrupt_clear();
    exception_clear();
    hw_interrupt_clear();

    tlb_cntrl_clear();
    cache_cntrl_clear();
    prefetch_hint_clear();
    clflush_clear();

    before_execution_clear();
    after_execution_clear();
    repeat_iteration_clear();

    inp_clear();
    inp2_clear();
    outp_clear();

    lin_access_clear();
    phy_access_clear();

    wrmsr_clear();
    vmexit_clear();
}
