#include <stdint.h>

#include "bochs.h"

// this is 0xff if xAPIC is supported or 0xf if not. AFAICT everything since
// pentium 4 has supported xAPIC, so we'll just hardcode this. Values come from
// bochs/main.cc
BOCHSAPI bool simulate_xapic = true;
BOCHSAPI Bit32u apic_id_mask = 0xff;
