use std::env;

fn main() {
    // TODO figure out why the CFLAGS arent being inherited...
    // .flag("-fsanitize=address").flag("-Wno-unused-parameter")
    //
    const WIN_STATIC_CRT: bool = cfg!(feature = "win_static_crt");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/cpu-cabi.cc")
        .compile("cpu-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/cpu-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("cpu-cabi");
    println!("cargo:rustc-link-lib=static=cpu-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/mem-cabi.cc")
        .compile("mem-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/mem-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("mem-cabi");
    println!("cargo:rustc-link-lib=static=mem-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/instr-cabi.cc")
        .compile("instr-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/instr-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("instr-cabi");
    println!("cargo:rustc-link-lib=static=instr-cabi");

    match env::var("PROFILE").unwrap().as_ref() {
        "release" => {
            #[cfg(not(target_os = "windows"))]
            cc::Build::new()
                .define("RUST_CC_RELEASE", None)
                .cpp(true)
                .flag_if_supported("-Wno-unused-parameter")
                .include("bochs")
                .include("bochs/instrument/bochscpu")
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");

            #[cfg(target_os = "windows")]
            cc::Build::new()
                .define("RUST_CC_RELEASE", None)
                .define("WIN32", None)
                .cpp(true)
                .include("bochs")
                .include("bochs/instrument/bochscpu")
                .file("cabi/logfunctions-cabi.cc")
                .static_crt(WIN_STATIC_CRT)
                .compile("logfunctions-cabi");
        }
        _ => {
            #[cfg(not(target_os = "windows"))]
            cc::Build::new()
                .cpp(true)
                .flag_if_supported("-Wno-unused-parameter")
                .include("bochs")
                .include("bochs/instrument/bochscpu")
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");

            #[cfg(target_os = "windows")]
            cc::Build::new()
                .cpp(true)
                .define("WIN32", None)
                .include("bochs")
                .include("bochs/instrument/bochscpu")
                .file("cabi/logfunctions-cabi.cc")
                .static_crt(WIN_STATIC_CRT)
                .compile("logfunctions-cabi");
        }
    }
    println!("cargo:rustc-link-lib=static=logfunctions-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/siminterface-cabi.cc")
        .compile("siminterface-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/siminterface-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("siminterface-cabi");
    println!("cargo:rustc-link-lib=static=siminterface-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/gui")
        .include("bochs/instrument/bochscpu")
        .file("cabi/paramtree.cc")
        .compile("paramtree");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/gui")
        .include("bochs/instrument/bochscpu")
        .file("cabi/paramtree.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("paramtree");
    println!("cargo:rustc-link-lib=static=paramtree");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/devices-cabi.cc")
        .compile("devices-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/devices-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("devices-cabi");
    println!("cargo:rustc-link-lib=static=devices-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/dbg.cc")
        .compile("dbg");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/dbg.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("dbg");
    println!("cargo:rustc-link-lib=static=dbg");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/gui.cc")
        .compile("gui");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/gui.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("gui");
    println!("cargo:rustc-link-lib=static=gui");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/system-cabi.cc")
        .compile("system-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/system-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("system-cabi");
    println!("cargo:rustc-link-lib=static=system-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/apic.cc")
        .compile("apic");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/apic.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("apic");
    println!("cargo:rustc-link-lib=static=apic");

    // intentionally not linked, just compiled to get the C_ASSERTs to check
    // the mappings
    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/opcode-cabi.cc")
        .compile("opcode");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include("bochs")
        .include("bochs/instrument/bochscpu")
        .file("cabi/opcode-cabi.cc")
        .static_crt(WIN_STATIC_CRT)
        .compile("opcode");
    println!("cargo:rustc-link-lib=static=opcode");

    // Absolute dir for linker search
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/lib", manifest_dir);

    println!("cargo:rustc-link-lib=static=cpu");
    println!("cargo:rustc-link-lib=static=fpu");
    println!("cargo:rustc-link-lib=static=cpudb");
    println!("cargo:rustc-link-lib=static=avx");
    println!("cargo:rustc-link-lib=static=softfloat");
}
