use std::ffi::c_void;

// these should match the constants in opcode-cabi.cc and maybe eventually
// be extracted with bindgen
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Opcode {
    Error = 0,
    Inserted = 1,
}

extern "C-unwind" {
    pub fn instr_bx_opcode(_: *const c_void) -> u32;
    pub fn instr_imm16(_: *const c_void) -> u16;
    pub fn instr_imm32(_: *const c_void) -> u32;
    pub fn instr_imm64(_: *const c_void) -> u64;
    // pub fn instr_dmp();
}
