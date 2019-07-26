use std::slice;

use crate::PhyAddress;
use crate::syncunsafecell::SyncUnsafeCell;

mod phy;
pub use phy::*;

mod virt;
pub use virt::*;

// despite all the benchmarks claiming that fxhash + hashbrown wins, for our
// benchmarks fnvhash + hashbrown seems to be the winning combo
mod fastmap64_mem;
pub use fastmap64_mem::{add_page, del_page};
use fastmap64_mem::{resolve_hva, resolve_hva_checked};


#[ctor]
static FAULT: SyncUnsafeCell<Box<dyn FnMut(PhyAddress)>> = {
    SyncUnsafeCell::new(Box::new(|_| panic!("no missing_page function set")))
};

const fn page_off(a: PhyAddress) -> (PhyAddress, usize) {
    (a & !0xfff, a as usize & 0xfff)
}

pub unsafe fn fault(gpa: PhyAddress) {
    let f = FAULT.0.get();
    (**f)(gpa);
}

#[no_mangle]
extern "C" fn mem_guest_to_host(gpa: PhyAddress, _rw: u32) -> *mut u8 {
    trace!("translating guest phys {:x}...", gpa);

    unsafe { phy_translate(gpa) }
}

#[no_mangle]
extern "C" fn mem_read_phy(gpa: PhyAddress, sz: u32, dst: *mut u8) {
    trace!("mem read {} bytes from phys {:x}...", sz, gpa);

    let sz = sz as usize;

    unsafe {
        let src_ptr = phy_translate(gpa);
        let src = slice::from_raw_parts(src_ptr, sz);
        let dst = slice::from_raw_parts_mut(dst, sz);

        dst.copy_from_slice(src);
        trace!("mem read {:x?}", src);
    }
}

#[no_mangle]
extern "C" fn mem_write_phy(gpa: PhyAddress, sz: u32, src: *const u8) {
    trace!("mem write {} bytes to phys {:x}...", sz, gpa);

    let sz = sz as usize;

    unsafe {
        let dst_ptr = phy_translate(gpa);
        let dst = slice::from_raw_parts_mut(dst_ptr, sz);
        let src = slice::from_raw_parts(src, sz);

        dst.copy_from_slice(src);
        trace!("mem write {:x?}", src);
    }
}

pub unsafe fn phy_translate(gpa: PhyAddress) -> *mut u8{
    if let Some(hva) = resolve_hva_checked(gpa) {
        return hva;
    }

    fault(gpa);

    resolve_hva(gpa)
}

pub unsafe fn missing_page<T: FnMut(PhyAddress) + 'static>(f: T) {
    *(FAULT.0.get()) = Box::new(f);
}
