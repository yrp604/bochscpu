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

unsigned instr_seg(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->seg();
}

unsigned instr_modC0(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;
    return instr->modC0();
}

Bit64u instr_resolve_addr(void *i) {
    bxInstruction_c *instr = (bxInstruction_c *)i;

    // @WARNING: The documentation for NEED_CPU_REG_SHORTCUTS clearly
    // states that it is not safe to use this macro outside of the
    // BX_CPU_C class. However, we need to use it here, as we need
    // to resolve the address of the instruction. I could not find
    // any other way to do this, so I'm just going to use it here.

    // Soooo, yeah, this is a bit of a hack. We can't simply use 
    // the BX_CPU_RESOLVE_ADDR macro, as it is inside BX_CPU_C class
    // and as a result, we can't call it from here. So, I just copied
    // the expansion of the macro here. It's not pretty, but it works.
    return ((instr)->as64L() ? BX_CPU_C::BxResolve64(instr)
                             : BX_CPU_C::BxResolve32(instr));
}

unsigned opcode_disasm_wrapper(bool is_32, bool is_64, bx_address cs_base,
                               bx_address ip, const Bit8u *instr, char *disbuf,
                               BxDisasmStyle style) {
    bxInstruction_c i;
    disasm(instr, is_32, is_64, disbuf, &i, cs_base, ip, style);
    unsigned ilen = i.ilen();
    return ilen;
}

// void instr_dmp() {
//     // 64
//     printf("CMP-64\n");
//     printf("const BX_IA_CMP_RAXId: u32 = %#x;\n", BX_IA_CMP_RAXId);
//     printf("const BX_IA_CMP_EqsIb: u32 = %#x;\n", BX_IA_CMP_EqsIb);
//     printf("const BX_IA_CMP_EqId: u32 = %#x;\n", BX_IA_CMP_EqId);
//     printf("const BX_IA_CMP_GqEq: u32 = %#x;\n", BX_IA_CMP_GqEq);
//     printf("const BX_IA_CMP_EqGq: u32 = %#x;\n", BX_IA_CMP_EqGq);

//     // 32
//     printf("CMP-32\n");
//     printf("const BX_IA_CMP_EAXId: u32 = %#x;\n", BX_IA_CMP_EAXId);
//     printf("const BX_IA_CMP_EdsIb: u32 = %#x;\n", BX_IA_CMP_EdsIb);
//     printf("const BX_IA_CMP_EdId: u32 = %#x;\n", BX_IA_CMP_EdId);
//     printf("const BX_IA_CMP_GdEd: XXX = %#x;\n", BX_IA_CMP_GdEd);
//     printf("const BX_IA_CMP_EdGd: XXX = %#x;\n", BX_IA_CMP_EdGd);

//     // 16
//     printf("CMP-16\n");
//     printf("const BX_IA_CMP_AXIw: u32 = %#x;\n", BX_IA_CMP_AXIw);
//     printf("const BX_IA_CMP_EwsIb: u32 = %#x;\n", BX_IA_CMP_EwsIb);
//     printf("const BX_IA_CMP_EwIw: u32 = %#x;\n", BX_IA_CMP_EwIw);
//     printf("const BX_IA_CMP_GwEw: XXX = %#x;\n", BX_IA_CMP_GwEw);
//     printf("const BX_IA_CMP_EwGw: XXX = %#x;\n", BX_IA_CMP_EwGw);

//     // 64
//     printf("SUB-64\n");
//     printf("BX_IA_SUB_RAXId = %#x,\n", BX_IA_SUB_RAXId);
//     printf("BX_IA_SUB_EqsIb = %#x,\n", BX_IA_SUB_EqsIb);
//     printf("BX_IA_SUB_EqId = %#x,\n", BX_IA_SUB_EqId);
//     printf("BX_IA_SUB_GqEq = %#x,\n", BX_IA_SUB_GqEq);
//     printf("BX_IA_SUB_EqGq = %#x,\n", BX_IA_SUB_EqGq);

//     // 32
//     printf("SUB-32\n");
//     printf("BX_IA_SUB_EAXId = %#x,\n", BX_IA_SUB_EAXId);
//     printf("BX_IA_SUB_EdsIb = %#x,\n", BX_IA_SUB_EdsIb);
//     printf("BX_IA_SUB_EdId = %#x,\n", BX_IA_SUB_EdId);
//     printf("BX_IA_SUB_GdEd = %#x,\n", BX_IA_SUB_GdEd);
//     printf("BX_IA_SUB_EdGd = %#x,\n", BX_IA_SUB_EdGd);

//     // 16
//     printf("SUB-16\n");
//     printf("BX_IA_SUB_AXIw = %#x,\n", BX_IA_SUB_AXIw);
//     printf("BX_IA_SUB_EwsIb = %#x,\n", BX_IA_SUB_EwsIb);
//     printf("BX_IA_SUB_EwIw = %#x,\n", BX_IA_SUB_EwIw);
//     printf("BX_IA_SUB_GwEw = %#x,\n", BX_IA_SUB_GwEw);
//     printf("BX_IA_SUB_EwGw = %#x,\n", BX_IA_SUB_EwGw);
// }
}
