#include <cstdint>
#include <new>

#include "bochs.h"
#include "cpu/cpu.h"
#include "cpu/apic.h"
#include "pc_system.h"

typedef BX_CPU_C *BX_CPU_C_PTR;


extern "C" {
void cpu_loop(unsigned id) {
    BX_CPU(id)->cpu_loop();
}

void cpu_new(unsigned id) {
#if BX_SUPPORT_SMP
    // bochs assumes that all things are init'd to zero, which breaks ASan so
    // we use placement new to zero the mem
    void *zero = malloc(sizeof(BX_CPU_C));
    memset(zero, 0, sizeof(BX_CPU_C));

    BX_CPU_C *c = new (zero) BX_CPU_C(id);

    BX_CPU(id) = c;
#endif

    BX_CPU(id)->initialize();
    BX_CPU(id)->reset(BX_RESET_HARDWARE);
    BX_CPU(id)->sanity_checks();

    BX_INSTR_INITIALIZE(id);
}

void cpu_delete(unsigned id) {
#if BX_SUPPORT_SMP

    BX_CPU(id)->~BX_CPU_C();
    free(BX_CPU(id));

    BX_CPU(id) = NULL;
#endif
}

void cpu_bail(unsigned id) {
    BX_CPU_C *c = BX_CPU(id);

    longjmp(c->jmp_buf_env, 1);
}

void cpu_set_mode(unsigned id) {
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

bx_address cpu_get_pc(unsigned id) {
    return BX_CPU(id)->get_instruction_pointer();
}

void cpu_set_pc(unsigned id, Bit64u val) {
    BX_CPU(id)->gen_reg[BX_64BIT_REG_RIP].rrx = val;
    BX_CPU(id)->prev_rip = val;
}

void cpu_set_sp(unsigned id, Bit64u val) {
    BX_CPU(id)->gen_reg[BX_64BIT_REG_RSP].rrx = val;
    BX_CPU(id)->prev_rsp = val;
}

Bit64u cpu_get_ssp(unsigned id) {
    return BX_CPU(id)->get_ssp();
}

void cpu_set_ssp(unsigned id, Bit64u val) {
    BX_CPU(id)->gen_reg[BX_64BIT_REG_SSP].rrx = val;
}

Bit64u cpu_get_reg64(unsigned id, unsigned reg) {
    return BX_CPU(id)->get_reg64(reg);
}

void cpu_set_reg64(unsigned id, unsigned reg, Bit64u val) {
    BX_CPU(id)->set_reg64(reg, val);
}

Bit32u cpu_get_eflags(unsigned id) {
    return BX_CPU(id)->eflags;
}

void cpu_set_eflags(unsigned id, Bit32u eflags) {
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

void cpu_get_seg(
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

void cpu_set_seg(
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

void cpu_get_ldtr(
        unsigned id,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    return get_seg(id, &BX_CPU(id)->ldtr, present, selector, base, limit, attr);
}

void cpu_set_ldtr(
        unsigned id,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    return set_seg(id, &BX_CPU(id)->ldtr, present, selector, base, limit, attr);
}

void cpu_get_tr(
        unsigned id,
        Bit32u *present,
        Bit16u *selector,
        bx_address *base,
        Bit32u *limit,
        Bit16u *attr)
{
    return get_seg(id, &BX_CPU(id)->tr, present, selector, base, limit, attr);
}

void cpu_set_tr(
        unsigned id,
        Bit32u present,
        Bit16u selector,
        bx_address base,
        Bit32u limit,
        Bit16u attr)
{
    return set_seg(id, &BX_CPU(id)->tr, present, selector, base, limit, attr);
}

void cpu_get_gdtr(unsigned id, bx_address *base, Bit16u *limit) {
    *base= BX_CPU(id)->gdtr.base;
    *limit = BX_CPU(id)->gdtr.limit;
}

void cpu_set_gdtr(unsigned id, bx_address base, Bit16u limit) {
    BX_CPU(id)->gdtr.base = base;
    BX_CPU(id)->gdtr.limit = limit;
}

void cpu_get_idtr(unsigned id, bx_address *base, Bit16u *limit) {
    *base= BX_CPU(id)->idtr.base;
    *limit = BX_CPU(id)->idtr.limit;
}

void cpu_set_idtr(unsigned id, bx_address base, Bit16u limit) {
    BX_CPU(id)->idtr.base = base;
    BX_CPU(id)->idtr.limit = limit;
}

// debug registers

bx_address cpu_get_dr(unsigned id, unsigned dr) {
    return BX_CPU(id)->dr[dr];
}

void cpu_set_dr(unsigned id, unsigned dr, bx_address v) {
    BX_CPU(id)->dr[dr] = v;
}

Bit32u cpu_get_dr6(unsigned id) {
    return BX_CPU(id)->dr6.get32();
}

void cpu_set_dr6(unsigned id, Bit32u v) {
    BX_CPU(id)->dr6.set32(v);
}

Bit32u cpu_get_dr7(unsigned id) {
    return BX_CPU(id)->dr7.get32();
}

void cpu_set_dr7(unsigned id, Bit32u v) {
    BX_CPU(id)->dr7.set32(v);
}

// control registers

Bit32u cpu_get_cr0(unsigned id) {
    return BX_CPU(id)->cr0.get32();
}

void cpu_set_cr0(unsigned id, Bit32u v) {
    BX_CPU(id)->cr0.set32(v);
}

bx_address cpu_get_cr2(unsigned id) {
    return BX_CPU(id)->cr2;
}

void cpu_set_cr2(unsigned id, bx_address v) {
    BX_CPU(id)->cr2 = v;
}

bx_address cpu_get_cr3(unsigned id) {
    return BX_CPU(id)->cr3;
}

void cpu_set_cr3(unsigned id, bx_address v) {
    BX_CPU(id)->cr3 = v;
}

Bit64u cpu_get_cr4(unsigned id) {
    return BX_CPU(id)->cr4.get();
}

void cpu_set_cr4(unsigned id, Bit64u v) {
    BX_CPU(id)->cr4.set(v);
}

Bit32u cpu_get_cr8(unsigned id) {
    return BX_CPU(id)->get_cr8();
}

void cpu_set_cr8(unsigned id, Bit32u v) {
#if BX_SUPPORT_APIC
    BX_CPU(id)->lapic->set_tpr((v & 0xf) << 4);
#endif
}

Bit32u cpu_get_efer(unsigned id) {
    return BX_CPU(id)->efer.get32();
}

void cpu_set_efer(unsigned id, Bit32u v) {
    BX_CPU(id)->efer.set32(v);
}

Bit32u cpu_get_xcr0(unsigned id) {
    return BX_CPU(id)->xcr0.get32();
}

void cpu_set_xcr0(unsigned id, Bit32u v) {
    BX_CPU(id)->xcr0.set32(v);
}

// model specific registers

Bit64u cpu_get_kernel_gs_base(unsigned id) {
    return BX_CPU(id)->msr.kernelgsbase;
}

void cpu_set_kernel_gs_base(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.kernelgsbase = v;
}

Bit32u cpu_get_sysenter_cs(unsigned id) {
    return BX_CPU(id)->msr.sysenter_cs_msr;
}

void cpu_set_sysenter_cs(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.sysenter_cs_msr = v;
}

bx_address cpu_get_sysenter_esp(unsigned id) {
    return BX_CPU(id)->msr.sysenter_esp_msr;
}

void cpu_set_sysenter_esp(unsigned id, bx_address v) {
    BX_CPU(id)->msr.sysenter_esp_msr = v;
}

bx_address cpu_get_sysenter_eip(unsigned id) {
    return BX_CPU(id)->msr.sysenter_eip_msr;
}

void cpu_set_sysenter_eip(unsigned id, bx_address v) {
    BX_CPU(id)->msr.sysenter_eip_msr = v;
}

Bit64u cpu_get_star(unsigned id) {
    return BX_CPU(id)->msr.star;
}

void cpu_set_star(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.star = v;
}

Bit64u cpu_get_lstar(unsigned id) {
    return BX_CPU(id)->msr.lstar;
}

void cpu_set_lstar(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.lstar = v;
}

Bit64u cpu_get_cstar(unsigned id) {
    return BX_CPU(id)->msr.cstar;
}

void cpu_set_cstar(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.cstar = v;
}

Bit32u cpu_get_fmask(unsigned id) {
    return BX_CPU(id)->msr.fmask;
}

void cpu_set_fmask(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.fmask = v;
}

Bit64u cpu_get_tsc(unsigned id) {
    return BX_CPU(id)->get_TSC();
}

void cpu_set_tsc(unsigned id, Bit64u v) {
    BX_CPU(id)->set_TSC(v);
}

Bit64u cpu_get_tsc_aux(unsigned id) {
    return BX_CPU(id)->msr.tsc_aux;
}

void cpu_set_tsc_aux(unsigned id, Bit32u v) {
    BX_CPU(id)->msr.tsc_aux = v;
}

bx_phy_address cpu_get_apicbase(unsigned id) {
    return BX_CPU(id)->msr.apicbase;
}

void cpu_set_apicbase(unsigned id, bx_phy_address v) {
    BX_CPU(id)->msr.apicbase = v;
}

Bit64u cpu_get_pat(unsigned id) {
    return BX_CPU(id)->msr.pat._u64;
}

void cpu_set_pat(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.pat._u64 = v;
}

Bit64u cpu_get_cet_control_u(unsigned id) {
    return BX_CPU(id)->msr.ia32_cet_control[1];
}

void cpu_set_cet_control_u(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_cet_control[1] = v;
}

Bit64u cpu_get_cet_control_s(unsigned id) {
    return BX_CPU(id)->msr.ia32_cet_control[0];
}

void cpu_set_cet_control_s(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_cet_control[0] = v;
}

Bit64u cpu_get_pl0_ssp(unsigned id) {
    return BX_CPU(id)->msr.ia32_pl_ssp[0];
}

void cpu_set_pl0_ssp(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_pl_ssp[0] = v;
}

Bit64u cpu_get_pl1_ssp(unsigned id) {
    return BX_CPU(id)->msr.ia32_pl_ssp[1];
}

void cpu_set_pl1_ssp(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_pl_ssp[1] = v;
}

Bit64u cpu_get_pl2_ssp(unsigned id) {
    return BX_CPU(id)->msr.ia32_pl_ssp[2];
}

void cpu_set_pl2_ssp(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_pl_ssp[2] = v;
}

Bit64u cpu_get_pl3_ssp(unsigned id) {
    return BX_CPU(id)->msr.ia32_pl_ssp[3];
}

void cpu_set_pl3_ssp(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_pl_ssp[3] = v;
}

Bit64u cpu_get_interrupt_ssp_table(unsigned id) {
    return BX_CPU(id)->msr.ia32_interrupt_ssp_table;
}

void cpu_set_interrupt_ssp_table(unsigned id, Bit64u v) {
    BX_CPU(id)->msr.ia32_interrupt_ssp_table = v;
}

// ZMM

void cpu_get_zmm(unsigned id, unsigned reg, Bit64u z[]) {
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

void cpu_set_zmm(unsigned id, unsigned reg, Bit64u z[]) {
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

Bit16u cpu_get_fp_cw(unsigned id) {
    return BX_CPU(id)->the_i387.cwd;
}

void cpu_set_fp_cw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.cwd = v;
}

Bit16u cpu_get_fp_sw(unsigned id) {
    return BX_CPU(id)->the_i387.swd;
}

void cpu_set_fp_sw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.swd = v;
}

Bit16u cpu_get_fp_tw(unsigned id) {
    return BX_CPU(id)->the_i387.twd;
}

void cpu_set_fp_tw(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.twd = v;
}

Bit16u cpu_get_fp_op(unsigned id) {
    return BX_CPU(id)->the_i387.foo;
}

void cpu_set_fp_op(unsigned id, Bit16u v) {
    BX_CPU(id)->the_i387.foo = v;
}

void cpu_get_fp_st(unsigned id, unsigned reg, Bit64u *fraction, Bit16u *exp) {
    const floatx80 f = BX_CPU(id)->the_i387.st_space[reg];
    *fraction = f.signif;
    *exp = f.signExp;
}

void cpu_set_fp_st(unsigned id, unsigned reg, const Bit64u fraction, const Bit16u exp) {
    floatx80 f;
    f.signif = fraction;
    f.signExp = exp;
    BX_CPU(id)->the_i387.st_space[reg] = f;
}

Bit32u cpu_get_cpu_mode(unsigned id) {
    return BX_CPU(id)->get_cpu_mode();
}

Bit32u cpu_get_mxcsr(unsigned id) {
    return BX_CPU(id)->mxcsr.mxcsr;
}

void cpu_set_mxcsr(unsigned id, Bit32u v) {
    BX_CPU(id)->mxcsr.mxcsr = v;
}

Bit32u cpu_get_mxcsr_mask(unsigned id) {
    return BX_CPU(id)->mxcsr_mask;
}

void cpu_set_mxcsr_mask(unsigned id, Bit32u v) {
    BX_CPU(id)->mxcsr_mask = v;
}

unsigned cpu_killbit(unsigned id) {
    return bx_pc_system.kill_bochs_request;
}

void cpu_set_killbit(unsigned id) {
    BX_CPU(id)->async_event = 1;
    bx_pc_system.kill_bochs_request = 1;
}

void cpu_clear_killbit(unsigned id) {
    BX_CPU(id)->async_event = 0;
    bx_pc_system.kill_bochs_request = 0;
}

void cpu_exception(unsigned id, unsigned vector, Bit16u error) {
    BX_CPU(id)->exception(vector, error);
}
}

Bit8u bx_cpu_count = 0xff; // max number of processsors

#if BX_SUPPORT_SMP
BX_CPU_C_PTR *bx_cpu_array = new BX_CPU_C_PTR[BX_SMP_PROCESSORS];
#else
BX_CPU_C bx_cpu = BX_CPU_C(0);
#endif
