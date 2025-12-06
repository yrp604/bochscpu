use std::env;

// Links are from https://github.com/yrp604/bochscpu-build/releases/tag/v0.5

#[cfg(all(target_arch = "x86_64", target_os = "windows", debug_assertions))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-windows-latest-x64-MD.zip";
#[cfg(all(target_arch = "x86_64", target_os = "windows", not(debug_assertions)))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-windows-latest-x64-MD.zip";
#[cfg(all(target_arch = "aarch64", target_os = "windows", debug_assertions))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-windows-11-arm-arm64-MD.zip";
#[cfg(all(target_arch = "aarch64", target_os = "windows", not(debug_assertions)))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-windows-11-arm-arm64-MT.zip";
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-ubuntu-latest-x64.zip";
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
const BOCHSCPU_BUILD_URL: &str = "https://github.com/yrp604/bochscpu-build/releases/download/v0.5/bochscpu-build-ubuntu-24.04-arm-arm64.zip";

fn download_bochscpu_build() {
    let mut response = reqwest::blocking::get(BOCHSCPU_BUILD_URL).unwrap();
    let config = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };

    #[cfg(target_os = "linux")]
    let tempfile = std::path::PathBuf::from(format!(
        "{}/bochscpu-build-{}.zip",
        std::env::var("TEMP").unwrap_or("/tmp".to_string()),
        config
    ));
    #[cfg(target_os = "windows")]
    let tempfile = std::path::PathBuf::from(format!(
        "{}/bochscpu-build-{}.zip",
        std::env::var("TEMP").unwrap_or("C:\\".to_string()),
        config
    ));

    if !tempfile.is_file() {
        let mut dest_file = std::fs::File::create(&tempfile).unwrap();
        std::io::copy(&mut response, &mut dest_file).unwrap();
    }

    let package_file = std::fs::File::open(&tempfile).unwrap();
    let mut archive = zip::ZipArchive::new(package_file).unwrap();

    archive.extract(".").unwrap();
}

fn main() {
    if !std::fs::exists("./lib").unwrap() {
        download_bochscpu_build();
    }

    // TODO figure out why the CFLAGS arent being inherited...
    // .flag("-fsanitize=address").flag("-Wno-unused-parameter")
    //
    #[cfg(target_os = "windows")]
    const CRT_STATIC: bool = cfg!(target_feature = "crt-static");

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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
                .static_crt(CRT_STATIC)
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
                .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
        .static_crt(CRT_STATIC)
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
