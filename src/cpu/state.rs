pub struct State {
    rax: u64,
    rcx: u64,
    rdx: u64,
    rbx: u64,
    rsp: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rflags: u64,

    // seg regs

    // idtr + friends

    cr0: u32,
    cr2: u64,
    cr3: u64,
    cr4: u32,
    cr8: u64,

    dr0: u64,
    dr1: u64,
    dr2: u64,
    dr3: u64,
    dr6: u32,
    dr7: u32,

    xcr0: u32,

    xmm0: u128,
    xmm1: u128,
    xmm2: u128,
    xmm3: u128,
    xmm4: u128,
    xmm5: u128,
    xmm6: u128,
    xmm7: u128,
    xmm8: u128,
    xmm9: u128,
    xmm10: u128,
    xmm11: u128,
    xmm12: u128,
    xmm13: u128,
    xmm14: u128,
    xmm15: u128,

    // TODO fp regs
    // TODO fp control info

    fpcw: u16,
    fpsw: u16,
    fptw: u16,

    // TODO xmm control info

    mxcsr: u32,
    mxcsr_mask: u32,

    // TODO cmm control info

    tsc: u64,
    efer: u32,
    kernel_gs_base: u64,
    apic_base: u64,
    pat: u64,
    sysenter_cs: u64,
    sysenter_eip: u64,
    sysenter_esp: u64,
    star: u64,
    lstar: u64,
    cstar: u64,
    sfmask: u64,
    tsc_aux: u64,
}
