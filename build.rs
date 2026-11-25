//! Download and setup bochscpu build in order to compile bochscpu
//!
//! By default, the latest build version will be attempted to be downloaded. A specific version can be
//! provided through the environment variable `BOCHSCPU_BUILD_VERSION` (e.g. `export BOCHSCPU_BUILD_VERSION=0.5`)

use serde_json::Value;
use std::env;
use std::io;
use std::fs::File;

fn get_bochscpu_build_url(version: Option<&str>) -> (String, String) {
    let version = version.unwrap_or("latest");

    let url = format!(
        "https://api.github.com/repos/yrp604/bochscpu-build/releases/{}",
        version
    );

    let mut req = ureq::get(&url)
        .header("User-Agent", "Mozilla/5.0");

    if let Ok(token) = std::env::var("GH_TOKEN") {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let res = req.call().unwrap();
    let text = res.into_body().read_to_string().unwrap();

    dbg!(&text);

    let js: Value = serde_json::from_str(text.as_str()).unwrap();

    // Expected filename format for releases (for v0.5+)

    #[cfg(all(
        target_arch = "x86_64",
        target_os = "windows",
        not(target_feature = "crt-static")
    ))]
    let filename: &str = "bochscpu-build-windows-latest-x64-MD.zip";
    #[cfg(all(
        target_arch = "x86_64",
        target_os = "windows",
        target_feature = "crt-static"
    ))]
    let filename: &str = "bochscpu-build-windows-latest-x64-MT.zip";
    #[cfg(all(
        target_arch = "aarch64",
        target_os = "windows",
        not(target_feature = "crt-static")
    ))]
    let filename: &str = "bochscpu-build-windows-11-arm-arm64-MD.zip";
    #[cfg(all(
        target_arch = "aarch64",
        target_os = "windows",
        target_feature = "crt-static"
    ))]
    let filename: &str = "bochscpu-build-windows-11-arm-arm64-MT.zip";
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    let filename: &str = "bochscpu-build-ubuntu-latest-x64.zip";
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    let filename: &str = "bochscpu-build-ubuntu-24.04-arm-arm64.zip";
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    let filename: &str = "bochscpu-build-macos-latest-arm64.zip";

    let asset = js["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["name"] == filename)
        .unwrap();

    (
        asset["name"].as_str().unwrap().to_string(),
        asset["browser_download_url"].as_str().unwrap().to_string(),
    )
}

fn download_bochscpu_build(url: &str) -> File {
    dbg!(url);

    let response = ureq::get(url).call().unwrap();
    let mut reader = response.into_body().into_reader();

    let mut tempfile = tempfile::tempfile().unwrap();

    io::copy(&mut reader, &mut tempfile).unwrap();

    tempfile
}

fn unpack_bochscpu_build(tempfile: File) {
    let mut archive = zip::ZipArchive::new(tempfile).unwrap();

    archive.extract(".").unwrap();
}

fn main() {
    if !std::fs::exists("./lib").unwrap() {
        let zip = match env::var("BOCHSCPU_BUILD_ARTIFACT_PATH") {
            Ok(path) => File::open(path).unwrap(),
            Err(_) => {
                let ver = std::env::var("BOCHSCPU_BUILD_VERSION").unwrap_or("latest".to_string());
                let (_fname, url) = get_bochscpu_build_url(Some(ver.as_str()));
                download_bochscpu_build(url.as_str())
            }
        };
        unpack_bochscpu_build(zip);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);

    // TODO figure out why the CFLAGS arent being inherited...
    // .flag("-fsanitize=address").flag("-Wno-unused-parameter")
    //
    #[cfg(target_os = "windows")]
    const CRT_STATIC: bool = cfg!(target_feature = "crt-static");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=cpu-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=instr-cabi");

    match env::var("PROFILE").unwrap().as_ref() {
        "release" => {
            #[cfg(not(target_os = "windows"))]
            cc::Build::new()
                .cargo_metadata(false)
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
                .cargo_metadata(false)
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
    // link-lib directive emitted later: see note near the end of this
    // function about mem-cabi/devices-cabi/logfunctions-cabi ordering.

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=siminterface-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=paramtree");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=dbg");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=gui");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=system-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=apic");

    // intentionally not linked, just compiled to get the C_ASSERTs to check
    // the mappings
    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cargo_metadata(false)
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
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=opcode");

    // rebuild if any file in cabi changes
    println!("cargo:rerun-if-changed=cabi");

    // Absolute dir for linker search
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/lib", manifest_dir);

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=cpu");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=fpu");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=cpudb");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=avx");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=static=softfloat");

    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=cpu");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=fpu");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=cpudb");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=avx");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=softfloat");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=mem-cabi");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=devices-cabi");
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=static:+whole-archive=logfunctions-cabi");

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
}
