use std::env;
use std::fs::{read_to_string, rename};
use std::path::Path;
use std::process::Command;

fn fetch_bochs() {
    let rev = read_to_string("BOCHS").unwrap_or(String::from("HEAD"));

    #[cfg(target_os = "windows")]
    let mut child = Command::new("wsl.exe")
        .args(&[
            "svn",
            "checkout",
            "--revision",
            &rev,
            "http://svn.code.sf.net/p/bochs/code/trunk/bochs",
            "bochs",
        ])
        .spawn()
        .expect("Could not launch svn under wsl");

    #[cfg(not(target_os = "windows"))]
    let mut child = Command::new("svn")
        .args(&[
            "checkout",
            "--revision",
            &rev,
            "http://svn.code.sf.net/p/bochs/code/trunk/bochs",
            "bochs",
        ])
        .spawn()
        .expect("Could not launch svn");

    assert_eq!(child.wait().unwrap().code().unwrap(), 0);
}

fn config_bochs() {
    #[cfg(target_os = "windows")]
    let mut child = Command::new("wsl.exe")
        .args(&["sh", "-c", "cd bochs && sh .conf.cpu-msvc"])
        .spawn()
        .expect("Could not configure bochs under wsl");
    #[cfg(not(target_os = "windows"))]
    let mut child = Command::new("sh")
        .args(&["-c", "cd bochs && sh .conf.cpu"])
        .spawn()
        .expect("Could not configure bochs");

    assert_eq!(child.wait().unwrap().code().unwrap(), 0);
}

fn build_bochs() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd")
            .args(&["/C", "cd bochs && nmake"])
            .status();
        let _ = Command::new("cmd")
            .args(&["/C", "cd bochs/cpu/fpu && nmake"])
            .status();

        rename("bochs/cpu/libcpu.a", "bochs/cpu/cpu.lib").unwrap();
        rename("bochs/cpu/fpu/libfpu.a", "bochs/cpu/fpu/fpu.lib").unwrap();
        rename("bochs/cpu/avx/libavx.a", "bochs/cpu/avx/avx.lib").unwrap();
        rename("bochs/cpu/cpudb/libcpudb.a", "bochs/cpu/cpudb/cpudb.lib").unwrap();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("sh")
            .args(&["-c", "cd bochs && make"])
            .status();
    }
}

fn main() {
    // TODO figure out why the CFLAGS arent being inherited...
    // .flag("-fsanitize=address").flag("-Wno-unused-parameter")
    //

    fetch_bochs();
    config_bochs();
    build_bochs();

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/cpu-cabi.cc")
        .compile("cpu-cabi");

    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/cpu-cabi.cc")
        .compile("cpu-cabi");

    println!("cargo:rustc-link-lib=static=cpu-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/mem-cabi.cc")
        .compile("mem-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/mem-cabi.cc")
        .compile("mem-cabi");

    println!("cargo:rustc-link-lib=static=mem-cabi");

    match env::var("PROFILE").unwrap().as_ref() {
        "release" => {
            #[cfg(not(target_os = "windows"))]
            cc::Build::new()
                .define("RUST_CC_RELEASE", None)
                .cpp(true)
                .flag_if_supported("-Wno-unused-parameter")
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");

            #[cfg(target_os = "windows")]
            cc::Build::new()
                .define("RUST_CC_RELEASE", None)
                .define("WIN32", None)
                .cpp(true)
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");
        }
        _ => {
            #[cfg(not(target_os = "windows"))]
            cc::Build::new()
                .cpp(true)
                .flag_if_supported("-Wno-unused-parameter")
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");

            #[cfg(target_os = "windows")]
            cc::Build::new()
                .cpp(true)
                .define("WIN32", None)
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");
        }
    }
    println!("cargo:rustc-link-lib=static=logfunctions-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/siminterface-cabi.cc")
        .compile("siminterface-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/siminterface-cabi.cc")
        .compile("siminterface-cabi");

    println!("cargo:rustc-link-lib=static=siminterface-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("bochs/gui"))
        .include(Path::new("cabi"))
        .file("cabi/paramtree.cc")
        .compile("paramtree");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("bochs/gui"))
        .include(Path::new("cabi"))
        .file("cabi/paramtree.cc")
        .compile("paramtree");
    println!("cargo:rustc-link-lib=static=paramtree");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/devices-cabi.cc")
        .compile("devices-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/devices-cabi.cc")
        .compile("devices-cabi");

    println!("cargo:rustc-link-lib=static=devices-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/dbg.cc")
        .compile("dbg");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/dbg.cc")
        .compile("dbg");

    println!("cargo:rustc-link-lib=static=dbg");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/gui.cc")
        .compile("gui");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/gui.cc")
        .compile("gui");

    println!("cargo:rustc-link-lib=static=gui");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/system-cabi.cc")
        .compile("system-cabi");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/system-cabi.cc")
        .compile("system-cabi");
    println!("cargo:rustc-link-lib=static=system-cabi");

    #[cfg(not(target_os = "windows"))]
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/apic.cc")
        .compile("apic");
    #[cfg(target_os = "windows")]
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/apic.cc")
        .compile("apic");

    println!("cargo:rustc-link-lib=static=apic");

    println!("cargo:rustc-link-search=bochs/cpu");
    println!("cargo:rustc-link-lib=static=cpu");

    println!("cargo:rustc-link-search=bochs/cpu/fpu");
    println!("cargo:rustc-link-lib=static=fpu");

    println!("cargo:rustc-link-search=bochs/cpu/cpudb");
    println!("cargo:rustc-link-lib=static=cpudb");

    println!("cargo:rustc-link-search=bochs/cpu/avx");
    println!("cargo:rustc-link-lib=static=avx");
}
