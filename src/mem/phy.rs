use std::mem;
use std::slice;

use crate::PhyAddress;
use crate::mem::phy_translate;

pub fn phy_read_u64(gpa: PhyAddress) -> u64 {
    let mut buf = [0; mem::size_of::<u64>()];
    phy_read_slice(gpa, &mut buf);
    u64::from_le_bytes(buf)
}

pub fn phy_read_slice(gpa: PhyAddress, buf: &mut [u8]) {
    // make sure we dont span pages
    debug_assert!(gpa + (buf.len() as PhyAddress) <= (gpa & !0xfff) + 0x1000);

    let src = unsafe {
        let src_ptr = phy_translate(gpa);
        slice::from_raw_parts(src_ptr, buf.len())
    };

    buf.copy_from_slice(src);
}

pub fn phy_read(gpa: PhyAddress, buf: &mut Vec<u8>, sz: usize) {
    // make sure we dont span pages
    debug_assert!(gpa + (sz as PhyAddress) <= (gpa & !0xfff) + 0x1000);

    let len = buf.len();
    buf.reserve(sz);
    let buf_slice = &mut buf[len..len + sz];
    phy_read_slice(gpa, buf_slice)
}

pub fn phy_write(gpa: PhyAddress, data: &[u8]) {
    // make sure we dont span pages
    debug_assert!(gpa + (data.len() as PhyAddress) <= (gpa & !0xfff) + 0x1000);

    let dst = unsafe {
        let dst_ptr = phy_translate(gpa);
        slice::from_raw_parts_mut(dst_ptr, data.len())
    };

    dst.copy_from_slice(data);
}
