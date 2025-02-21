use std::collections::BTreeMap;
use std::ffi::{CStr, c_void};
use std::os::raw::c_char;
use std::ptr;

use crate::NUM_CPUS;
use crate::params::*;

lazy_static! {
    static ref PARAMS_ENUM: BTreeMap<&'static str, ParamEnum> = {
        let mut m = BTreeMap::new();

        // from cpudb.h
        m.insert(
            "cpu.model",
            ParamEnum::new(
                c"model",
                &[
                    c"bx_generic",
                    c"pentium",
                    c"pentium_mxx",
                    c"amd_k6_2_chomper",
                    c"p2_klamath",
                    c"p3_katmai",
                    c"p4_willamette",
                    c"core_duo_t2500_yonah",
                    c"atom_n270",
                    c"p4_prescott_celeron_336",
                    c"athlon64_clawhammer",
                    c"athlon64_venice",
                    c"turion64_tyler",
                    c"phenom_8650_toliman",
                    c"core2_penryn_t9600",
                    c"corei5_lynnfield_750",
                    c"corei5_arrandale_m520",
                    c"corei7_sandy_bridge_2600k",
                    c"zambezi",
                    c"trinity_apu",
                    c"ryzen",
                    c"corei7_ivy_bridge_3770k",
                    c"corei7_haswell_4770",
                    c"broadwell_ult",
                    c"corei7_skylake_x",
                    c"corei3_cnl",
                    c"corei7_icelake_u",
                    c"tigerlake",
                ],
                27 // default to tigerlake
            )
        );

        m.insert(
            "cpuid.apic",
            ParamEnum::new(
                c"apic",
                &[
                    c"legacy",
                    c"xapic",
                    c"xapic_ext",
                    c"x2apic",
                ],
                3
            )
        );

        m.insert(
            "cpuid.simd",
            ParamEnum::new(
                c"simd",
                &[
                    c"none",
                    c"sse",
                    c"sse2",
                    c"sse3",
                    c"ssse3",
                    c"sse4_1",
                    c"sse4_2",
                    c"avx",
                    c"avx2",
                ],
                8
            )
        );

        m
    };

    static ref PARAMS_NUM: BTreeMap<&'static str, ParamNum> = {
        let mut m = BTreeMap::new();

        m.insert("cpu.n_threads", ParamNum::new(c"n_threads", 1, 4, 1));
        m.insert("cpu.n_cores", ParamNum::new(c"n_cores", 1, 8, 1));
        m.insert("cpu.n_processors", ParamNum::new(c"n_processors", 1, NUM_CPUS as u64, 1));
        m.insert("cpu.quantum", ParamNum::new(c"quantum", 1, 32, 16));

        m.insert("cpuid.level", ParamNum::new(c"level", 5, 6, 6));
        m.insert("cpuid.vmx", ParamNum::new(c"vmx", 0, 2, 2));
        m.insert("cpuid.bmi", ParamNum::new(c"bmi", 0, 2, 2));

        // cannot find values for these vvv
        m.insert("cpuid.stepping", ParamNum::new(c"stepping", 0, 0, 0));
        m.insert("cpuid.model", ParamNum::new(c"model", 0, 0, 0));
        m.insert("cpuid.family", ParamNum::new(c"family", 0, 6, 6));

        m
    };

    static ref PARAMS_BOOL: BTreeMap<&'static str, ParamBool> = {
        let mut m = BTreeMap::new();

        m.insert("cpuid.mmx", ParamBool::new(c"mmx", true));
        m.insert("cpuid.sse4a", ParamBool::new(c"sse4a", true));
        m.insert("cpuid.misaligned_sse", ParamBool::new(c"misaligned_sse", true));
        m.insert("cpuid.sep", ParamBool::new(c"sep", true));
        m.insert("cpuid.xsave", ParamBool::new(c"xsave", true));
        m.insert("cpuid.xsaveopt", ParamBool::new(c"xsaveopt", true));
        m.insert("cpuid.aes", ParamBool::new(c"aes", true));
        m.insert("cpuid.sha", ParamBool::new(c"sha", true));
        m.insert("cpuid.adx", ParamBool::new(c"adx", true));
        m.insert("cpuid.x86_64", ParamBool::new(c"x86_64", true));
        m.insert("cpuid.fsgsbase", ParamBool::new(c"fsgsbase", true));
        m.insert("cpuid.pcid", ParamBool::new(c"pcid", true));
        m.insert("cpuid.smep", ParamBool::new(c"smep", true));
        m.insert("cpuid.smap", ParamBool::new(c"smap", true));

        m.insert("cpuid.mwait", ParamBool::new(c"mwait", false));
        m.insert("cpuid.movbe", ParamBool::new(c"movbe", false));
        m.insert("cpuid.1g_pages", ParamBool::new(c"1g_pages", false));
        m.insert("cpuid.avx_f16c", ParamBool::new(c"avx_f16c", true));
        m.insert("cpuid.avx_fma", ParamBool::new(c"avx_fma", true));
        m.insert("cpuid.fma4", ParamBool::new(c"fma4", false));
        m.insert("cpuid.xop", ParamBool::new(c"xop", false));
        m.insert("cpuid.tbm", ParamBool::new(c"tbm", false));

        m.insert("cpu.cpuid_limit_winnt", ParamBool::new(c"cpuid_limit_winnt", false));
        m.insert("cpu.ignore_bad_msrs", ParamBool::new(c"ignore_bad_msrs", false));
        // this needs to be set to false, because the reset path calls DEV_cmos_get_reg(0x0f),
        // which segfaults as I haven't implemented that stub yet...
        m.insert("cpu.reset_on_triple_fault", ParamBool::new(c"reset_on_triple_fault", false));
        m.insert("cpu.ignore_bad_msrs", ParamBool::new(c"ignore_base_msrs", true));

        m
    };

    static ref PARAMS_STRING: BTreeMap<&'static str, ParamString> = {
        let mut m = BTreeMap::new();

        // this key just needs to exist, doesnt need to be a valid file name
        m.insert("cpu.msrs", ParamString::new(c"msrs", c""));
        m.insert(
            "cpu.brand_string",
            ParamString::new(
                c"Intel(R) Core(TM) i7-7800X CPU @ 3.50GHz",
                c""
            )
        );

        m.insert(
            "cpu.add_features",
            ParamString::new(c"add_features", c"")
        );

        m.insert(
            "cpu.exclude_features",
            ParamString::new(c"exclude_features", c"")
        );


        m
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sim_get_param_enum(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up enum param for {}...", s);

    match PARAMS_ENUM.get(&s) {
        None => {
            warn!("no enum parameter: {}", s);
            ptr::null_mut::<c_void>()
        }
        Some(v) => v.0,
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sim_get_param_num(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up num param for {}...", s);

    match PARAMS_NUM.get(&s) {
        None => {
            warn!("no num parameter: {}", s);
            ptr::null_mut::<c_void>()
        }
        Some(v) => v.0,
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sim_get_param_bool(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up bool param for {}...", s);

    match PARAMS_BOOL.get(&s) {
        None => {
            warn!("no bool parameter: {}", s);
            ptr::null_mut::<c_void>()
        }
        Some(v) => v.0,
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sim_get_param_string(p: *const c_char) -> *mut c_void {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    trace!("looking up string param for {}...", s);

    match PARAMS_STRING.get(&s) {
        None => {
            warn!("no string parameter: {}", s);
            ptr::null_mut::<c_void>()
        }
        Some(v) => v.0,
    }
}
