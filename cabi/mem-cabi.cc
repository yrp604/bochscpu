#include "bochs.h"

BX_MEM_C::BX_MEM_C() {}
BX_MEM_C::~BX_MEM_C() {}

void BX_MEM_C::writePhysicalPage(BX_CPU_C *cpu, bx_phy_address addr,
    unsigned len, void *data)
{
    assert(false);
}

void BX_MEM_C::readPhysicalPage(BX_CPU_C *cpu, bx_phy_address addr, unsigned len, void *data)
{
    assert(false);
}

Bit8u *BX_MEM_C::getHostMemAddr(BX_CPU_C *cpu, bx_phy_address addr, unsigned rw)
{
    assert(false);
}

BOCHSAPI BX_MEM_C bx_mem;
