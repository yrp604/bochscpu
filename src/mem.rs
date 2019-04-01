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

    unsafe {
        let m = &(*(MEM.0.get()));

        *(m.get(&a).unwrap())
    }
}

pub unsafe fn add_page(a: PhyAddress, p: *mut u8) {
    (*(MEM.0.get())).insert(a, p);
}

pub unsafe fn del_page(a: PhyAddress) {
    (*(MEM.0.get())).remove(&a);
}
