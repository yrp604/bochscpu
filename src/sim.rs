use std::collections::BTreeMap;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr;

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

        // from cpudb.h
        m.insert(
            "cpu.model",
            ParamEnum::new(
                cstr!("model"),
                &[
                    cstr!("bx_generic"),
                    cstr!("pentium"),
                    cstr!("pentium_mxx"),
                    cstr!("amd_k6_2_chomper"),
                    cstr!("p2_klamath"),
                    cstr!("p3_katmai"),
                    cstr!("p4_willamette"),
                    cstr!("core_duo_t2500_yonah"),
                    cstr!("atom_n270"),
                    cstr!("p4_prescott_celeron_336"),
                    cstr!("athlon64_clawhammer"),
                    cstr!("athlon64_venice"),
                    cstr!("turion64_tyler"),
                    cstr!("phenom_8650_toliman"),
                    cstr!("core2_penryn_t9600"),
                    cstr!("corei5_lynnfield_750"),
                    cstr!("corei5_arrandale_m520"),
                    cstr!("corei7_sandy_bridge_2600k"),
                    cstr!("zambezi"),
                    cstr!("trinity_apu"),
                    cstr!("ryzen"),
                    cstr!("corei7_ivy_bridge_3770k"),
                    cstr!("corei7_haswell_4770"),
                    cstr!("broadwell_ult"),
                    cstr!("corei7_skylake_x"),
                    cstr!("corei3_cnl"),
                ],
                24
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
                    cstr!("avx"),
                    cstr!("avx2"),
                ],
                8
            )
        );

        m
    };

    static ref PARAMS_NUM: BTreeMap<&'static str, ParamNum> = {
        let mut m = BTreeMap::new();

        m.insert("cpu.n_threads", ParamNum::new(cstr!("n_threads"), 1, 4, 1));
        m.insert("cpu.n_cores", ParamNum::new(cstr!("n_cores"), 1, 8, 1));
        m.insert("cpu.n_processors", ParamNum::new(cstr!("n_processors"), 1, NUM_CPUS as u64, 1));
        m.insert("cpu.quantum", ParamNum::new(cstr!("quantum"), 1, 32, 16));

        m.insert("cpuid.level", ParamNum::new(cstr!("level"), 5, 6, 6));
        m.insert("cpuid.vmx", ParamNum::new(cstr!("vmx"), 0, 2, 2));
        m.insert("cpuid.bmi", ParamNum::new(cstr!("bmi"), 0, 2, 2));

        // cannot find values for these vvv
        m.insert("cpuid.stepping", ParamNum::new(cstr!("stepping"), 0, 0, 0));
        m.insert("cpuid.model", ParamNum::new(cstr!("model"), 0, 0, 0));
        m.insert("cpuid.family", ParamNum::new(cstr!("family"), 0, 6, 6));

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
        m.insert("cpuid.avx_f16c", ParamBool::new(cstr!("avx_f16c"), true));
        m.insert("cpuid.avx_fma", ParamBool::new(cstr!("avx_fma"), true));
        m.insert("cpuid.fma4", ParamBool::new(cstr!("fma4"), false));
        m.insert("cpuid.xop", ParamBool::new(cstr!("xop"), false));
        m.insert("cpuid.tbm", ParamBool::new(cstr!("tbm"), false));

        m.insert("cpu.cpuid_limit_winnt", ParamBool::new(cstr!("cpuid_limit_winnt"), false));
        m.insert("cpu.ignore_bad_msrs", ParamBool::new(cstr!("ignore_bad_msrs"), false));
        // this is set to true assuming there is a hook on reset which will stop the emulation
        m.insert("cpu.reset_on_triple_fault", ParamBool::new(cstr!("reset_on_triple_fault"), true));
        m.insert("cpu.ignore_bad_msrs", ParamBool::new(cstr!("ignore_base_msrs"), true));

        m
    };

    static ref PARAMS_STRING: BTreeMap<&'static str, ParamString> = {
        let mut m = BTreeMap::new();

        // this key just needs to exist, doesnt need to be a valid file name
        m.insert("cpu.msrs", ParamString::new(cstr!("msrs"), cstr!("")));
        m.insert(
            "cpuid.brand_string",
            ParamString::new(
                cstr!("Intel(R) Core(TM) i7-7800X CPU @ 3.50GHz\0\0\0\0\0\0\0\0"),
                cstr!("")
            )
        );

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
            ptr::null_mut::<c_void>()
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
            ptr::null_mut::<c_void>()
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
            ptr::null_mut::<c_void>()
        },
        Some(v) => v.0,
    }
}

#[no_mangle]
extern "C" fn sim_get_param_string(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up string param for {}...", s);

    match PARAMS_STRING.get(&s) {
        None => {
            warn!("no string parameter: {}", s);
            ptr::null_mut::<c_void>()
        },
        Some(v) => v.0,
    }
}
