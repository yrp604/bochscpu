use std::slice;

use crate::cpu::{cpu_bail, cpu_killbit};
use crate::hook;
use crate::PhyAddress;

mod phy;
pub use phy::*;

mod virt;
pub use virt::*;

// despite all the benchmarks claiming that fxhash + hashbrown wins, for our
// benchmarks fnvhash + hashbrown seems to be the winning combo
mod fastmap64_mem;
use fastmap64_mem::page_insert as mem_insert;
pub use fastmap64_mem::{mem, page_remove};
use fastmap64_mem::{resolve_hva, resolve_hva_checked};

pub const fn phy_mask(gpa: PhyAddress) -> PhyAddress {
    gpa & 0x000f_ffff_ffff_ffff
}

const fn page_off(a: PhyAddress) -> (PhyAddress, usize) {
    (a & !0xfff, a as usize & 0xfff)
}

pub unsafe fn page_insert(gpa: PhyAddress, hva: *mut u8) {
    assert_eq!(hva.align_offset(0x1000), 0);

    mem_insert(gpa, hva)
}

#[no_mangle]
extern "C-unwind" fn mem_guest_to_host(cpu: u32, gpa: PhyAddress, _rw: u32) -> *mut u8 {
    trace!("translating guest phys {:x}...", gpa);

    unsafe { guest_phy_translate(cpu, gpa) }
}

#[no_mangle]
extern "C-unwind" fn mem_read_phy(cpu: u32, gpa: PhyAddress, sz: u32, dst: *mut u8) {
    trace!("mem read {} bytes from phys {:x}...", sz, gpa);

    let sz = sz as usize;

    unsafe {
        let src_ptr = guest_phy_translate(cpu, gpa);
        let src = slice::from_raw_parts(src_ptr, sz);
        let dst = slice::from_raw_parts_mut(dst, sz);

        dst.copy_from_slice(src);
        trace!("mem read {:x?}", src);
    }
}

#[no_mangle]
extern "C-unwind" fn mem_write_phy(cpu: u32, gpa: PhyAddress, sz: u32, src: *const u8) {
    trace!("mem write {} bytes to phys {:x}...", sz, gpa);

    let sz = sz as usize;

    unsafe {
        let dst_ptr = guest_phy_translate(cpu, gpa);
        let dst = slice::from_raw_parts_mut(dst_ptr, sz);
        let src = slice::from_raw_parts(src, sz);

        dst.copy_from_slice(src);
        trace!("mem write {:x?}", src);
    }
}

pub unsafe fn guest_phy_translate(cpu: u32, gpa: PhyAddress) -> *mut u8 {
    // i think this is needed because bochs will call into this with high bits
    // set?
    let real_gpa = gpa & 0x000f_ffff_ffff_ffff;

    if let Some(hva) = resolve_hva_checked(real_gpa) {
        return hva;
    }

    hook::fault(real_gpa);

    // check to see if our fault handler requested the cpu be killed
    if cpu_killbit(cpu) != 0 {
        cpu_bail(cpu)
    }

    resolve_hva(real_gpa)
}

// this function exists to split translations happening by the emulator and
// those requested by the guest. Emulator translations requests do not have an
// associated cpu and thus cannot be killed by the page fault hook.
pub unsafe fn phy_translate(gpa: PhyAddress) -> *mut u8 {
    // i think this is needed because bochs will call into this with high bits
    // set?
    let real_gpa = phy_mask(gpa);

    if let Some(hva) = resolve_hva_checked(real_gpa) {
        return hva;
    }

    hook::fault(real_gpa);

    resolve_hva(real_gpa)
}
