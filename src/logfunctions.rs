use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic;
use std::process;

/// hack to ensure that we capture an unwinding panic and just ignore it 
fn log_ignoring_late_panics(f: impl FnOnce() + panic::UnwindSafe) {
    let _unused = panic::catch_unwind(f);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_error(p: *const c_char) {
    log_ignoring_late_panics(|| {
        let s = unsafe {
            assert!(!p.is_null());

            CStr::from_ptr(p).to_str().unwrap()
        };

        error!("{}", s);
    });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_fatal1(p: *const c_char) {
    logfunctions_error(p);

    process::exit(1);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_info(p: *const c_char) {
    log_ignoring_late_panics(|| {
        let s = unsafe {
            assert!(!p.is_null());

            CStr::from_ptr(p).to_str().unwrap()
        };

        info!("{}", s);
    });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_ldebug(p: *const c_char) {
    log_ignoring_late_panics(|| {
        let s = unsafe {
            assert!(!p.is_null());

            CStr::from_ptr(p).to_str().unwrap()
        };

        debug!("{}", s);
    });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_lwarn(p: *const c_char) {
    log_ignoring_late_panics(|| {
        let s = unsafe {
            assert!(!p.is_null());

            CStr::from_ptr(p).to_str().unwrap()
        };

        warn!("{}", s);
    });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn logfunctions_panic(p: *const c_char) {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    println!("{}", s);
    process::exit(1);
}
