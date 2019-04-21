use crate::{Address, PhyAddress};
use crate::mem::{phy_read_u64};

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
    (gpa & !0x1ff, gpa & 0x1ff)
}

const fn pte_flags(pte: Address) -> (PhyAddress, u64) {
    (pte & !0xfff, pte & 0xfff)
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

pub fn virt_read_checked(cr3: Address, gva: Address, buf: &mut Vec<u8>, sz: usize) -> Result<(), VirtMemError> {
    let gpa = virt_translate_checked(cr3, gva)?;
    Ok(())
}

pub fn virt_write_checked(cr3: PhyAddress, gva: Address, buf: &[u8]) -> Result<(), VirtMemError> {
    let gpa = virt_translate_checked(cr3, gva)?;
    Ok(())
}

pub fn virt_translate(cr3: PhyAddress, gva: Address) -> PhyAddress {
    virt_translate_checked(cr3, gva).unwrap()
}

pub fn virt_translate_checked(cr3: PhyAddress, gva: Address) -> Result<PhyAddress, VirtMemError> {
    let pml4e_addr = (cr3 & !0x1ff) + pml4_index(gva) * 8;
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
        return Ok((pdpte & 0xffff_ffff_c000_0000) + (gva & 0x3fff_ffff));
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
        return Ok((pde & 0xffff_ffff_ffe0_0000) + (gva & 0x1f_ffff));
    }

    let pte_addr = pt_base + pt_index(gva) * 8;
    let pte = phy_read_u64(pte_addr);

    let (pte_paddr, pte_flags) = pte_flags(pte);

    if pte_flags & 1 == 0 {
        return Err(VirtMemError::PteNotPresent);
    }

    return Ok(pte_paddr + page_offset(gva));
}
