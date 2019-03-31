#include "bochs.h"
#include "cpu/cpu.h"

typedef BX_CPU_C *BX_CPU_C_PTR;

extern "C" {
BOCHSAPI void cpu_loop(unsigned id) {
    bx_cpu_array[id]->cpu_loop();
}

BOCHSAPI void cpu_new(unsigned id) {
    BX_CPU_C *c  = new BX_CPU_C(id);
    c->initialize();
    c->sanity_checks();
    bx_cpu_array[id] = new BX_CPU_C(id);
}

BOCHSAPI void cpu_delete(unsigned id) {
    delete bx_cpu_array[id];
}
}

Bit8u bx_cpu_count = 0xff; // max number of processsors
BOCHSAPI BX_CPU_C_PTR *bx_cpu_array = new BX_CPU_C_PTR[BX_SMP_PROCESSORS];
