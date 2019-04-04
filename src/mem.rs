use std::collections::BTreeMap;

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

pub unsafe fn page_add(a: PhyAddress, p: *mut u8) {
    mem().insert(a, p);
}

pub unsafe fn page_del(a: PhyAddress) {
    mem().remove(&a);
}
