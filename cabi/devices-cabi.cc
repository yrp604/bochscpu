#include "bochs.h"

#include "iodev/iodev.h"

bx_devices_c::bx_devices_c() {}
bx_devices_c::~bx_devices_c() {}

Bit32u bx_devices_c::inp(Bit16u addr, unsigned len) {
    Bit32u ret = 0;

    BX_INSTR_INP2(addr, len, &ret);

    return ret;
}

void bx_devices_c::outp(Bit16u addr, Bit32u value, unsigned len) {
    BX_INSTR_OUTP(addr, len, &value);
}

Bit32u bx_pci_device_c::pci_read_handler(unsigned char, unsigned int) { assert(false); return 0; }

logfunctions *pluginlog;
bx_devices_c bx_devices;
