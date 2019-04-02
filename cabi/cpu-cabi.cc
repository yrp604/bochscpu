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

BOCHSAPI BX_CPU_C* cpu_from(unsigned id) {
    return bx_cpu_array[id];
}
}

Bit8u bx_cpu_count = 0xff; // max number of processsors
BOCHSAPI BX_CPU_C_PTR *bx_cpu_array = new BX_CPU_C_PTR[BX_SMP_PROCESSORS];
