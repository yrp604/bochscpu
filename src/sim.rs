use std::collections::BTreeMap;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

use crate::NUM_CPUS;
use crate::params::*;

macro_rules! cstr { ($s:literal) => {
    unsafe {
        CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes())
    }
}}

lazy_static! {
    static ref PARAMS_ENUM: BTreeMap<&'static str, ParamEnum> = {
        let mut m = BTreeMap::new();

        // FIXME this should probably be filled out. It might work now?
        m.insert(
            "cpu.model",
            ParamEnum::new(
                cstr!("model"),
                &[cstr!("corei7_sandy_bridge_2600k")],
                0
            )
        );

        m.insert(
            "cpuid.apic",
            ParamEnum::new(
                cstr!("apic"),
                &[
                    cstr!("legacy"),
                    cstr!("xapic"),
                    cstr!("xapic_ext"),
                    cstr!("x2apic"),
                ],
                3
            )
        );

        m.insert(
            "cpuid.simd",
            ParamEnum::new(
                cstr!("simd"),
                &[
                    cstr!("none"),
                    cstr!("sse"),
                    cstr!("sse2"),
                    cstr!("sse3"),
                    cstr!("ssse3"),
                    cstr!("sse4_1"),
                    cstr!("sse4_2"),
                ],
                6
            )
        );

        m
    };

    static ref PARAMS_NUM: BTreeMap<&'static str, ParamNum> = {
        let mut m = BTreeMap::new();

        m.insert("cpu.n_threads", ParamNum::new(cstr!("n_threads"), 1, 4, 1));
        m.insert("cpu.n_cores", ParamNum::new(cstr!("n_cores"), 1, 8, 1));
        m.insert("cpu.n_processors", ParamNum::new(cstr!("n_processors"), 1, NUM_CPUS as u64, 1));

        m.insert("cpuid.level", ParamNum::new(cstr!("level"), 5, 6, 6));
        m.insert("cpuid.vmx", ParamNum::new(cstr!("vmx"), 0, 2, 2));

        m
    };

    static ref PARAMS_BOOL: BTreeMap<&'static str, ParamBool> = {
        let mut m = BTreeMap::new();

        m.insert("cpuid.mmx", ParamBool::new(cstr!("mmx"), true));
        m.insert("cpuid.sse4a", ParamBool::new(cstr!("sse4a"), true));
        m.insert("cpuid.misaligned_sse", ParamBool::new(cstr!("misaligned_sse"), true));
        m.insert("cpuid.sep", ParamBool::new(cstr!("sep"), true));
        m.insert("cpuid.xsave", ParamBool::new(cstr!("xsave"), true));
        m.insert("cpuid.xsaveopt", ParamBool::new(cstr!("xsaveopt"), true));
        m.insert("cpuid.aes", ParamBool::new(cstr!("aes"), true));
        m.insert("cpuid.sha", ParamBool::new(cstr!("sha"), true));
        m.insert("cpuid.adx", ParamBool::new(cstr!("adx"), true));
        m.insert("cpuid.x86_64", ParamBool::new(cstr!("x86_64"), true));
        m.insert("cpuid.fsgsbase", ParamBool::new(cstr!("fsgsbase"), true));
        m.insert("cpuid.pcid", ParamBool::new(cstr!("pcid"), true));
        m.insert("cpuid.smep", ParamBool::new(cstr!("smep"), true));
        m.insert("cpuid.smap", ParamBool::new(cstr!("smap"), true));

        m.insert("cpuid.mwait", ParamBool::new(cstr!("mwait"), false));
        m.insert("cpuid.movbe", ParamBool::new(cstr!("movbe"), false));
        m.insert("cpuid.1g_pages", ParamBool::new(cstr!("1g_pages"), false));

        m.insert("cpu.cpuid_limit_winnt", ParamBool::new(cstr!("cpuid_limit_winnt"), false));
        m.insert("cpu.ignore_bad_msrs", ParamBool::new(cstr!("ignore_bad_msrs"), false));

        m
    };
}

#[no_mangle]
extern "C" fn sim_get_param_enum(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up enum param for {}...", s);

    match PARAMS_ENUM.get(&s) {
        None => {
            warn!("no enum parameter: {}", s);
            0 as *mut c_void
        },
        Some(v) => v.0,
    }
}

#[no_mangle]
extern "C" fn sim_get_param_num(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up num param for {}...", s);

    match PARAMS_NUM.get(&s) {
        None => {
            warn!("no num parameter: {}", s);
            0 as *mut c_void
        },
        Some(v) => v.0,
    }
}

#[no_mangle]
extern "C" fn sim_get_param_bool(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up bool param for {}...", s);

    match PARAMS_BOOL.get(&s) {
        None => {
            warn!("no bool parameter: {}", s);
            0 as *mut c_void
        },
        Some(v) => v.0,
    }
}
