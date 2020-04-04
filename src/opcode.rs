use std::ffi::c_void;

extern "C" {
    pub fn instr_bx_opcode(_: *const c_void) -> u32;
    pub fn instr_imm16(_: *const c_void) -> u16;
    pub fn instr_imm32(_: *const c_void) -> u32;
    pub fn instr_imm64(_: *const c_void) -> u64;
    // pub fn instr_dmp();
}
