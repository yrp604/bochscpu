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

pub trait InitEnvHook = FnMut();
pub trait ExitEnvHook = FnMut();

static mut INIT_ENV_HOOKS: Vec<Box<dyn InitEnvHook>> = Vec::new();
static mut EXIT_ENV_HOOKS: Vec<Box<dyn ExitEnvHook>> = Vec::new();

pub trait InitializeHook = FnMut(u32);
pub trait ExitHook = FnMut(u32);
pub trait ResetHook = FnMut(u32, u32);
pub trait HltHook = FnMut(u32);
pub trait MwaitHook = FnMut(u32, PhyAddress, u32, u32);

static mut INITIALIZE_HOOKS: Vec<Box<dyn InitializeHook>> = Vec::new();
static mut EXIT_HOOKS: Vec<Box<dyn ExitHook>> = Vec::new();
static mut RESET_HOOKS: Vec<Box<dyn ResetHook>> = Vec::new();
static mut HLT_HOOKS: Vec<Box<dyn HltHook>> = Vec::new();
static mut MWAIT_HOOKS: Vec<Box<dyn MwaitHook>> = Vec::new();

pub trait CnearBranchTakenHook = FnMut(u32, Address, Address);
pub trait CnearBranchNotTakenHook = FnMut(u32, Address);
pub trait UcnearBranchHook = FnMut(u32, Branch, Address, Address);
pub trait FarBranchHook = FnMut(u32, Branch, (u16, Address), (u16, Address));

static mut CNEAR_BRANCH_TAKEN_HOOKS: Vec<Box<dyn CnearBranchTakenHook>> = Vec::new();
static mut CNEAR_BRANCH_NOT_TAKEN_HOOKS: Vec<Box<dyn CnearBranchNotTakenHook>> = Vec::new();
static mut UCNEAR_BRANCH_HOOKS: Vec<Box<dyn UcnearBranchHook>> = Vec::new();
static mut FAR_BRANCH_HOOKS: Vec<Box<dyn FarBranchHook>> = Vec::new();

pub trait OpcodeHook = FnMut(u32, *mut c_void, &[u8], bool, bool);
pub trait InterruptHook = FnMut(u32, u32);
pub trait ExceptionHook = FnMut(u32, u32, u32);
pub trait HwInterruptHook = FnMut(u32, u32, (u16, Address));

static mut OPCODE_HOOKS: Vec<Box<dyn OpcodeHook>> = Vec::new();
static mut INTERRUPT_HOOKS: Vec<Box<dyn InterruptHook>> = Vec::new();
static mut EXCEPTION_HOOKS: Vec<Box<dyn ExceptionHook>> = Vec::new();
static mut HW_INTERRUPT_HOOKS: Vec<Box<dyn HwInterruptHook>> = Vec::new();

pub trait TlbCntrlHook = FnMut(u32, TlbCntrl, Option<PhyAddress>);
pub trait CacheCntrlHook = FnMut(u32, CacheCntrl);
pub trait PrefetchHintHook = FnMut(u32, PrefetchHint, u32, Address);
pub trait ClflushHook = FnMut(u32, Address, PhyAddress);

static mut TLB_CNTRL_HOOKS: Vec<Box<dyn TlbCntrlHook>> = Vec::new();
static mut CACHE_CNTRL_HOOKS: Vec<Box<dyn CacheCntrlHook>> = Vec::new();
static mut PREFETCH_HINT_HOOKS: Vec<Box<dyn PrefetchHintHook>> = Vec::new();
static mut CLFLUSH_HOOKS: Vec<Box<dyn ClflushHook>> = Vec::new();

pub trait BeforeExecutionHook = FnMut(u32, *mut c_void);
pub trait AfterExecutionHook = FnMut(u32, *mut c_void);
pub trait RepeatIterationHook = FnMut(u32, *mut c_void);

static mut BEFORE_EXECUTION_HOOKS: Vec<Box<dyn BeforeExecutionHook>> = Vec::new();
static mut AFTER_EXECUTION_HOOKS: Vec<Box<dyn AfterExecutionHook>> = Vec::new();
static mut REPEAT_ITERATION_HOOKS: Vec<Box<dyn RepeatIterationHook>> = Vec::new();

pub trait InpHook = FnMut(u16, usize);
pub trait Inp2Hook = FnMut(u16, usize, u32);
pub trait OutpHook = FnMut(u16, usize, u32);

static mut INP_HOOKS: Vec<Box<dyn InpHook>> = Vec::new();
static mut INP2_HOOKS: Vec<Box<dyn Inp2Hook>> = Vec::new();
static mut OUTP_HOOKS: Vec<Box<dyn OutpHook>> = Vec::new();

pub trait LinAccessHook = FnMut(u32, Address, Address, usize, u32, MemAccess);
pub trait PhyAccessHook = FnMut(u32, Address, usize, u32, MemAccess);

static mut LIN_ACCESS_HOOKS: Vec<Box<dyn LinAccessHook>> = Vec::new();
static mut PHY_ACCESS_HOOKS: Vec<Box<dyn PhyAccessHook>> = Vec::new();

pub trait WrmsrHook = FnMut(u32, u32, u64);
static mut WRMSR_HOOKS: Vec<Box<dyn WrmsrHook>> = Vec::new();

pub trait VmexitHook = FnMut(u32, u32, u64);
static mut VMEXIT_HOOKS: Vec<Box<dyn VmexitHook>> = Vec::new();

//

pub unsafe fn init_env<T: InitEnvHook + 'static>(h: T) {
    INIT_ENV_HOOKS.push(Box::new(h))
}

pub unsafe fn init_env_clear() {
    INIT_ENV_HOOKS.clear()
}

pub unsafe fn exit_env<T: ExitEnvHook + 'static>(h: T) {
    EXIT_ENV_HOOKS.push(Box::new(h))
}

pub unsafe fn exit_env_clear() {
    EXIT_ENV_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_init_env() {
    unsafe { INIT_ENV_HOOKS.iter_mut().for_each(|x| x()) }
}
#[no_mangle]
extern "C" fn bx_instr_exit_env() {
    unsafe { EXIT_ENV_HOOKS.iter_mut().for_each(|x| x()) }
}

//

pub unsafe fn initialize<T: InitializeHook + 'static>(h: T) {
    INITIALIZE_HOOKS.push(Box::new(h))
}

pub unsafe fn initialize_clear() {
    INITIALIZE_HOOKS.clear()
}

pub unsafe fn exit<T: ExitHook + 'static>(h: T) {
    EXIT_HOOKS.push(Box::new(h))
}

pub unsafe fn exit_clear() {
    EXIT_HOOKS.clear()
}

pub unsafe fn reset<T: ResetHook + 'static>(h: T) {
    RESET_HOOKS.push(Box::new(h))
}

pub unsafe fn reset_clear() {
    RESET_HOOKS.clear()
}

pub unsafe fn hlt<T: HltHook + 'static>(h: T) {
    HLT_HOOKS.push(Box::new(h))
}

pub unsafe fn hlt_clear() {
    HLT_HOOKS.clear()
}

pub unsafe fn mwait<T: MwaitHook + 'static>(h: T) {
    MWAIT_HOOKS.push(Box::new(h))
}

pub unsafe fn mwait_clear() {
    MWAIT_HOOKS.clear()
}

#[no_mangle]
extern "C" fn bx_instr_initialize(cpu: u32) {
    unsafe {
        INITIALIZE_HOOKS.iter_mut().for_each(|x| x(cpu));

        // no setjmp state to restore to here...
    }
}
#[no_mangle]
extern "C" fn bx_instr_exit(cpu: u32) {
    unsafe {
        EXIT_HOOKS.iter_mut().for_each(|x| x(cpu));
    }
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
            .for_each(|x| x(cpu, addr, len, flags));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

//

pub unsafe fn cnear_branch_taken<T: CnearBranchTakenHook + 'static>(h: T) {
    CNEAR_BRANCH_TAKEN_HOOKS.push(Box::new(h))
}

pub unsafe fn cnear_branch_taken_clear() {
    CNEAR_BRANCH_TAKEN_HOOKS.clear()
}

pub unsafe fn cnear_branch_not_taken<T: CnearBranchNotTakenHook + 'static>(h: T) {
    CNEAR_BRANCH_NOT_TAKEN_HOOKS.push(Box::new(h))
}

pub unsafe fn cnear_branch_not_taken_clear() {
    CNEAR_BRANCH_NOT_TAKEN_HOOKS.clear()
}

pub unsafe fn ucnear_branch<T: UcnearBranchHook + 'static>(h: T) {
    UCNEAR_BRANCH_HOOKS.push(Box::new(h))
}

pub unsafe fn ucnear_branch_clear() {
    UCNEAR_BRANCH_HOOKS.clear()
}

pub unsafe fn far_branch<T: FarBranchHook + 'static>(h: T) {
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

pub unsafe fn opcode<T: OpcodeHook + 'static>(h: T) {
    OPCODE_HOOKS.push(Box::new(h))
}

pub unsafe fn opcode_clear() {
    OPCODE_HOOKS.clear()
}

pub unsafe fn interrupt<T: InterruptHook + 'static>(h: T) {
    INTERRUPT_HOOKS.push(Box::new(h))
}

pub unsafe fn interrupt_clear() {
    INTERRUPT_HOOKS.clear()
}

pub unsafe fn exception<T: ExceptionHook + 'static>(h: T) {
    EXCEPTION_HOOKS.push(Box::new(h))
}

pub unsafe fn exception_clear() {
    EXCEPTION_HOOKS.clear()
}

pub unsafe fn hw_interrupt<T: HwInterruptHook + 'static>(h: T) {
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

pub unsafe fn tlb_cntrl<T: TlbCntrlHook + 'static>(h: T) {
    TLB_CNTRL_HOOKS.push(Box::new(h))
}

pub unsafe fn tlb_cntrl_clear() {
    TLB_CNTRL_HOOKS.clear()
}

pub unsafe fn cache_cntrl<T: CacheCntrlHook + 'static>(h: T) {
    CACHE_CNTRL_HOOKS.push(Box::new(h))
}

pub unsafe fn cache_cntrl_clear() {
    CACHE_CNTRL_HOOKS.clear()
}

pub unsafe fn prefetch_hint<T: PrefetchHintHook + 'static>(h: T) {
    PREFETCH_HINT_HOOKS.push(Box::new(h))
}

pub unsafe fn prefetch_hint_clear() {
    PREFETCH_HINT_HOOKS.clear()
}

pub unsafe fn clflush<T: ClflushHook + 'static>(h: T) {
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
pub unsafe fn before_execution<T: BeforeExecutionHook + 'static>(h: T) {
    BEFORE_EXECUTION_HOOKS.push(Box::new(h))
}

pub unsafe fn before_execution_clear() {
    BEFORE_EXECUTION_HOOKS.clear()
}

pub unsafe fn after_execution<T: AfterExecutionHook + 'static>(h: T) {
    AFTER_EXECUTION_HOOKS.push(Box::new(h))
}

pub unsafe fn after_execution_clear() {
    AFTER_EXECUTION_HOOKS.clear()
}

pub unsafe fn repeat_iteration<T: RepeatIterationHook + 'static>(h: T) {
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

pub unsafe fn inp<T: InpHook + 'static>(h: T) {
    INP_HOOKS.push(Box::new(h))
}

pub unsafe fn inp_clear() {
    INP_HOOKS.clear()
}

pub unsafe fn inp2<T: Inp2Hook + 'static>(h: T) {
    INP2_HOOKS.push(Box::new(h))
}

pub unsafe fn inp2_clear() {
    INP2_HOOKS.clear()
}

pub unsafe fn outp<T: OutpHook + 'static>(h: T) {
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

pub unsafe fn lin_access<T: LinAccessHook + 'static>(h: T) {
    LIN_ACCESS_HOOKS.push(Box::new(h))
}

pub unsafe fn lin_access_clear() {
    LIN_ACCESS_HOOKS.clear()
}

pub unsafe fn phy_access<T: PhyAccessHook + 'static>(h: T) {
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

pub unsafe fn wrmsr<T: WrmsrHook + 'static>(h: T) {
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

pub unsafe fn vmexit<T: VmexitHook + 'static>(h: T) {
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
    init_env_clear();
    exit_env_clear();

    initialize_clear();
    exit_clear();
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
