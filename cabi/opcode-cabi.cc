#include "bochs.h"
#include "cpu/cpu.h"
#include "cpu/decoder/ia_opcodes.h"

#define C_ASSERT(e) typedef char __C_ASSERT__[(e)?1:-1]

C_ASSERT(BX_IA_ERROR == 0);
// this is needed by over to detect end of trace events in before_exec hooks
// to prevent double firing on some pc values
C_ASSERT(BX_INSERTED_OPCODE == 1);
