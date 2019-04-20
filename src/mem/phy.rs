use std::mem;
use std::slice;

use crate::{Address, PhyAddress};
use crate::mem::phy_translate;

pub fn phy_read_u64(gpa: PhyAddress) -> u64 {
    let mut buf = [0; mem::size_of::<u64>()];
    phy_read_slice(gpa, &mut buf);
    u64::from_le_bytes(buf)
}

pub fn phy_read_u32(gpa: PhyAddress) -> u32 {
    let mut buf = [0; mem::size_of::<u32>()];
    phy_read_slice(gpa, &mut buf);
    u32::from_le_bytes(buf)
}

pub fn phy_read_u16(gpa: PhyAddress) -> u16 {
    let mut buf = [0; mem::size_of::<u16>()];
    phy_read_slice(gpa, &mut buf);
    u16::from_le_bytes(buf)
}

pub fn phy_read_u8(gpa: PhyAddress) -> u8 {
    let mut buf = [0; mem::size_of::<u8>()];
    phy_read_slice(gpa, &mut buf);
    u8::from_le_bytes(buf)
}

pub fn phy_write_u64(gpa: PhyAddress, val: u64) {
    phy_write(gpa, &val.to_le_bytes());
}

pub fn phy_write_u32(gpa: PhyAddress, val: u32) {
    phy_write(gpa, &val.to_le_bytes());
}

pub fn phy_write_u16(gpa: PhyAddress, val: u16) {
    phy_write(gpa, &val.to_le_bytes());
}

pub fn phy_write_u8(gpa: PhyAddress, val: u8) {
    phy_write(gpa, &val.to_le_bytes());
}

pub fn phy_read_slice(gpa: PhyAddress, buf: &mut [u8]) {
    // make sure we dont span pages
    debug_assert!(gpa + (buf.len() as PhyAddress) < (gpa & !0xfff) + 0x1000);

    let src = unsafe {
        let src_ptr = phy_translate(gpa);
        slice::from_raw_parts(src_ptr, buf.len())
    };


    &buf.copy_from_slice(src);
}

pub fn phy_read_page(gpa: PhyAddress, buf: &mut Vec<u8>, sz: usize) {
    // make sure we dont span pages
    debug_assert!(gpa + (sz as PhyAddress) < (gpa & !0xfff) + 0x1000);

    let len = buf.len();
    buf.reserve(len + sz);
    let buf_slice = &mut buf[len..len+sz];
    phy_read_slice(gpa, buf_slice)
}

pub fn phy_write(gpa: PhyAddress, data: &[u8]) {
    // make sure we dont span pages
    debug_assert!(gpa + (data.len() as PhyAddress) < (gpa & !0xfff) + 0x1000);

    let dst = unsafe {
        let dst_ptr = phy_translate(gpa);
        slice::from_raw_parts_mut(dst_ptr, data.len())
    };

    dst.copy_from_slice(data);
}
