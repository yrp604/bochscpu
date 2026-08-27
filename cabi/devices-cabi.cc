#include "bochs.h"

#include "iodev/iodev.h"


void bx_devices_c::init_stubs()
{
  pluginCmosDevice = &stubCmos;
  pluginDmaDevice = &stubDma;
  pluginHardDrive = &stubHardDrive;
  pluginPicDevice = &stubPic;
  pluginPitDevice = &stubPit;
  pluginSpeaker = &stubSpeaker;
  pluginVgaDevice = &stubVga;
#if BX_SUPPORT_IODEBUG
  pluginIODebug = &stubIODebug;
#endif
#if BX_SUPPORT_APIC
  pluginIOAPIC = &stubIOAPIC;
#endif
#if BX_SUPPORT_GAMEPORT
  pluginGameport = &stubGameport;
#endif
#if BX_SUPPORT_PCI
  pluginPci2IsaBridge = &stubPci2Isa;
  pluginPciIdeController = &stubPciIde;
  pluginACPIController = &stubACPIController;
#endif
  pluginExtFpuIRQ = &stubExtFpuIRQ;
}

bx_devices_c::bx_devices_c() {
    init_stubs();
}

bx_devices_c::~bx_devices_c() {}
Bit32u bx_devices_c::inp(Bit16u addr, unsigned len) { assert(false); return 0; }
void bx_devices_c::outp(Bit16u addr, Bit32u value, unsigned len) { assert(false); }

Bit32u bx_pci_device_c::pci_read_handler(unsigned char, unsigned int) { assert(false); return 0; }

logfunctions *pluginlog;
bx_devices_c bx_devices;