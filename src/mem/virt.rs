use std::error::Error;
use std::fmt;
use std::iter;
use std::mem;

use crate::mem::{phy_mask, phy_read_slice, phy_read_u64, phy_write};
use crate::{Address, PhyAddress};

const fn pml4_index(gva: Address) -> u64 {
    gva >> (12 + (9 * 3)) & 0x1ff
}

const fn pdpt_index(gva: Address) -> u64 {
    gva >> (12 + (9 * 2)) & 0x1ff
}

const fn pd_index(gva: Address) -> u64 {
    gva >> (12 + (9 * 1)) & 0x1ff
}

const fn pt_index(gva: Address) -> u64 {
    gva >> (12 + (9 * 0)) & 0x1ff
}

const fn base_flags(gpa: Address) -> (Address, u64) {
    (phy_mask(gpa) & !0xfff, gpa & 0x1ff)
}

const fn pte_flags(pte: Address) -> (PhyAddress, u64) {
    (phy_mask(pte) & !0xfff, pte & 0xfff)
}

const fn page_offset(gva: Address) -> u64 {
    gva & 0xfff
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VirtMemError {
    Pml4eNotPresent,
    PdpteNotPresent,
    PdeNotPresent,
    PteNotPresent,
}

impl fmt::Display for VirtMemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VirtMemError {
    fn description(&self) -> &str {
        "virtual to physical translation error"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

pub fn virt_read_u64(cr3: PhyAddress, gva: Address) -> u64 {
    let mut buf = [0; mem::size_of::<u64>()];
    virt_read_slice(cr3, gva, &mut buf);
    u64::from_le_bytes(buf)
}

pub fn virt_read_u32(cr3: PhyAddress, gva: Address) -> u32 {
    let mut buf = [0; mem::size_of::<u32>()];
    virt_read_slice(cr3, gva, &mut buf);
    u32::from_le_bytes(buf)
}

pub fn virt_read_u16(cr3: PhyAddress, gva: Address) -> u16 {
    let mut buf = [0; mem::size_of::<u16>()];
    virt_read_slice(cr3, gva, &mut buf);
    u16::from_le_bytes(buf)
}

pub fn virt_read_u8(cr3: PhyAddress, gva: Address) -> u8 {
    let mut buf = [0; mem::size_of::<u8>()];
    virt_read_slice(cr3, gva, &mut buf);
    u8::from_le_bytes(buf)
}

fn chunked(start: Address, sz: usize) -> impl Iterator<Item = (Address, usize)> {
    debug_assert!(start.checked_add(sz as u64).is_some());

    let mut remaining = sz;
    let mut base = start;

    iter::from_fn(move || {
        if remaining == 0 {
            None
        } else {
            let chunk_base = base;

            let chunk_sz = if base as usize + remaining > (base as usize & !0xfff) + 0x1000 {
                ((base & !0xfff) + 0x1000 - base) as usize
            } else {
                remaining
            };

            base += chunk_sz as Address;
            remaining -= chunk_sz;

            Some((chunk_base, chunk_sz))
        }
    })
}

pub fn virt_read(cr3: PhyAddress, gva: Address, buf: &mut Vec<u8>, sz: usize) {
    virt_read_checked(cr3, gva, buf, sz).unwrap()
}

pub fn virt_read_checked(
    cr3: PhyAddress,
    gva: Address,
    buf: &mut Vec<u8>,
    sz: usize,
) -> Result<(), VirtMemError> {
    debug_assert!(gva.checked_add(sz as u64).is_some());

    let len = buf.len();
    buf.reserve(sz);

    unsafe {
        buf.set_len(len + sz);

        let r = virt_read_slice_checked(cr3, gva, &mut buf[len..len + sz]);

        // if we errored, roll the length back to the original
        if r.is_err() {
            buf.set_len(len);
        }

        r
    }
}

pub fn virt_read_slice(cr3: PhyAddress, gva: Address, buf: &mut [u8]) {
    virt_read_slice_checked(cr3, gva, buf).unwrap()
}

pub fn virt_read_slice_checked(
    cr3: PhyAddress,
    gva: Address,
    buf: &mut [u8],
) -> Result<(), VirtMemError> {
    debug_assert!(gva.checked_add(buf.len() as u64).is_some());

    let mut off = 0;

    for (start, sz) in chunked(gva, buf.len()) {
        let gpa = virt_translate_checked(cr3, start)?;
        phy_read_slice(gpa, &mut buf[off..off + sz]);
        off += sz;
    }

    Ok(())
}

pub fn virt_write(cr3: PhyAddress, gva: Address, buf: &[u8]) {
    virt_write_checked(cr3, gva, buf).unwrap()
}

pub fn virt_write_checked(cr3: PhyAddress, gva: Address, buf: &[u8]) -> Result<(), VirtMemError> {
    debug_assert!(gva.checked_add(buf.len() as u64).is_some());

    let mut off = 0;

    for (start, sz) in chunked(gva, buf.len()) {
        let gpa = virt_translate_checked(cr3, start)?;

        phy_write(gpa, &buf[off..off + sz]);

        off += sz;
    }

    Ok(())
}

pub fn virt_translate(cr3: PhyAddress, gva: Address) -> PhyAddress {
    virt_translate_checked(cr3, gva).unwrap()
}

pub fn virt_translate_checked(cr3: PhyAddress, gva: Address) -> Result<PhyAddress, VirtMemError> {
    let (pml4_base, _) = base_flags(cr3);

    let pml4e_addr = pml4_base + pml4_index(gva) * 8;
    let pml4e = phy_read_u64(pml4e_addr);

    let (pdpt_base, pml4e_flags) = base_flags(pml4e);

    if pml4e_flags & 1 == 0 {
        return Err(VirtMemError::Pml4eNotPresent);
    }

    let pdpte_addr = pdpt_base + pdpt_index(gva) * 8;
    let pdpte = phy_read_u64(pdpte_addr);

    let (pd_base, pdpte_flags) = base_flags(pdpte);

    if pdpte_flags & 1 == 0 {
        return Err(VirtMemError::PdpteNotPresent);
    }

    // huge pages:
    // 7 (PS) - Page size; must be 1 (otherwise, this entry references a page
    // directory; see Table 4-1
    if pdpte_flags & 1 << 7 != 0 {
        return Ok((pd_base & 0xffff_ffff_c000_0000) + (gva & 0x3fff_ffff));
    }

    let pde_addr = pd_base + pd_index(gva) * 8;
    let pde = phy_read_u64(pde_addr);

    let (pt_base, pde_flags) = base_flags(pde);

    if pde_flags & 1 == 0 {
        return Err(VirtMemError::PdeNotPresent);
    }

    // large pages:
    // 7 (PS) - Page size; must be 1 (otherwise, this entry references a page
    // table; see Table 4-18
    if pde_flags & 1 << 7 != 0 {
        return Ok((pt_base & 0xffff_ffff_ffe0_0000) + (gva & 0x1f_ffff));
    }

    let pte_addr = pt_base + pt_index(gva) * 8;
    let pte = phy_read_u64(pte_addr);

    let (pte_paddr, pte_flags) = pte_flags(pte);

    if pte_paddr >> 63 != 0 {
        println!("wtf: {:x}", pte_paddr);
    }

    if pte_flags & 1 == 0 {
        return Err(VirtMemError::PteNotPresent);
    }

    Ok(pte_paddr + page_offset(gva))
}
