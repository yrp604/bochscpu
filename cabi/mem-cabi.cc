#include "bochs.h"

namespace rust {
extern "C" {
    Bit8u* mem_guest_to_host(bx_phy_address, unsigned);
    void mem_read_phy(bx_phy_address, unsigned, void *);
    void mem_write_phy(bx_phy_address, unsigned, void *);
}
}

BX_MEM_C::BX_MEM_C() {}
BX_MEM_C::~BX_MEM_C() {}

void BX_MEM_C::writePhysicalPage(BX_CPU_C *cpu, bx_phy_address addr,
    unsigned len, void *data)
{
    return rust::mem_write_phy(addr, len, data);
}

void BX_MEM_C::readPhysicalPage(BX_CPU_C *cpu, bx_phy_address addr, unsigned len, void *data)
{
    return rust::mem_read_phy(addr, len, data);
}

Bit8u *BX_MEM_C::getHostMemAddr(BX_CPU_C *cpu, bx_phy_address addr, unsigned rw)
{
    return rust::mem_guest_to_host(addr, rw);
}

BOCHSAPI BX_MEM_C bx_mem;
