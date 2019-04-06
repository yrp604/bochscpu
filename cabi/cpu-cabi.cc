#include <cstdint>
#include <new>

#include "bochs.h"
#include "cpu/cpu.h"


typedef BX_CPU_C *BX_CPU_C_PTR;

extern "C" {
BOCHSAPI void cpu_loop(unsigned id) {
    bx_cpu_array[id]->cpu_loop();
}

BOCHSAPI BX_CPU_C* cpu_new(unsigned id) {
    // bochs assumes that all things are init'd to zero, which breaks ASan so
    // we use placement new to zero the mem
    void *zero = new uint8_t[sizeof(BX_CPU_C)];
    memset(zero, 0, sizeof(BX_CPU_C));

    BX_CPU_C *c = new (zero) BX_CPU_C(id);

    c->initialize();
    c->sanity_checks();

    bx_cpu_array[id] = c;

    BX_INSTR_INITIALIZE(id);

    return c;
}

BOCHSAPI void cpu_delete(unsigned id) {
    bx_cpu_array[id]->~BX_CPU_C();

    delete[] bx_cpu_array[id];

    bx_cpu_array[id] = NULL;
}

BOCHSAPI void cpu_set_state(unsigned id) {
    BX_CPU_C *c = bx_cpu_array[id];
    c->TLB_flush();

#if BX_CPU_LEVEL >= 4
    c->handleAlignmentCheck(/* CR0.AC reloaded */);
#endif

    c->handleCpuModeChange();

#if BX_CPU_LEVEL >= 6
     c->handleSseModeChange();
#if BX_SUPPORT_AVX
     c->handleAvxModeChange();
#endif
#endif
}

// general purpose regs

BOCHSAPI bx_address cpu_get_pc(unsigned id) {
    return bx_cpu_array[id]->get_instruction_pointer();
}

BOCHSAPI Bit64u cpu_get_reg64(unsigned id, unsigned reg) {
    return bx_cpu_array[id]->get_reg64(reg);
}

BOCHSAPI void cpu_set_reg64(unsigned id, unsigned reg, Bit64u val) {
    bx_cpu_array[id]->set_reg64(reg, val);
}

BOCHSAPI Bit32u cpu_get_eflags(unsigned id) {
    return bx_cpu_array[id]->eflags;
}

BOCHSAPI void cpu_set_eflags(unsigned id, Bit32u eflags) {
    bx_cpu_array[id]->eflags = eflags;
}

// TODO implement get segment registers

BOCHSAPI void cpu_set_seg(unsigned id, unsigned seg) {
    assert(false);
}

// debug registers

BOCHSAPI bx_address cpu_get_dr(unsigned id, unsigned dr) {
    return bx_cpu_array[id]->dr[dr];
}

BOCHSAPI void cpu_set_dr(unsigned id, unsigned dr, bx_address v) {
    bx_cpu_array[id]->dr[dr] = v;
}

BOCHSAPI Bit32u cpu_get_dr6(unsigned id) {
    return bx_cpu_array[id]->dr6.val32;
}

BOCHSAPI void cpu_set_dr6(unsigned id, Bit32u v) {
    bx_cpu_array[id]->dr6.val32 = v;
}

BOCHSAPI Bit32u cpu_get_dr7(unsigned id) {
    return bx_cpu_array[id]->dr7.val32;
}

BOCHSAPI void cpu_set_dr7(unsigned id, Bit32u v) {
    bx_cpu_array[id]->dr7.val32 = v;
}

// control registers

BOCHSAPI Bit32u cpu_get_cr0(unsigned id) {
    return bx_cpu_array[id]->cr0.val32;
}

BOCHSAPI void cpu_set_cr0(unsigned id, Bit32u v) {
    bx_cpu_array[id]->cr0.val32 = v;
}

BOCHSAPI bx_address cpu_get_cr2(unsigned id) {
    return bx_cpu_array[id]->cr2;
}

BOCHSAPI void cpu_set_cr2(unsigned id, bx_address v) {
    bx_cpu_array[id]->cr2 = v;
}

BOCHSAPI bx_address cpu_get_cr3(unsigned id) {
    return bx_cpu_array[id]->cr3;
}

BOCHSAPI void cpu_set_cr3(unsigned id, bx_address v) {
    bx_cpu_array[id]->cr3 = v;
}

BOCHSAPI Bit32u cpu_get_cr4(unsigned id) {
    return bx_cpu_array[id]->cr4.val32;
}

BOCHSAPI void cpu_set_cr4(unsigned id, Bit32u v) {
    bx_cpu_array[id]->cr4.val32 = v;
}

BOCHSAPI Bit32u cpu_get_efer(unsigned id) {
    return bx_cpu_array[id]->efer.val32;
}

BOCHSAPI void cpu_set_efer(unsigned id, Bit32u v) {
    bx_cpu_array[id]->efer.val32 = v;
}

BOCHSAPI Bit32u cpu_get_xcr0(unsigned id) {
    return bx_cpu_array[id]->xcr0.val32;
}

BOCHSAPI void cpu_set_xcr0(unsigned id, Bit32u v) {
    bx_cpu_array[id]->xcr0.val32 = v;
}

// model specific registers

BOCHSAPI Bit64u cpu_get_kernel_gs_base(unsigned id) {
    return bx_cpu_array[id]->msr.kernelgsbase;
}

BOCHSAPI void cpu_set_kernel_gs_base(unsigned id, Bit64u v) {
    bx_cpu_array[id]->msr.kernelgsbase = v;
}

BOCHSAPI Bit32u cpu_get_sysenter_cs(unsigned id) {
    return bx_cpu_array[id]->msr.sysenter_cs_msr;
}

BOCHSAPI void cpu_set_sysenter_cs(unsigned id, Bit32u v) {
    bx_cpu_array[id]->msr.sysenter_cs_msr = v;
}

BOCHSAPI bx_address cpu_get_sysenter_esp(unsigned id) {
    return bx_cpu_array[id]->msr.sysenter_esp_msr;
}

BOCHSAPI void cpu_set_sysenter_esp(unsigned id, bx_address v) {
    bx_cpu_array[id]->msr.sysenter_esp_msr = v;
}

BOCHSAPI bx_address cpu_get_sysenter_eip(unsigned id) {
    return bx_cpu_array[id]->msr.sysenter_eip_msr;
}

BOCHSAPI void cpu_set_sysenter_eip(unsigned id, bx_address v) {
    bx_cpu_array[id]->msr.sysenter_eip_msr = v;
}

BOCHSAPI Bit64u cpu_get_star(unsigned id) {
    return bx_cpu_array[id]->msr.star;
}

BOCHSAPI void cpu_set_star(unsigned id, Bit64u v) {
    bx_cpu_array[id]->msr.star = v;
}

BOCHSAPI Bit64u cpu_get_lstar(unsigned id) {
    return bx_cpu_array[id]->msr.lstar;
}

BOCHSAPI void cpu_set_lstar(unsigned id, Bit64u v) {
    bx_cpu_array[id]->msr.lstar = v;
}

BOCHSAPI Bit64u cpu_get_cstar(unsigned id) {
    return bx_cpu_array[id]->msr.cstar;
}

BOCHSAPI void cpu_set_cstar(unsigned id, Bit64u v) {
    bx_cpu_array[id]->msr.cstar = v;
}

BOCHSAPI Bit32u cpu_get_fmask(unsigned id) {
    return bx_cpu_array[id]->msr.fmask;
}

BOCHSAPI void cpu_set_fmask(unsigned id, Bit32u v) {
    bx_cpu_array[id]->msr.fmask = v;
}

}

Bit8u bx_cpu_count = 0xff; // max number of processsors
BOCHSAPI BX_CPU_C_PTR *bx_cpu_array = new BX_CPU_C_PTR[BX_SMP_PROCESSORS];
