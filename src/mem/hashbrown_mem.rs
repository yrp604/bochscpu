use hashbrown::HashMap;

use crate::PhyAddress;
use crate::mem::page_off;
use crate::syncunsafecell::SyncUnsafeCell;

#[ctor]
pub static MEM: SyncUnsafeCell<HashMap<PhyAddress, *mut u8>> = {
    SyncUnsafeCell::new(HashMap::new())
};

unsafe fn mem() -> &'static mut HashMap<PhyAddress, *mut u8> {
    &mut (*(MEM.0.get()))
}

pub unsafe fn resolve_hva(gpa: PhyAddress) -> *mut u8 {
    let (page, off) = page_off(gpa);
    (*(mem().get(&page).unwrap())).add(off)
}

pub unsafe fn resolve_hva_checked(gpa: PhyAddress) -> Option<*mut u8> {
    let (page, off) = page_off(gpa);

    match mem().get(&page) {
        Some(p) => Some(p.add(off)),
        None => None,
    }
}

pub unsafe fn add_page(gpa: PhyAddress, hva: *mut u8) {
    let (page, _) = page_off(gpa);
    mem().insert(page, hva);
}

pub unsafe fn del_page(gpa: PhyAddress) {
    let (page, _) = page_off(gpa);
    mem().remove(&page);
}
