use std::ffi::CStr;
use std::os::raw::c_char;
use std::process;

#[no_mangle]
extern "C" fn logfunctions_error(p: *const c_char) {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    error!("{}", s);
}

#[no_mangle]
extern "C" fn logfunctions_fatal1(p: *const c_char) {
    logfunctions_error(p);

    process::exit(1);
}

#[no_mangle]
extern "C" fn logfunctions_info(p: *const c_char) {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    info!("{}", s);
}

#[no_mangle]
extern "C" fn logfunctions_ldebug(p: *const c_char) {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    debug!("{}", s);
}

#[no_mangle]
extern "C" fn logfunctions_panic(p: *const c_char) {
    let s = unsafe {
        assert!(!p.is_null());

        CStr::from_ptr(p).to_str().unwrap()
    };

    panic!("{}", s);
}
