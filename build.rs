//! Download and setup bochscpu build in order to compile bochscpu
//!
//! By default, the latest build version will be attempted to be downloaded. A specific version can be
//! provided through the environment variable `BOCHSCPU_BUILD_VERSION` (e.g. `export BOCHSCPU_BUILD_VERSION=0.5`)

use json;
use std::env;

fn get_bochscpu_build_url(version: Option<&str>) -> (String, String) {
    let version = version.unwrap_or("latest");
    let cli = reqwest::blocking::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (platform; rv:gecko-version) Gecko/gecko-trail Firefox/15")
        .build()
        .unwrap();
    let res = cli
        .get(format!(
            "https://api.github.com/repos/yrp604/bochscpu-build/releases/{}",
            version
        ))
        .send()
        .unwrap();
    let text = res.text().unwrap();
    dbg!(&text);
    let js = json::parse(text.as_str()).unwrap();

    // Expected filename format for releases (for v0.5+)

    #[cfg(all(target_arch = "x86_64", target_os = "windows", debug_assertions))]
    let filename: &str = "bochscpu-build-windows-latest-x64-MD.zip";
    #[cfg(all(target_arch = "x86_64", target_os = "windows", not(debug_assertions)))]
    let filename: &str = "bochscpu-build-windows-latest-x64-MD.zip";
    #[cfg(all(target_arch = "aarch64", target_os = "windows", debug_assertions))]
    let filename: &str = "bochscpu-build-windows-11-arm-arm64-MD.zip";
    #[cfg(all(target_arch = "aarch64", target_os = "windows", not(debug_assertions)))]
    let filename: &str = "bochscpu-build-windows-11-arm-arm64-MT.zip";
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    let filename: &str = "bochscpu-build-ubuntu-latest-x64.zip";
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    let filename: &str = "bochscpu-build-ubuntu-24.04-arm-arm64.zip";

    let asset = js["assets"]
        .members()
        .filter(|x| x["name"] == filename)
        .next()
        .unwrap();
    (
        asset["name"].to_string(),
        asset["browser_download_url"].to_string(),
    )
}

fn download_bochscpu_build(url: &str) {
    let mut response = reqwest::blocking::get(url).unwrap();
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
    let ver = std::env::var("BOCHSCPU_BUILD_VERSION").unwrap_or("latest".to_string());
    let (_fname, url) = get_bochscpu_build_url(Some(ver.as_str()));

    if !std::fs::exists("./lib").unwrap() {
        download_bochscpu_build(url.as_str());
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

    // rebuild if any file in cabi changes
    println!("cargo:rerun-if-changed=cabi");

    // Absolute dir for linker search
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/lib", manifest_dir);

    println!("cargo:rustc-link-lib=static=cpu");
    println!("cargo:rustc-link-lib=static=fpu");
    println!("cargo:rustc-link-lib=static=cpudb");
    println!("cargo:rustc-link-lib=static=avx");
    println!("cargo:rustc-link-lib=static=softfloat");
}
