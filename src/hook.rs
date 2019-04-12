use std::ffi::c_void;
use std::slice;
use unreachable::unreachable;

use crate::{Address, PhyAddress};

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
            _ => unsafe { unreachable() },
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
            _ => unsafe { unreachable() }
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
            _ => unsafe { unreachable() },
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
            _ => unsafe { unreachable() },
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
            _ => unsafe { unreachable() },
        }
    }
}

pub trait InitEnvHook = FnMut();
pub trait ExitEnvHook = FnMut();

static mut INIT_ENV_HOOKS: Vec<Box<InitEnvHook>> = Vec::new();
static mut EXIT_ENV_HOOKS: Vec<Box<ExitEnvHook>> = Vec::new();

pub trait InitializeHook = FnMut(u32);
pub trait ExitHook = FnMut(u32);
pub trait ResetHook = FnMut(u32, u32);
pub trait HltHook = FnMut(u32);
pub trait MwaitHook = FnMut(u32, PhyAddress, u32, u32);

static mut INITIALIZE_HOOKS: Vec<Box<InitializeHook>> = Vec::new();
static mut EXIT_HOOKS: Vec<Box<ExitHook>> = Vec::new();
static mut RESET_HOOKS: Vec<Box<ResetHook>> = Vec::new();
static mut HLT_HOOKS: Vec<Box<HltHook>> = Vec::new();
static mut MWAIT_HOOKS: Vec<Box<MwaitHook>> = Vec::new();

pub trait CnearBranchTakenHook = FnMut(u32, Address, Address);
pub trait CnearBranchNotTakenHook = FnMut(u32, Address);
pub trait UcnearBranchHook = FnMut(u32, u32, Address, Address);
pub trait FarBranchHook = FnMut(u32, Branch, (u16, Address), (u16, Address));

static mut CNEAR_BRANCH_TAKEN_HOOKS: Vec<Box<CnearBranchTakenHook>> = Vec::new();
static mut CNEAR_BRANCH_NOT_TAKEN_HOOKS: Vec<Box<CnearBranchNotTakenHook>> = Vec::new();
static mut UCNEAR_BRANCH_HOOKS: Vec<Box<UcnearBranchHook>> = Vec::new();
static mut FAR_BRANCH_HOOKS: Vec<Box<FarBranchHook>> = Vec::new();

pub trait OpcodeHook = FnMut(u32, *mut c_void, &[u8], bool, bool);
pub trait InterruptHook = FnMut(u32, u32);
pub trait ExceptionHook = FnMut(u32, u32, u32);
pub trait HwInterruptHook = FnMut(u32, u32, (u16, Address));

static mut OPCODE_HOOKS: Vec<Box<OpcodeHook>> = Vec::new();
static mut INTERRUPT_HOOKS: Vec<Box<InterruptHook>> = Vec::new();
static mut EXCEPTION_HOOKS: Vec<Box<ExceptionHook>> = Vec::new();
static mut HW_INTERRUPT_HOOKS: Vec<Box<HwInterruptHook>> = Vec::new();

pub trait TlbCntrlHook = FnMut(u32, TlbCntrl, Option<PhyAddress>);
pub trait CacheCntrlHook = FnMut(u32, CacheCntrl);
pub trait PrefetchHintHook = FnMut(u32, PrefetchHint, u32, Address);
pub trait ClflushHook = FnMut(u32, Address, PhyAddress);

static mut TLB_CNTRL_HOOKS: Vec<Box<TlbCntrlHook>> = Vec::new();
static mut CACHE_CNTRL_HOOKS: Vec<Box<CacheCntrlHook>> = Vec::new();
static mut PREFETCH_HINT_HOOKS: Vec<Box<PrefetchHintHook>> = Vec::new();
static mut CLFLUSH_HOOKS: Vec<Box<ClflushHook>> = Vec::new();

pub trait BeforeExecutionHook = FnMut(u32, *mut c_void);
pub trait AfterExecutionHook = FnMut(u32, *mut c_void);
pub trait RepeatIterationHook = FnMut(u32, *mut c_void);

static mut BEFORE_EXECUTION_HOOKS: Vec<Box<BeforeExecutionHook>> = Vec::new();
static mut AFTER_EXECUTION_HOOKS: Vec<Box<AfterExecutionHook>> = Vec::new();
static mut REPEAT_ITERATION_HOOKS: Vec<Box<RepeatIterationHook>> = Vec::new();

pub trait InpHook = FnMut(u16, usize);
pub trait Inp2Hook = FnMut(u16, usize, u32);
pub trait OutpHook = FnMut(u16, usize, u32);

static mut INP_HOOKS: Vec<Box<InpHook>> = Vec::new();
static mut INP2_HOOKS: Vec<Box<Inp2Hook>> = Vec::new();
static mut OUTP_HOOKS: Vec<Box<OutpHook>> = Vec::new();

pub trait LinAccessHook = FnMut(u32, Address, Address, usize, u32, MemAccess);
pub trait PhyAccessHook = FnMut(u32, Address, usize, u32, MemAccess);

static mut LIN_ACCESS_HOOKS: Vec<Box<LinAccessHook>> = Vec::new();
static mut PHY_ACCESS_HOOKS: Vec<Box<PhyAccessHook>> = Vec::new();

pub trait WrmsrHook = FnMut(u32, u32, u64);
static mut WRMSR_HOOKS:  Vec<Box<WrmsrHook>> = Vec::new();

pub trait VmexitHook = FnMut(u32, u32, u64);
static mut VMEXIT_HOOKS: Vec<Box<VmexitHook>> = Vec::new();

//

pub unsafe fn init_env<T: InitEnvHook + 'static>(h: T) {
    INIT_ENV_HOOKS.push(Box::new(h))
}
pub unsafe fn exit_env<T: ExitEnvHook + 'static>(h: T) {
    EXIT_ENV_HOOKS.push(Box::new(h))
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
pub unsafe fn exit<T: ExitHook + 'static>(h: T) {
    EXIT_HOOKS.push(Box::new(h))
}
pub unsafe fn reset<T: ResetHook + 'static>(h: T) {
    RESET_HOOKS.push(Box::new(h))
}
pub unsafe fn hlt<T: HltHook + 'static>(h: T) {
    HLT_HOOKS.push(Box::new(h))
}
pub unsafe fn mwait<T: MwaitHook + 'static>(h: T) {
    MWAIT_HOOKS.push(Box::new(h))
}

#[no_mangle]
extern "C" fn bx_instr_initialize(cpu: u32) {
    unsafe { INITIALIZE_HOOKS.iter_mut().for_each(|x| x(cpu)) }
}
#[no_mangle]
extern "C" fn bx_instr_exit(cpu: u32) {
    unsafe { EXIT_HOOKS.iter_mut().for_each(|x| x(cpu)) }
}
#[no_mangle]
extern "C" fn bx_instr_reset(cpu: u32, ty: u32) {
    unsafe { RESET_HOOKS.iter_mut().for_each(|x| x(cpu, ty)) }

}
#[no_mangle]
extern "C" fn bx_instr_hlt(cpu: u32) {
    unsafe { HLT_HOOKS.iter_mut().for_each(|x| x(cpu)) }
}
#[no_mangle]
extern "C" fn bx_instr_mwait(cpu: u32, addr: PhyAddress, len: u32, flags: u32) {
    unsafe { MWAIT_HOOKS.iter_mut().for_each(|x| x(cpu, addr, len, flags)) }
}

//

pub unsafe fn cnear_branch_taken<T: CnearBranchTakenHook + 'static>(h: T) {
    CNEAR_BRANCH_TAKEN_HOOKS.push(Box::new(h))
}
pub unsafe fn cnear_branch_not_taken<T: CnearBranchNotTakenHook + 'static>(h: T) {
    CNEAR_BRANCH_NOT_TAKEN_HOOKS.push(Box::new(h))
}
pub unsafe fn ucnear_branch<T: UcnearBranchHook + 'static>(h: T) {
    UCNEAR_BRANCH_HOOKS.push(Box::new(h))
}
pub unsafe fn far_branch<T: FarBranchHook + 'static>(h: T) {
    FAR_BRANCH_HOOKS.push(Box::new(h))
}

#[no_mangle]
extern "C" fn bx_instr_cnear_branch_taken(cpu: u32, branch_eip: Address, new_eip: Address) {
    unsafe { CNEAR_BRANCH_TAKEN_HOOKS.iter_mut().for_each(|x| x(cpu, branch_eip, new_eip)) }
}
#[no_mangle]
extern "C" fn bx_instr_cnear_branch_not_taken(cpu: u32, branch_eip: Address) {
    unsafe { CNEAR_BRANCH_NOT_TAKEN_HOOKS.iter_mut().for_each(|x| x(cpu, branch_eip)) }
}
#[no_mangle]
extern "C" fn bx_instr_ucnear_branch(cpu: u32, what: u32, branch_eip: Address, new_eip: Address) {
    unsafe { UCNEAR_BRANCH_HOOKS.iter_mut().for_each(|x| x(cpu, what, branch_eip, new_eip)) }
}
#[no_mangle]
extern "C" fn bx_instr_far_branch(
    cpu: u32,
    what: u32,
    prev_cs: u16,
    prev_eip: Address,
    new_cs: u16,
    new_eip: Address)
{
    unsafe {
        FAR_BRANCH_HOOKS.iter_mut().for_each(
            |x| x(cpu, what.into(), (prev_cs, prev_eip), (new_cs, new_eip))
        )
    }
}

//
//
pub unsafe fn opcode<T: OpcodeHook + 'static>(h: T) {
    OPCODE_HOOKS.push(Box::new(h))
}
pub unsafe fn interrupt<T: InterruptHook + 'static>(h: T) {
    INTERRUPT_HOOKS.push(Box::new(h))
}
pub unsafe fn exception<T: ExceptionHook + 'static>(h: T) {
    EXCEPTION_HOOKS.push(Box::new(h))
}
pub unsafe fn hw_interrupt<T: HwInterruptHook + 'static>(h: T) {
    HW_INTERRUPT_HOOKS.push(Box::new(h))
}
#[no_mangle]
extern "C" fn bx_instr_opcode(cpu: u32, i: *mut c_void, opcode: *const u8, len: u32, is32: u32, is64: u32) {
    unsafe {
        OPCODE_HOOKS.iter_mut().for_each(
            |x| x(
                cpu,
                i as *mut _ as *mut c_void,
                slice::from_raw_parts(opcode, len as usize),
                is32 != 0,
                is64 != 0
            )
        )
    }
}
#[no_mangle]
extern "C" fn bx_instr_interrupt(cpu: u32, vector: u32) {
    unsafe { INTERRUPT_HOOKS.iter_mut().for_each(|x| x(cpu, vector)) }
}

#[no_mangle]
extern "C" fn bx_instr_exception(cpu: u32, vector: u32, error_code: u32) {
    unsafe { EXCEPTION_HOOKS.iter_mut().for_each(|x| x(cpu, vector, error_code)) }
}
#[no_mangle]
extern "C" fn bx_instr_hwinterrupt(cpu: u32, vector: u32, cs: u16, eip: Address) {
    unsafe { HW_INTERRUPT_HOOKS.iter_mut().for_each(|x| x(cpu, vector, (cs, eip))) }
}

//

pub unsafe fn tlb_cntrl<T:TlbCntrlHook + 'static>(h: T) {
    TLB_CNTRL_HOOKS.push(Box::new(h))
}
pub unsafe fn cache_cntrl<T: CacheCntrlHook + 'static>(h: T) {
    CACHE_CNTRL_HOOKS.push(Box::new(h))
}
pub unsafe fn prefetch_hint<T: PrefetchHintHook + 'static>(h: T) {
    PREFETCH_HINT_HOOKS.push(Box::new(h))
}
pub unsafe fn clflush<T: ClflushHook + 'static>(h: T) {
    CLFLUSH_HOOKS.push(Box::new(h))
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

    unsafe { TLB_CNTRL_HOOKS.iter_mut().for_each(|x| x(cpu, ty, maybe_cr3)) }
}
#[no_mangle]
extern "C" fn bx_instr_cache_cntrl(cpu: u32, what: u32) {
    unsafe { CACHE_CNTRL_HOOKS.iter_mut().for_each(|x| x(cpu, what.into())) }
}
#[no_mangle]
extern "C" fn bx_instr_prefetch_hint(cpu: u32, what: u32, seg: u32, offset: Address) {
    unsafe { PREFETCH_HINT_HOOKS.iter_mut().for_each(|x| x(cpu, what.into(), seg, offset)) }
}
#[no_mangle]
extern "C" fn bx_instr_clflush(cpu: u32, laddr: Address, paddr: PhyAddress) {
    unsafe { CLFLUSH_HOOKS.iter_mut().for_each(|x| x(cpu, laddr, paddr)) }
}

//

// You probably don't want this -- due to some as of yet unknown state that we
// dont restore this will sometimes be called twice for a single guest
// instruction. Use after_execution.
pub unsafe fn before_execution<T: BeforeExecutionHook + 'static>(h: T) {
    BEFORE_EXECUTION_HOOKS.push(Box::new(h))
}
pub unsafe fn after_execution<T: AfterExecutionHook + 'static>(h: T) {
    AFTER_EXECUTION_HOOKS.push(Box::new(h))
}
pub unsafe fn repeat_iteration<T: RepeatIterationHook + 'static>(h: T) {
    REPEAT_ITERATION_HOOKS.push(Box::new(h))
}
#[no_mangle]
extern "C" fn bx_instr_before_execution(cpu: u32, i: *mut c_void) {
    unsafe { BEFORE_EXECUTION_HOOKS.iter_mut().for_each(|x| x(cpu, i)) }
}
#[no_mangle]
extern "C" fn bx_instr_after_execution(cpu: u32, i: *mut c_void) {
    unsafe { AFTER_EXECUTION_HOOKS.iter_mut().for_each(|x| x(cpu, i)) }
}
#[no_mangle]
extern "C" fn bx_instr_repeat_iteration(cpu: u32, i: *mut c_void) {
    unsafe { REPEAT_ITERATION_HOOKS.iter_mut().for_each(|x| x(cpu, i)) }
}

//

pub unsafe fn inp<T: InpHook + 'static>(h: T) {
    INP_HOOKS.push(Box::new(h))
}
pub unsafe fn inp2<T: Inp2Hook + 'static>(h: T) {
    INP2_HOOKS.push(Box::new(h))
}
pub unsafe fn outp<T: OutpHook + 'static>(h: T) {
    OUTP_HOOKS.push(Box::new(h))
}

#[no_mangle]
extern "C" fn bx_instr_inp(addr: u16, len: u32) {
    unsafe { INP_HOOKS.iter_mut().for_each(|x| x(addr, len as usize)) }
}
#[no_mangle]
extern "C" fn bx_instr_inp2(addr: u16, len: u32, val: u32) {
    unsafe { INP2_HOOKS.iter_mut().for_each(|x| x(addr, len as usize, val)) }
}
#[no_mangle]
extern "C" fn bx_instr_outp(addr: u16, len: u32, val: u32) {
    unsafe { OUTP_HOOKS.iter_mut().for_each(|x| x(addr, len as usize, val)) }
}

//

pub unsafe fn lin_access<T: LinAccessHook + 'static>(h: T) {
    LIN_ACCESS_HOOKS.push(Box::new(h))
}
pub unsafe fn phy_access<T: PhyAccessHook + 'static>(h: T) {
    PHY_ACCESS_HOOKS.push(Box::new(h))
}
#[no_mangle]
extern "C" fn bx_instr_lin_access(cpu: u32, lin: Address, phy: Address, len: u32, memtype: u32, rw: u32) {
    unsafe { LIN_ACCESS_HOOKS.iter_mut().for_each(|x| x(cpu, lin, phy, len as usize, memtype, rw.into())) }
}
#[no_mangle]
extern "C" fn bx_instr_phy_access(cpu: u32, phy: Address, len: u32, memtype: u32, rw: u32) {
    unsafe { PHY_ACCESS_HOOKS.iter_mut().for_each(|x| x(cpu, phy, len as usize, memtype, rw.into())) }
}

//

pub unsafe fn wrmsr<T: WrmsrHook + 'static>(h: T) {
    WRMSR_HOOKS.push(Box::new(h))
}
#[no_mangle]
extern "C" fn bx_instr_wrmsr(cpu: u32, addr: u32, value: u64) {
    unsafe { WRMSR_HOOKS.iter_mut().for_each(|x| x(cpu, addr, value)) }
}

//

pub unsafe fn vmexit<T: VmexitHook + 'static>(h: T) {
    VMEXIT_HOOKS.push(Box::new(h))
}
#[no_mangle]
extern "C" fn bx_instr_vmexit(cpu: u32, reason: u32, qualification: u64) {
    unsafe { VMEXIT_HOOKS.iter_mut().for_each(|x| x(cpu, reason, qualification)) }
}

