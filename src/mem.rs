use std::collections::BTreeMap;

use crate::PhyAddress;
use crate::syncunsafecell::SyncUnsafeCell;


#[ctor]
pub static MEM: SyncUnsafeCell<BTreeMap<PhyAddress, *mut u8>> = {
    SyncUnsafeCell::new(BTreeMap::new())
};


#[no_mangle]
extern "C" fn mem_guest_to_host(a: PhyAddress, _rw: u32) -> *mut u8 {
    trace!("translating guest phys {:x}...", a);

    let page = a & !0xfff;
    let off = a & 0xfff;

    unsafe {
        let m = &(*(MEM.0.get()));

        (*(m.get(&page).unwrap())).add(off as usize)
    }
}

pub unsafe fn page_add(a: PhyAddress, p: *mut u8) {
    (*(MEM.0.get())).insert(a, p);
}

pub unsafe fn page_del(a: PhyAddress) {
    (*(MEM.0.get())).remove(&a);
}
