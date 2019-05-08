use serde::{Deserialize};

use crate::cpu::{GlobalSeg, Seg, Zmm};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Deserialize)]
pub struct State {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,

    pub es: Seg,
    pub cs: Seg,
    pub ss: Seg,
    pub ds: Seg,
    pub fs: Seg,
    pub gs: Seg,

    pub ldtr: Seg,
    pub tr: Seg,
    pub gdtr: GlobalSeg,
    pub idtr: GlobalSeg,

    pub cr0: u32,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u32,
    pub cr8: u64,

    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr6: u32,
    pub dr7: u32,

    pub xcr0: u32,

    pub zmm: [Zmm; 32],

    // TODO fp regs
    // TODO fp control info

    pub fpcw: u16,
    pub fpsw: u16,
    pub fptw: u16,
    pub fpop: u16,
    pub fpst: [u64; 8],

    // TODO xmm control info

    pub mxcsr: u32,
    pub mxcsr_mask: u32,

    // TODO cmm control info

    pub tsc: u64,
    pub efer: u32,
    pub kernel_gs_base: u64,
    pub apic_base: u64,
    pub pat: u64,
    pub sysenter_cs: u64,
    pub sysenter_eip: u64,
    pub sysenter_esp: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub tsc_aux: u64,
}
