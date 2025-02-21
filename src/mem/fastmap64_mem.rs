use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use fnv::FnvHasher;

use crate::PhyAddress;
use crate::mem::page_off;
use crate::syncunsafecell::SyncUnsafeCell;

pub type FastMap64<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

#[ctor]
pub static MEM: SyncUnsafeCell<FastMap64<PhyAddress, *mut u8>> =
    unsafe { SyncUnsafeCell::new(FastMap64::default()) };

unsafe fn mem() -> &'static mut FastMap64<PhyAddress, *mut u8> {
    unsafe { &mut (*(MEM.0.get())) }
}

pub unsafe fn resolve_hva(gpa: PhyAddress) -> *mut u8 {
    unsafe {
        let (page, off) = page_off(gpa);
        (*(mem().get(&page).unwrap())).add(off)
    }
}

pub unsafe fn resolve_hva_checked(gpa: PhyAddress) -> Option<*mut u8> {
    unsafe {
        let (page, off) = page_off(gpa);

        mem().get(&page).map(|p| p.add(off))
    }
}

pub unsafe fn page_insert(gpa: PhyAddress, hva: *mut u8) {
    unsafe {
        let (page, _) = page_off(gpa);
        mem().insert(page, hva);
    }
}

pub unsafe fn page_remove(gpa: PhyAddress) {
    unsafe {
        let (page, _) = page_off(gpa);
        mem().remove(&page);
    }
}
