#include "bochs.h"
#include "cpu/cpu.h"
#include "memory/memory-bochs.h"

namespace rust {
extern "C" {
    Bit8u* mem_guest_to_host(unsigned, bx_phy_address, unsigned);
    void mem_read_phy(unsigned, bx_phy_address, unsigned, void *);
    void mem_write_phy(unsigned, bx_phy_address, unsigned, void *);
}
}

BX_MEM_C::BX_MEM_C() = default;
BX_MEM_C::~BX_MEM_C() = default;
BX_MEMORY_STUB_C::BX_MEMORY_STUB_C() = default;
BX_MEMORY_STUB_C::~BX_MEMORY_STUB_C() = default;

void BX_MEM_C::writePhysicalPage(BX_CPU_C *cpu, bx_phy_address addr,
                                                                      unsigned len, void *data)
{
    return rust::mem_write_phy(cpu->which_cpu(), addr, len, data);
}

void BX_MEM_C::readPhysicalPage(BX_CPU_C *cpu, bx_phy_address addr, unsigned len, void *data)
{
    return rust::mem_read_phy(cpu->which_cpu(), addr, len, data);
}

Bit8u *BX_MEM_C::getHostMemAddr(BX_CPU_C *cpu, bx_phy_address addr, unsigned rw)
{
    return rust::mem_guest_to_host(cpu->which_cpu(), addr, rw);
}

bool BX_MEM_C::dbg_fetch_mem(BX_CPU_C *cpu, bx_phy_address addr, unsigned len, Bit8u *buf)
{
    assert(false);

    return false;
}

Bit64u BX_MEMORY_STUB_C::get_memory_len()
{
    return (BX_MEM_THIS len);
}

BOCHSAPI BX_MEM_C bx_mem;
