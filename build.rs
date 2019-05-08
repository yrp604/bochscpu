use std::env;
use std::path::Path;

fn main() {
    // TODO figure out why the CFLAGS arent being inherited...
    // .flag("-fsanitize=address").flag("-Wno-unused-parameter")
    //

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/cpu-cabi.cc")
        .compile("cpu-cabi");
    println!("cargo:rustc-link-lib=static=cpu-cabi");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/mem-cabi.cc")
        .compile("mem-cabi");
    println!("cargo:rustc-link-lib=static=mem-cabi");

    match env::var("PROFILE").unwrap().as_ref() {
        "release" => {
            cc::Build::new()
                .define("RUST_CC_RELEASE", Some(""))
                .cpp(true)
                .flag("-Wno-unused-parameter")
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");
        },
        _ => {
            cc::Build::new()
                .cpp(true)
                .flag("-Wno-unused-parameter")
                .include(Path::new("bochs"))
                .include(Path::new("cabi"))
                .file("cabi/logfunctions-cabi.cc")
                .compile("logfunctions-cabi");
        }
    }
    println!("cargo:rustc-link-lib=static=logfunctions-cabi");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/siminterface-cabi.cc")
        .compile("siminterface-cabi");
    println!("cargo:rustc-link-lib=static=siminterface-cabi");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("bochs/gui"))
        .include(Path::new("cabi"))
        .file("cabi/paramtree.cc")
        .compile("paramtree");
    println!("cargo:rustc-link-lib=static=paramtree");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/devices-cabi.cc")
        .compile("devices-cabi");
    println!("cargo:rustc-link-lib=static=devices-cabi");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/dbg.cc")
        .compile("dbg");
    println!("cargo:rustc-link-lib=static=dbg");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/gui.cc")
        .compile("gui");
    println!("cargo:rustc-link-lib=static=gui");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
        .include(Path::new("bochs"))
        .include(Path::new("cabi"))
        .file("cabi/system-cabi.cc")
        .compile("system-cabi");
    println!("cargo:rustc-link-lib=static=system-cabi");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-parameter")
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
