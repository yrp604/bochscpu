use std::ffi::{c_char, c_void};
use std::hint::unreachable_unchecked;

use crate::Address;

// these should match the constants in opcode-cabi.cc and maybe eventually
// be extracted with bindgen
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Opcode {
    Error = 0,
    Inserted = 1,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum DisasmStyle {
    Intel = 0,
    Gas = 1,
}

impl From<u32> for DisasmStyle {
    fn from(i: u32) -> Self {
        match i {
            0 => DisasmStyle::Intel,
            1 => DisasmStyle::Gas,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

extern "C-unwind" {
    pub fn instr_bx_opcode(_: *const c_void) -> u32;
    pub fn instr_imm16(_: *const c_void) -> u16;
    pub fn instr_imm32(_: *const c_void) -> u32;
    pub fn instr_imm64(_: *const c_void) -> u64;
    pub fn instr_src(_: *const c_void) -> u32;
    pub fn instr_dst(_: *const c_void) -> u32;
    pub fn instr_seg(_: *const c_void) -> u32;
    pub fn instr_modC0(_: *const c_void) -> u32;
    pub fn instr_resolve_addr(_: *const c_void) -> u64;
    pub fn opcode_disasm_wrapper(
        _: u32,
        _: u32,
        _: *mut Address,
        _: *mut Address,
        _: *mut u8,
        _: *const c_char,
        _: DisasmStyle,
    ) -> u32;

    // pub fn instr_dmp();
}
