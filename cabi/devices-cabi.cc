#include "bochs.h"

#include "iodev/iodev.h"

bx_devices_c::bx_devices_c() {}
bx_devices_c::~bx_devices_c() {}
Bit32u bx_devices_c::inp(Bit16u addr, unsigned len) { assert(false); }
void bx_devices_c::outp(Bit16u addr, Bit32u value, unsigned len) { assert(false); }

Bit32u bx_pci_device_c::pci_read_handler(unsigned char, unsigned int) { assert(false); }

logfunctions *pluginlog;
bx_devices_c bx_devices;
