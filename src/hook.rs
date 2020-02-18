use std::ffi::c_void;
use std::mem;
use std::slice;
use std::hint::unreachable_unchecked;

use crate::{Address, PhyAddress};
use crate::cpu::{cpu_bail, cpu_killbit};
use crate::syncunsafecell::SyncUnsafeCell;

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

pub trait ResetHook {
    fn reset(&self, _id: u32, _ty: u32) {}
}

pub trait HltHook {
    fn hlt(&self, _id: u32) {}
}

pub trait MwaitHook {
    fn mwait(&self, _id: u32, _addr: PhyAddress, _len: usize, _flags: u32) {}
}

pub trait CnearBranchTakenHook {
    fn cnear_branch_taken(&self, _id: u32, _branch_pc: Address, _new_pc: Address) {}
}

pub trait CnearBranchNotTakenHook {
    fn cnear_branch_not_taken(&self, _id: u32, _pc: Address) {}
}

pub trait UcnearBranchHook {
    fn ucnear_branch(&self, _id: u32, _what: Branch, _branch_pc: Address, _new_pc: Address) {}
}

pub trait FarBranchHook {
    fn far_branch(&self, _id: u32, _what: Branch, _branch_pc: (u16, Address), _new_pc: (u16, Address)) {}
}

pub trait OpcodeHook {
    fn opcode(&self, _id: u32, _ins: *mut c_void, _opcode: &[u8], _is_32: bool, _is_64: bool) {}
}

pub trait InterruptHook {
    fn interrupt(&self, _id: u32, _vector: u32) {}
}

pub trait ExceptionHook {
    fn exception(&self, _id: u32, _vector: u32, _error_code: u32) {}
}

pub trait HwInterruptHook {
    fn hw_interrupt(&self, _id: u32, _vector: u32, _pc: (u16, Address)) {}
}

pub trait TlbCntrlHook {
    fn tlb_cntrl(&self, _id: u32, _what: TlbCntrl, _new_cr: Option<PhyAddress>) {}
}

pub trait CacheCntrlHook {
    fn cache_cntrl(&self, _id: u32, _what: CacheCntrl) {}
}

pub trait PrefetchHintHook {
    fn prefetch_hint(&self, _id: u32, _what: PrefetchHint, _seg: u32, _off: Address) {}
}

pub trait ClflushHook {
    fn clflush(&self, _id: u32, _vaddr: Address, _paddr: PhyAddress) {}
}

pub trait BeforeExecutionHook {
    fn before_execution(&self, _id: u32, _ins: *mut c_void) {}
}

pub trait AfterExecutionHook {
    fn after_execution(&self, _id: u32, _ins: *mut c_void) {}
}

pub trait RepeatIterationHook {
    fn repeat_iteration(&self, _id: u32, _ins: *mut c_void) {}
}

pub trait InpHook {
    fn inp(&self, _addr: u16, _len: usize) {}
}

pub trait Inp2Hook {
    fn inp2(&self, _addr: u16, _len: usize, _val: u32) {}
}

pub trait OutpHook {
    fn outp(&self, _addr: u16, _len: usize, _val: u32) {}
}

pub trait LinAccessHook {
    fn lin_access(&self, _id: u32, _vaddr: Address, _paddr: Address, _len: usize, _memty: MemType, _rw: MemAccess) {}
}

pub trait PhyAccessHook {
    fn phy_access(&self, _id: u32, _paddr: PhyAddress, _len: usize, _memty: MemType, _rw: MemAccess) {}
}

pub trait WrmsrHook {
    fn wrmsr(&self, _id: u32, _msr: u32, _val: u64) {}
}

pub trait VmexitHook {
    fn vmexit(&self, _id: u32, _reason: u32, _qualification: u64) {}
}

static RESET_HOOKS: SyncUnsafeCell<Vec<& dyn ResetHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn reset_hooks() -> &'static mut Vec<&'static dyn ResetHook> {
    &mut *(RESET_HOOKS.0.get())
}

pub unsafe fn register_reset<'a>(h: &'a dyn ResetHook) {
    let hook = mem::transmute::<&'a dyn ResetHook, &'static dyn ResetHook>(h);
    reset_hooks().push(hook);
}

pub unsafe fn clear_reset() {
    reset_hooks().clear();
}

static HLT_HOOKS: SyncUnsafeCell<Vec<& dyn HltHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn hlt_hooks() -> &'static mut Vec<&'static dyn HltHook> {
    &mut *(HLT_HOOKS.0.get())
}

pub unsafe fn register_hlt<'a>(h: &'a dyn HltHook) {
    let hook = mem::transmute::<&'a dyn HltHook, &'static dyn HltHook>(h);
    hlt_hooks().push(hook);
}

pub unsafe fn clear_hlt() {
    hlt_hooks().clear();
}

static MWAIT_HOOKS: SyncUnsafeCell<Vec<& dyn MwaitHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn mwait_hooks() -> &'static mut Vec<&'static dyn MwaitHook> {
    &mut *(MWAIT_HOOKS.0.get())
}

pub unsafe fn register_mwait<'a>(h: &'a dyn MwaitHook) {
    let hook = mem::transmute::<&'a dyn MwaitHook, &'static dyn MwaitHook>(h);
    mwait_hooks().push(hook);
}

pub unsafe fn clear_mwait() {
    mwait_hooks().clear();
}

static CNEAR_BRANCH_TAKEN_HOOKS: SyncUnsafeCell<Vec<& dyn CnearBranchTakenHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn cnear_branch_taken_hooks() -> &'static mut Vec<&'static dyn CnearBranchTakenHook> {
    &mut *(CNEAR_BRANCH_TAKEN_HOOKS.0.get())
}

pub unsafe fn register_cnear_branch_taken<'a>(h: &'a dyn CnearBranchTakenHook) {
    let hook = mem::transmute::<&'a dyn CnearBranchTakenHook, &'static dyn CnearBranchTakenHook>(h);
    cnear_branch_taken_hooks().push(hook);
}

pub unsafe fn clear_cnear_branch_taken() {
    cnear_branch_taken_hooks().clear();
}

static CNEAR_BRANCH_NOT_TAKEN_HOOKS: SyncUnsafeCell<Vec<& dyn CnearBranchNotTakenHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn cnear_branch_not_taken_hooks() -> &'static mut Vec<&'static dyn CnearBranchNotTakenHook> {
    &mut *(CNEAR_BRANCH_NOT_TAKEN_HOOKS.0.get())
}

pub unsafe fn register_cnear_branch_not_taken<'a>(h: &'a dyn CnearBranchNotTakenHook) {
    let hook = mem::transmute::<&'a dyn CnearBranchNotTakenHook, &'static dyn CnearBranchNotTakenHook>(h);
    cnear_branch_not_taken_hooks().push(hook);
}

pub unsafe fn clear_cnear_branch_not_taken() {
    cnear_branch_not_taken_hooks().clear();
}

static UCNEAR_BRANCH_HOOKS: SyncUnsafeCell<Vec<& dyn UcnearBranchHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn ucnear_branch_hooks() -> &'static mut Vec<&'static dyn UcnearBranchHook> {
    &mut *(UCNEAR_BRANCH_HOOKS.0.get())
}

pub unsafe fn register_ucnear_branch<'a>(h: &'a dyn UcnearBranchHook) {
    let hook = mem::transmute::<&'a dyn UcnearBranchHook, &'static dyn UcnearBranchHook>(h);
    ucnear_branch_hooks().push(hook);
}

pub unsafe fn clear_ucnear_branch() {
    ucnear_branch_hooks().clear();
}

static FAR_BRANCH_HOOKS: SyncUnsafeCell<Vec<& dyn FarBranchHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn far_branch_hooks() -> &'static mut Vec<&'static dyn FarBranchHook> {
    &mut *(FAR_BRANCH_HOOKS.0.get())
}

pub unsafe fn register_far_branch<'a>(h: &'a dyn FarBranchHook) {
    let hook = mem::transmute::<&'a dyn FarBranchHook, &'static dyn FarBranchHook>(h);
    far_branch_hooks().push(hook);
}

pub unsafe fn clear_far_branch() {
    far_branch_hooks().clear();
}

static OPCODE_HOOKS: SyncUnsafeCell<Vec<& dyn OpcodeHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn opcode_hooks() -> &'static mut Vec<&'static dyn OpcodeHook> {
    &mut *(OPCODE_HOOKS.0.get())
}

pub unsafe fn register_opcode<'a>(h: &'a dyn OpcodeHook) {
    let hook = mem::transmute::<&'a dyn OpcodeHook, &'static dyn OpcodeHook>(h);
    opcode_hooks().push(hook);
}

pub unsafe fn clear_opcode() {
    opcode_hooks().clear();
}

static INTERRUPT_HOOKS: SyncUnsafeCell<Vec<& dyn InterruptHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn interrupt_hooks() -> &'static mut Vec<&'static dyn InterruptHook> {
    &mut *(INTERRUPT_HOOKS.0.get())
}

pub unsafe fn register_interrupt<'a>(h: &'a dyn InterruptHook) {
    let hook = mem::transmute::<&'a dyn InterruptHook, &'static dyn InterruptHook>(h);
    interrupt_hooks().push(hook);
}

pub unsafe fn clear_interrupt() {
    interrupt_hooks().clear();
}

static EXCEPTION_HOOKS: SyncUnsafeCell<Vec<& dyn ExceptionHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn exception_hooks() -> &'static mut Vec<&'static dyn ExceptionHook> {
    &mut *(EXCEPTION_HOOKS.0.get())
}

pub unsafe fn register_exception<'a>(h: &'a dyn ExceptionHook) {
    let hook = mem::transmute::<&'a dyn ExceptionHook, &'static dyn ExceptionHook>(h);
    exception_hooks().push(hook);
}

pub unsafe fn clear_exception() {
    exception_hooks().clear();
}

static HW_INTERRUPT_HOOKS: SyncUnsafeCell<Vec<& dyn HwInterruptHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn hw_interrupt_hooks() -> &'static mut Vec<&'static dyn HwInterruptHook> {
    &mut *(HW_INTERRUPT_HOOKS.0.get())
}

pub unsafe fn register_hw_interrupt<'a>(h: &'a dyn HwInterruptHook) {
    let hook = mem::transmute::<&'a dyn HwInterruptHook, &'static dyn HwInterruptHook>(h);
    hw_interrupt_hooks().push(hook);
}

pub unsafe fn clear_hw_interrupt() {
    hw_interrupt_hooks().clear();
}

static TLB_CNTRL_HOOKS: SyncUnsafeCell<Vec<& dyn TlbCntrlHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn tlb_cntrl_hooks() -> &'static mut Vec<&'static dyn TlbCntrlHook> {
    &mut *(TLB_CNTRL_HOOKS.0.get())
}

pub unsafe fn register_tlb_cntrl<'a>(h: &'a dyn TlbCntrlHook) {
    let hook = mem::transmute::<&'a dyn TlbCntrlHook, &'static dyn TlbCntrlHook>(h);
    tlb_cntrl_hooks().push(hook);
}

pub unsafe fn clear_tlb_cntrl() {
    tlb_cntrl_hooks().clear();
}

static CACHE_CNTRL_HOOKS: SyncUnsafeCell<Vec<& dyn CacheCntrlHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn cache_cntrl_hooks() -> &'static mut Vec<&'static dyn CacheCntrlHook> {
    &mut *(CACHE_CNTRL_HOOKS.0.get())
}

pub unsafe fn register_cache_cntrl<'a>(h: &'a dyn CacheCntrlHook) {
    let hook = mem::transmute::<&'a dyn CacheCntrlHook, &'static dyn CacheCntrlHook>(h);
    cache_cntrl_hooks().push(hook);
}

pub unsafe fn clear_cache_cntrl() {
    cache_cntrl_hooks().clear();
}

static PREFETCH_HINT_HOOKS: SyncUnsafeCell<Vec<& dyn PrefetchHintHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn prefetch_hint_hooks() -> &'static mut Vec<&'static dyn PrefetchHintHook> {
    &mut *(PREFETCH_HINT_HOOKS.0.get())
}

pub unsafe fn register_prefetch_hint<'a>(h: &'a dyn PrefetchHintHook) {
    let hook = mem::transmute::<&'a dyn PrefetchHintHook, &'static dyn PrefetchHintHook>(h);
    prefetch_hint_hooks().push(hook);
}

pub unsafe fn clear_prefetch_hint() {
    prefetch_hint_hooks().clear();
}

static CLFLUSH_HOOKS: SyncUnsafeCell<Vec<& dyn ClflushHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn clflush_hooks() -> &'static mut Vec<&'static dyn ClflushHook> {
    &mut *(CLFLUSH_HOOKS.0.get())
}

pub unsafe fn register_clflush<'a>(h: &'a dyn ClflushHook) {
    let hook = mem::transmute::<&'a dyn ClflushHook, &'static dyn ClflushHook>(h);
    clflush_hooks().push(hook);
}

pub unsafe fn clear_clflush() {
    clflush_hooks().clear();
}

static BEFORE_EXECUTION_HOOKS: SyncUnsafeCell<Vec<& dyn BeforeExecutionHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn before_execution_hooks() -> &'static mut Vec<&'static dyn BeforeExecutionHook> {
    &mut *(BEFORE_EXECUTION_HOOKS.0.get())
}

pub unsafe fn register_before_execution<'a>(h: &'a dyn BeforeExecutionHook) {
    let hook = mem::transmute::<&'a dyn BeforeExecutionHook, &'static dyn BeforeExecutionHook>(h);
    before_execution_hooks().push(hook);
}

pub unsafe fn clear_before_execution() {
    before_execution_hooks().clear();
}

static AFTER_EXECUTION_HOOKS: SyncUnsafeCell<Vec<& dyn AfterExecutionHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn after_execution_hooks() -> &'static mut Vec<&'static dyn AfterExecutionHook> {
    &mut *(AFTER_EXECUTION_HOOKS.0.get())
}

pub unsafe fn register_after_execution<'a>(h: &'a dyn AfterExecutionHook) {
    let hook = mem::transmute::<&'a dyn AfterExecutionHook, &'static dyn AfterExecutionHook>(h);
    after_execution_hooks().push(hook);
}

pub unsafe fn clear_after_execution() {
    after_execution_hooks().clear();
}

static REPEAT_ITERATION_HOOKS: SyncUnsafeCell<Vec<& dyn RepeatIterationHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn repeat_iteration_hooks() -> &'static mut Vec<&'static dyn RepeatIterationHook> {
    &mut *(REPEAT_ITERATION_HOOKS.0.get())
}

pub unsafe fn register_repeat_iteration<'a>(h: &'a dyn RepeatIterationHook) {
    let hook = mem::transmute::<&'a dyn RepeatIterationHook, &'static dyn RepeatIterationHook>(h);
    repeat_iteration_hooks().push(hook);
}

pub unsafe fn clear_repeat_iteration() {
    repeat_iteration_hooks().clear();
}

static INP_HOOKS: SyncUnsafeCell<Vec<& dyn InpHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn inp_hooks() -> &'static mut Vec<&'static dyn InpHook> {
    &mut *(INP_HOOKS.0.get())
}

pub unsafe fn register_inp<'a>(h: &'a dyn InpHook) {
    let hook = mem::transmute::<&'a dyn InpHook, &'static dyn InpHook>(h);
    inp_hooks().push(hook);
}

pub unsafe fn clear_inp() {
    inp_hooks().clear();
}

static INP2_HOOKS: SyncUnsafeCell<Vec<& dyn Inp2Hook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn inp2_hooks() -> &'static mut Vec<&'static dyn Inp2Hook> {
    &mut *(INP2_HOOKS.0.get())
}

pub unsafe fn register_inp2<'a>(h: &'a dyn Inp2Hook) {
    let hook = mem::transmute::<&'a dyn Inp2Hook, &'static dyn Inp2Hook>(h);
    inp2_hooks().push(hook);
}

pub unsafe fn clear_inp2() {
    inp2_hooks().clear();
}

static OUTP_HOOKS: SyncUnsafeCell<Vec<& dyn OutpHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn outp_hooks() -> &'static mut Vec<&'static dyn OutpHook> {
    &mut *(OUTP_HOOKS.0.get())
}

pub unsafe fn register_outp<'a>(h: &'a dyn OutpHook) {
    let hook = mem::transmute::<&'a dyn OutpHook, &'static dyn OutpHook>(h);
    outp_hooks().push(hook);
}

pub unsafe fn clear_outp() {
    outp_hooks().clear();
}

static LIN_ACCESS_HOOKS: SyncUnsafeCell<Vec<& dyn LinAccessHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn lin_access_hooks() -> &'static mut Vec<&'static dyn LinAccessHook> {
    &mut *(LIN_ACCESS_HOOKS.0.get())
}

pub unsafe fn register_lin_access<'a>(h: &'a dyn LinAccessHook) {
    let hook = mem::transmute::<&'a dyn LinAccessHook, &'static dyn LinAccessHook>(h);
    lin_access_hooks().push(hook);
}

pub unsafe fn clear_lin_access() {
    lin_access_hooks().clear();
}

static PHY_ACCESS_HOOKS: SyncUnsafeCell<Vec<& dyn PhyAccessHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn phy_access_hooks() -> &'static mut Vec<&'static dyn PhyAccessHook> {
    &mut *(PHY_ACCESS_HOOKS.0.get())
}

pub unsafe fn register_phy_access<'a>(h: &'a dyn PhyAccessHook) {
    let hook = mem::transmute::<&'a dyn PhyAccessHook, &'static dyn PhyAccessHook>(h);
    phy_access_hooks().push(hook);
}

pub unsafe fn clear_phy_access() {
    phy_access_hooks().clear();
}

static WRMSR_HOOKS: SyncUnsafeCell<Vec<& dyn WrmsrHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn wrmsr_hooks() -> &'static mut Vec<&'static dyn WrmsrHook> {
    &mut *(WRMSR_HOOKS.0.get())
}

pub unsafe fn register_wrmsr<'a>(h: &'a dyn WrmsrHook) {
    let hook = mem::transmute::<&'a dyn WrmsrHook, &'static dyn WrmsrHook>(h);
    wrmsr_hooks().push(hook);
}

pub unsafe fn clear_wrmsr() {
    wrmsr_hooks().clear();
}

static VMEXIT_HOOKS: SyncUnsafeCell<Vec<& dyn VmexitHook>> = SyncUnsafeCell::new(Vec::new());

unsafe fn vmexit_hooks() -> &'static mut Vec<&'static dyn VmexitHook> {
    &mut *(VMEXIT_HOOKS.0.get())
}

pub unsafe fn register_vmexit<'a>(h: &'a dyn VmexitHook) {
    let hook = mem::transmute::<&'a dyn VmexitHook, &'static dyn VmexitHook>(h);
    vmexit_hooks().push(hook);
}

pub unsafe fn clear_vmexit() {
    vmexit_hooks().clear();
}


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

#[no_mangle]
unsafe extern "C" fn bx_instr_reset(cpu: u32, ty: u32) {
    reset_hooks().iter_mut().for_each(|x| x.reset(cpu, ty));

    // avoid the overhead of calling Cpu::from and just check the raw flags
    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_hlt(cpu: u32) {
    hlt_hooks().iter_mut().for_each(|x| x.hlt(cpu));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_mwait(cpu: u32, addr: PhyAddress, len: u32, flags: u32) {
    mwait_hooks()
        .iter_mut()
        .for_each(|x| x.mwait(cpu, addr, len as usize, flags));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_cnear_branch_taken(cpu: u32, branch_eip: Address, new_eip: Address) {
    cnear_branch_taken_hooks()
        .iter_mut()
        .for_each(|x| x.cnear_branch_taken(cpu, branch_eip, new_eip));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
#[no_mangle]
unsafe extern "C" fn bx_instr_cnear_branch_not_taken(cpu: u32, branch_eip: Address) {
    cnear_branch_not_taken_hooks()
        .iter_mut()
        .for_each(|x| x.cnear_branch_not_taken(cpu, branch_eip));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
#[no_mangle]
unsafe extern "C" fn bx_instr_ucnear_branch(cpu: u32, what: u32, branch_eip: Address, new_eip: Address) {
    ucnear_branch_hooks()
        .iter_mut()
        .for_each(|x| x.ucnear_branch(cpu, what.into(), branch_eip, new_eip));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
#[no_mangle]
unsafe extern "C" fn bx_instr_far_branch(
    cpu: u32,
    what: u32,
    prev_cs: u16,
    prev_eip: Address,
    new_cs: u16,
    new_eip: Address,
) {
    far_branch_hooks()
        .iter_mut()
        .for_each(|x| x.far_branch(cpu, what.into(), (prev_cs, prev_eip), (new_cs, new_eip)));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_opcode(
    cpu: u32,
    i: *mut c_void,
    opcode: *const u8,
    len: u32,
    is32: u32,
    is64: u32,
) {
    opcode_hooks().iter_mut().for_each(|x| {
        x.opcode(
            cpu,
            i as *mut _ as *mut c_void,
            slice::from_raw_parts(opcode, len as usize),
            is32 != 0,
            is64 != 0,
        )
    });

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_interrupt(cpu: u32, vector: u32) {
    interrupt_hooks().iter_mut().for_each(|x| x.interrupt(cpu, vector));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_exception(cpu: u32, vector: u32, error_code: u32) {
    exception_hooks()
        .iter_mut()
        .for_each(|x| x.exception(cpu, vector, error_code));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
#[no_mangle]
unsafe extern "C" fn bx_instr_hwinterrupt(cpu: u32, vector: u32, cs: u16, eip: Address) {
    hw_interrupt_hooks()
        .iter_mut()
        .for_each(|x| x.hw_interrupt(cpu, vector, (cs, eip)));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
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
        tlb_cntrl_hooks()
            .iter_mut()
            .for_each(|x| x.tlb_cntrl(cpu, ty, maybe_cr3));

        if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
    }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_cache_cntrl(cpu: u32, what: u32) {
    cache_cntrl_hooks()
        .iter_mut()
        .for_each(|x| x.cache_cntrl(cpu, what.into()));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_prefetch_hint(cpu: u32, what: u32, seg: u32, offset: Address) {
    prefetch_hint_hooks()
        .iter_mut()
        .for_each(|x| x.prefetch_hint(cpu, what.into(), seg, offset));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_clflush(cpu: u32, laddr: Address, paddr: PhyAddress) {
    clflush_hooks().iter_mut().for_each(|x| x.clflush(cpu, laddr, paddr));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_before_execution(cpu: u32, i: *mut c_void) {
    before_execution_hooks().iter_mut().for_each(|x| x.before_execution(cpu, i));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
#[no_mangle]
unsafe extern "C" fn bx_instr_after_execution(cpu: u32, i: *mut c_void) {
    after_execution_hooks().iter_mut().for_each(|x| x.after_execution(cpu, i));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_repeat_iteration(cpu: u32, i: *mut c_void) {
    repeat_iteration_hooks().iter_mut().for_each(|x| x.repeat_iteration(cpu, i));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_lin_access(
    cpu: u32,
    lin: Address,
    phy: Address,
    len: u32,
    memtype: u32,
    rw: u32,
) {
    lin_access_hooks()
        .iter_mut()
        .for_each(|x| x.lin_access(cpu, lin, phy, len as usize, memtype.into(), rw.into()));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_phy_access(cpu: u32, phy: Address, len: u32, memtype: u32, rw: u32) {
    phy_access_hooks()
        .iter_mut()
        .for_each(|x| x.phy_access(cpu, phy, len as usize, memtype.into(), rw.into()));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}


#[no_mangle]
unsafe extern "C" fn bx_instr_inp(addr: u16, len: u32) {
    inp_hooks().iter_mut().for_each(|x| x.inp(addr, len as usize));
}
#[no_mangle]
unsafe extern "C" fn bx_instr_inp2(addr: u16, len: u32, val: u32) {
    inp2_hooks()
        .iter_mut()
        .for_each(|x| x.inp2(addr, len as usize, val));
}
#[no_mangle]
unsafe extern "C" fn bx_instr_outp(addr: u16, len: u32, val: u32) {
    outp_hooks()
        .iter_mut()
        .for_each(|x| x.outp(addr, len as usize, val));
}


#[no_mangle]
unsafe extern "C" fn bx_instr_wrmsr(cpu: u32, addr: u32, value: u64) {
    wrmsr_hooks().iter_mut().for_each(|x| x.wrmsr(cpu, addr, value));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}

#[no_mangle]
unsafe extern "C" fn bx_instr_vmexit(cpu: u32, reason: u32, qualification: u64) {
    vmexit_hooks()
        .iter_mut()
        .for_each(|x| x.vmexit(cpu, reason, qualification));

    if cpu_killbit(cpu) != 0 { cpu_bail(cpu) }
}
