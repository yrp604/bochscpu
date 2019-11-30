#include <cstdint>
#include <new>

#include "bochs.h"
#include "cpu/cpu.h"

typedef BX_CPU_C *BX_CPU_C_PTR;

extern "C" {
BOCHSAPI void cpu_loop(unsigned id) {
    BX_CPU(id)->cpu_loop();
}

BOCHSAPI void cpu_new(unsigned id) {
#if BX_SUPPORT_SMP
    // bochs assumes that all things are init'd to zero, which breaks ASan so
    // we use placement new to zero the mem
    void *zero = malloc(sizeof(BX_CPU_C));
    memset(zero, 0, sizeof(BX_CPU_C));

    BX_CPU_C *c = new (zero) BX_CPU_C(id);


    BX_CPU(id) = c;
#endif

    BX_CPU(id)->initialize();
    BX_CPU(id)->sanity_checks();

    BX_INSTR_INITIALIZE(id);
}

BOCHSAPI void cpu_delete(unsigned id) {
#if BX_SUPPORT_SMP

    BX_CPU(id)->~BX_CPU_C();
    free(BX_CPU(id));

    BX_CPU(id) = NULL;
#endif
}

BOCHSAPI void cpu_set_state(unsigned id) {
    BX_CPU_C *c = BX_CPU(id);

    c->TLB_flush();

#if BX_CPU_LEVEL >= 4
    c->handleAlignmentCheck(/* CR0.AC reloaded */);
#endif

    c->handleCpuModeChange();

#if BX_CPU_LEVEL >= 6
    c->handleSseModeChange();
#endif
}

// general purpose regs

BOCHSAPI bx_address cpu_get_pc(unsigned id) {
    return BX_CPU(id)->get_instruction_pointer();
}

BOCHSAPI void cpu_set_pc(unsigned id, Bit64u val) {
    BX_CPU(id)->gen_reg[BX_64BIT_REG_RIP].rrx = val;
    BX_CPU(id)->prev_rip = val;
}

BOCHSAPI void cpu_set_sp(unsigned id, Bit64u val) {
    BX_CPU(id)->gen_reg[BX_64BIT_REG_RSP].rrx = val;
    BX_CPU(id)->prev_rsp = val;
}

BOCHSAPI Bit64u cpu_get_reg64(unsigned id, unsigned reg) {
    return BX_CPU(id)->get_reg64(reg);
}

BOCHSAPI void cpu_set_reg64(unsigned id, unsigned reg, Bit64u val) {
    BX_CPU(id)->set_reg64(reg, val);
}

BOCHSAPI Bit32u cpu_get_eflags(unsigned id) {
    return BX_CPU(id)->eflags;
}

BOCHSAPI void cpu_set_eflags(unsigned id, Bit32u eflags) {
    BX_CPU(id)->setEFlags(eflags);
}

void get_seg(
        unsigned id,
        bx_segment_reg_t *seg,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    *present  = seg->cache.valid;
    *base     = seg->cache.u.segment.base;
    *limit    = seg->cache.u.segment.limit_scaled;
    *selector = seg->selector.value;
    *attr     = (BX_CPU(id)->get_descriptor_h(&seg->cache) >> 8) & 0xffff;
}

void set_seg(
        unsigned id,
        bx_segment_reg_t *seg,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    BX_CPU(id)->set_segment_ar_data(
            seg,
            present,
            selector,
            base,
            limit,
            attr
    );
}

BOCHSAPI void cpu_get_seg(
        unsigned id,
        unsigned sreg,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    return get_seg(id, &BX_CPU(id)->sregs[sreg], present, selector, base, limit, attr);
}

BOCHSAPI void cpu_set_seg(
        unsigned id,
        unsigned sreg,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    return set_seg(id, &BX_CPU(id)->sregs[sreg], present, selector, base, limit, attr);
}

BOCHSAPI void cpu_get_ldtr(
        unsigned id,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    return get_seg(id, &BX_CPU(id)->ldtr, present, selector, base, limit, attr);
}

BOCHSAPI void cpu_set_ldtr(
        unsigned id,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    return set_seg(id, &BX_CPU(id)->ldtr, present, selector, base, limit, attr);
}

BOCHSAPI void cpu_get_tr(
        unsigned id,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    return get_seg(id, &BX_CPU(id)->tr, present, selector, base, limit, attr);
}

BOCHSAPI void cpu_set_tr(
        unsigned id,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    return set_seg(id, &BX_CPU(id)->tr, present, selector, base, limit, attr);
}

BOCHSAPI void cpu_get_gdtr(unsigned id, bx_address *base, Bit16u *limit) {
    *base= BX_CPU(id)->gdtr.base;
    *limit = BX_CPU(id)->gdtr.limit;
}

BOCHSAPI void cpu_set_gdtr(unsigned id, bx_address base, Bit16u limit) {
    BX_CPU(id)->gdtr.base = base;
    BX_CPU(id)->gdtr.limit = limit;
}

BOCHSAPI void cpu_get_idtr(unsigned id, bx_address *base, Bit16u *limit) {
    *base= BX_CPU(id)->idtr.base;
    *limit = BX_CPU(id)->idtr.limit;
}

BOCHSAPI void cpu_set_idtr(unsigned id, bx_address base, Bit16u limit) {
    BX_CPU(id)->idtr.base = base;
    BX_CPU(id)->idtr.limit = limit;
}

// debug registers

BOCHSAPI bx_address cpu_get_dr(unsigned id, unsigned dr) {
    return BX_CPU(id)->dr[dr];
}

BOCHSAPI void cpu_set_dr(unsigned id, unsigned dr, bx_address v) {
    BX_CPU(id)->dr[dr] = v;
}

BOCHSAPI Bit32u cpu_get_dr6(unsigned id) {
    return BX_CPU(id)->dr6.get32();
}

BOCHSAPI void cpu_set_dr6(unsigned id, Bit32u v) {
    BX_CPU(id)->dr6.set32(v);
}

BOCHSAPI Bit32u cpu_get_dr7(unsigned id) {
    return BX_CPU(id)->dr7.get32();
}

BOCHSAPI void cpu_set_dr7(unsigned id, Bit32u v) {
    BX_CPU(id)->dr7.set32(v);
}

// control registers

BOCHSAPI Bit32u cpu_get_cr0(unsigned id) {
    return BX_CPU(id)->cr0.get32();
}

BOCHSAPI void cpu_set_cr0(unsigned id, Bit32u v) {
    BX_CPU(id)->cr0.set32(v);
}

BOCHSAPI bx_address cpu_get_cr2(unsigned id) {
    return BX_CPU(id)->cr2;
}

BOCHSAPI void cpu_set_cr2(unsigned id, bx_address v) {
    BX_CPU(id)->cr2 = v;
}

BOCHSAPI bx_address cpu_get_cr3(unsigned id) {
    return BX_CPU(id)->cr3;
}

BOCHSAPI void cpu_set_cr3(unsigned id, bx_address v) {
    BX_CPU(id)->cr3 = v;
}

BOCHSAPI Bit32u cpu_get_cr4(unsigned id) {
    return BX_CPU(id)->cr4.get32();
}

BOCHSAPI void cpu_set_cr4(unsigned id, Bit32u v) {
    BX_CPU(id)->cr4.set32(v);
}

BOCHSAPI Bit32u cpu_get_cr8(unsigned id) {
    return BX_CPU(id)->get_cr8();
}

BOCHSAPI void cpu_set_cr8(unsigned id, Bit32u v) {
    BX_CPU(id)->lapic.set_tpr((v & 0xf) << 4);
}

BOCHSAPI Bit32u cpu_get_efer(unsigned id) {
    return BX_CPU(id)->efer.get32();
}

BOCHSAPI void cpu_set_efer(unsigned id, Bit32u v) {
    BX_CPU(id)->efer.set32(v);
}

BOCHSAPI Bit32u cpu_get_xcr0(unsigned id) {
    return BX_CPU(id)->xcr0.get32();
}

BOCHSAPI void cpu_set_xcr0(unsigned id, Bit32u v) {
    BX_CPU(id)->xcr0.set32(v);
}

// model specific registers

BOCHSAPI Bit64u cpu_get_kernel_gs_base(unsigned id) {
    return BX_CPU(id)->msr.kernelgsbase;
}

BOCHSAPI void cpu_set_kernel_gs_base(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.kernelgsbase = v;
}

BOCHSAPI Bit32u cpu_get_sysenter_cs(unsigned id) {
    return BX_CPU(id)->msr.sysenter_cs_msr;
}

BOCHSAPI void cpu_set_sysenter_cs(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.sysenter_cs_msr = v;
}

BOCHSAPI bx_address cpu_get_sysenter_esp(unsigned id) {
    return BX_CPU(id)->msr.sysenter_esp_msr;
}

BOCHSAPI void cpu_set_sysenter_esp(unsigned id, bx_address v) {
    BX_CPU(id)->msr.sysenter_esp_msr = v;
}

BOCHSAPI bx_address cpu_get_sysenter_eip(unsigned id) {
    return BX_CPU(id)->msr.sysenter_eip_msr;
}

BOCHSAPI void cpu_set_sysenter_eip(unsigned id, bx_address v) {
    BX_CPU(id)->msr.sysenter_eip_msr = v;
}

BOCHSAPI Bit64u cpu_get_star(unsigned id) {
    return BX_CPU(id)->msr.star;
}

BOCHSAPI void cpu_set_star(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.star = v;
}

BOCHSAPI Bit64u cpu_get_lstar(unsigned id) {
    return BX_CPU(id)->msr.lstar;
}

BOCHSAPI void cpu_set_lstar(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.lstar = v;
}

BOCHSAPI Bit64u cpu_get_cstar(unsigned id) {
    return BX_CPU(id)->msr.cstar;
}

BOCHSAPI void cpu_set_cstar(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.cstar = v;
}

BOCHSAPI Bit32u cpu_get_fmask(unsigned id) {
    return BX_CPU(id)->msr.fmask;
}

BOCHSAPI void cpu_set_fmask(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.fmask = v;
}

BOCHSAPI Bit64u cpu_get_tsc(unsigned id) {
    return BX_CPU(id)->get_TSC();
}

BOCHSAPI void cpu_set_tsc(unsigned id, Bit64u v) {
    BX_CPU(id)->set_TSC(v);
}

BOCHSAPI Bit64u cpu_get_tsc_aux(unsigned id) {
    return BX_CPU(id)->msr.tsc_aux;
}

BOCHSAPI void cpu_set_tsc_aux(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.tsc_aux = v;
}

BOCHSAPI bx_phy_address cpu_get_apicbase(unsigned id) {
    return BX_CPU(id)->msr.apicbase;
}

BOCHSAPI void cpu_set_apicbase(unsigned id, bx_phy_address v) {
    BX_CPU(id)->msr.apicbase = v;
}

BOCHSAPI Bit64u cpu_get_pat(unsigned id) {
    return BX_CPU(id)->msr.pat._u64;
}

BOCHSAPI void cpu_set_pat(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.pat._u64 = v;
}



// ZMM

BOCHSAPI void cpu_get_zmm(unsigned id, unsigned reg, Bit64u z[]) {
#if BX_SUPPORT_EVEX
    z[0] = BX_CPU(id)->vmm[reg].zmm_u64[0];
    z[1] = BX_CPU(id)->vmm[reg].zmm_u64[1];
    z[2] = BX_CPU(id)->vmm[reg].zmm_u64[2];
    z[3] = BX_CPU(id)->vmm[reg].zmm_u64[3];
    z[4] = BX_CPU(id)->vmm[reg].zmm_u64[4];
    z[5] = BX_CPU(id)->vmm[reg].zmm_u64[5];
    z[6] = BX_CPU(id)->vmm[reg].zmm_u64[6];
    z[7] = BX_CPU(id)->vmm[reg].zmm_u64[7];
#elif BX_SUPPORT_AVX
    z[0] = BX_CPU(id)->vmm[reg].ymm_u64[0];
    z[1] = BX_CPU(id)->vmm[reg].ymm_u64[1];
    z[2] = BX_CPU(id)->vmm[reg].ymm_u64[2];
    z[3] = BX_CPU(id)->vmm[reg].ymm_u64[3];
    z[4] = 0;
    z[5] = 0;
    z[6] = 0;
    z[7] = 0;
#else
    z[0] = BX_CPU(id)->vmm[reg].xmm_u64[0];
    z[1] = BX_CPU(id)->vmm[reg].xmm_u64[1];
    z[2] = 0;
    z[3] = 0;
    z[4] = 0;
    z[5] = 0;
    z[6] = 0;
    z[7] = 0;
#endif
}

BOCHSAPI void cpu_set_zmm(unsigned id, unsigned reg, Bit64u z[]) {
#if BX_SUPPORT_EVEX
    BX_CPU(id)->vmm[reg].zmm_u64[0] = z[0];
    BX_CPU(id)->vmm[reg].zmm_u64[1] = z[1];
    BX_CPU(id)->vmm[reg].zmm_u64[2] = z[2];
    BX_CPU(id)->vmm[reg].zmm_u64[3] = z[3];
    BX_CPU(id)->vmm[reg].zmm_u64[4] = z[4];
    BX_CPU(id)->vmm[reg].zmm_u64[5] = z[5];
    BX_CPU(id)->vmm[reg].zmm_u64[6] = z[6];
    BX_CPU(id)->vmm[reg].zmm_u64[7] = z[7];
#elif BX_SUPPORT_AVX
    BX_CPU(id)->vmm[reg].ymm_u64[0] = z[0];
    BX_CPU(id)->vmm[reg].ymm_u64[1] = z[1];
    BX_CPU(id)->vmm[reg].ymm_u64[2] = z[2];
    BX_CPU(id)->vmm[reg].ymm_u64[3] = z[3];
#else
    BX_CPU(id)->vmm[reg].xmm_u64[0] = z[0];
    BX_CPU(id)->vmm[reg].xmm_u64[1] = z[1];
#endif
}

// FP registers

BOCHSAPI Bit16u cpu_get_fp_cw(unsigned id) {
    return BX_CPU(id)->the_i387.cwd;
}

BOCHSAPI void cpu_set_fp_cw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.cwd = v;
}

BOCHSAPI Bit16u cpu_get_fp_sw(unsigned id) {
    return BX_CPU(id)->the_i387.swd;
}

BOCHSAPI void cpu_set_fp_sw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.swd = v;
}

BOCHSAPI Bit16u cpu_get_fp_tw(unsigned id) {
    return BX_CPU(id)->the_i387.twd;
}

BOCHSAPI void cpu_set_fp_tw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.twd = v;
}

BOCHSAPI Bit16u cpu_get_fp_op(unsigned id) {
    return BX_CPU(id)->the_i387.foo;
}

BOCHSAPI void cpu_set_fp_op(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.foo = v;
}

BOCHSAPI Bit64u cpu_get_fp_st(unsigned id, unsigned reg) {
    float_status_t s;
    return (Bit64u)floatx80_to_int64(BX_CPU(id)->the_i387.st_space[reg], s);
}

BOCHSAPI void cpu_set_fp_st(unsigned id, unsigned reg, Bit64u v) {
    BX_CPU(id)->the_i387.st_space[reg] = int64_to_floatx80(v);
}

BOCHSAPI Bit32u cpu_get_mxcsr(unsigned id) {
    return BX_CPU(id)->mxcsr.mxcsr;
}

BOCHSAPI void cpu_set_mxcsr(unsigned id, Bit32u v) {
    BX_CPU(id)->mxcsr.mxcsr = v;
}

BOCHSAPI Bit32u cpu_get_mxcsr_mask(unsigned id) {
    return BX_CPU(id)->mxcsr_mask;
}

BOCHSAPI void cpu_set_mxcsr_mask(unsigned id, Bit32u v) {
    BX_CPU(id)->mxcsr_mask = v;
}

BOCHSAPI unsigned cpu_killbit(unsigned id) {
    return bx_pc_system.kill_bochs_request;
}

BOCHSAPI void cpu_set_killbit(unsigned id) {
    BX_CPU(id)->async_event = 1;
    bx_pc_system.kill_bochs_request = 1;
}

BOCHSAPI void cpu_clear_killbit(unsigned id) {
    BX_CPU(id)->async_event = 0;
    bx_pc_system.kill_bochs_request = 0;
}
}

Bit8u bx_cpu_count = 0xff; // max number of processsors

#if BX_SUPPORT_SMP
BOCHSAPI BX_CPU_C_PTR *bx_cpu_array = new BX_CPU_C_PTR[BX_SMP_PROCESSORS];
#else
BOCHSAPI BX_CPU_C bx_cpu = BX_CPU_C(0);
#endif
