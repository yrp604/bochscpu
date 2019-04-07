use std::collections::BTreeMap;
use std::slice;

use crate::PhyAddress;
use crate::syncunsafecell::SyncUnsafeCell;


#[ctor]
pub static MEM: SyncUnsafeCell<BTreeMap<PhyAddress, *mut u8>> = {
    SyncUnsafeCell::new(BTreeMap::new())
};

pub unsafe fn mem() -> &'static mut BTreeMap<PhyAddress, *mut u8> {
    &mut (*(MEM.0.get()))
}

#[no_mangle]
extern "C" fn mem_guest_to_host(a: PhyAddress, _rw: u32) -> *mut u8 {
    trace!("translating guest phys {:x}...", a);

    let page = a & !0xfff;
    let off = a & 0xfff;

    unsafe {
        (*(mem().get(&page).unwrap())).add(off as usize)
    }
}

#[no_mangle]
extern "C" fn mem_read_phy(a: PhyAddress, sz: u32, dst: *mut u8) {
    trace!("mem read {} bytes from phys {:x}...", sz, a);

    let sz = sz as usize;
    let page = a & !0xfff;
    let off = a & 0xfff;

    unsafe {
        let src_ptr = (*(mem().get(&page).unwrap())).add(off as usize);
        let src = slice::from_raw_parts(src_ptr, sz);
        let dst = slice::from_raw_parts_mut(dst, sz);

        dst.copy_from_slice(src);
    }
}

#[no_mangle]
extern "C" fn mem_write_phy(a: PhyAddress, sz: u32, src: *const u8) {
    trace!("mem write {} bytes to phys {:x}...", sz, a);

    let sz = sz as usize;
    let page = a & !0xfff;
    let off = a & 0xfff;

    unsafe {
        let dst_ptr = (*(mem().get(&page).unwrap())).add(off as usize);
        let dst = slice::from_raw_parts_mut(dst_ptr, sz);
        let src = slice::from_raw_parts(src, sz);

        dst.copy_from_slice(src);
    }
}
