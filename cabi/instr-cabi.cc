#include "bochs.h"
#include "cpu/cpu.h"

#include "cpu/decoder/ia_opcodes.h"
#include "cpu/decoder/instr.h"

extern "C" {
// NOTE: this is the _bochs_ opcode, not the intel opcode
unsigned instr_bx_opcode(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->getIaOpcode();
}

Bit16u instr_imm16(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->Iw();
}

Bit32u instr_imm32(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->Id();
}

Bit64u instr_imm64(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->Iq();
}

unsigned instr_src(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->src();
}

unsigned instr_dst(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->dst();
}

unsigned opcode_disasm_wrapper(bool is_32, bool is_64, bx_address cs_base,
                               bx_address ip, const Bit8u *instr, char *disbuf,
                               BxDisasmStyle style) {
    bxInstruction_c i;
    disasm(instr, is_32, is_64, disbuf, &i, cs_base, ip, style);
    unsigned ilen = i.ilen();
    return ilen;
}

/*
void instr_dmp() {
    // 64
    printf("const BX_IA_CMP_RAXId: u32 = %#x;\n", BX_IA_CMP_RAXId);
    printf("const BX_IA_CMP_EqsIb: u32 = %#x;\n", BX_IA_CMP_EqsIb);
    printf("const BX_IA_CMP_EqId: u32 = %#x;\n", BX_IA_CMP_EqId);

    // 32
    printf("const BX_IA_CMP_EAXId: u32 = %#x;\n", BX_IA_CMP_EAXId);
    printf("const BX_IA_CMP_EdId: u32 = %#x;\n", BX_IA_CMP_EdId);
    printf("const BX_IA_CMP_EdsIb: u32 = %#x;\n", BX_IA_CMP_EdsIb);

    // 16
    printf("const BX_IA_CMP_AXIw: u32 = %#x;\n", BX_IA_CMP_AXIw);
    printf("const BX_IA_CMP_EwIw: u32 = %#x;\n", BX_IA_CMP_EwIw);
    printf("const BX_IA_CMP_EwsIb: u32 = %#x;\n", BX_IA_CMP_EwsIb);
}
*/
}
