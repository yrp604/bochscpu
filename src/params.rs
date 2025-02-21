use std::ffi::{CStr, c_void};
use std::marker::Sync;
use std::os::raw::c_char;
use std::ptr;

unsafe extern "C-unwind" {
    fn sim_new_param_enum(name: *const c_char, val: *const *const c_char, idx: u32) -> *mut c_void;
    fn sim_delete_param_enum(name: *mut c_void);

    fn sim_new_param_num(name: *const c_char, min: u64, max: u64, val: u64) -> *mut c_void;
    fn sim_delete_param_num(n: *mut c_void);

    fn sim_new_param_bool(name: *const c_char, val: u32) -> *mut c_void;
    fn sim_delete_param_bool(n: *mut c_void);

    fn sim_new_param_string(name: *const c_char, val: *const c_char, sz: u32) -> *mut c_void;
    fn sim_delete_param_string(n: *mut c_void);
}

pub struct ParamEnum(pub *mut c_void, Vec<*const c_char>);
impl ParamEnum {
    pub fn new(name: &'static CStr, val: &[&'static CStr], idx: usize) -> Self {
        let mut a: Vec<*const c_char> =
            val.iter().map(|x| x as *const _ as *const c_char).collect();
        assert!(idx < a.len());
        a.push(ptr::null());

        let p = unsafe { sim_new_param_enum(name.as_ptr(), a.as_ptr(), idx as u32) };

        Self(p, a)
    }
}
impl Drop for ParamEnum {
    fn drop(&mut self) {
        unsafe { sim_delete_param_enum(self.0) }
    }
}
unsafe impl Sync for ParamEnum {}

pub struct ParamNum(pub *mut c_void);
impl ParamNum {
    pub fn new(name: &'static CStr, min: u64, max: u64, val: u64) -> Self {
        let p = unsafe { sim_new_param_num(name.as_ptr(), min, max, val) };

        Self(p)
    }
}
impl Drop for ParamNum {
    fn drop(&mut self) {
        unsafe { sim_delete_param_num(self.0) }
    }
}
unsafe impl Sync for ParamNum {}

pub struct ParamBool(pub *mut c_void);
impl ParamBool {
    pub fn new(name: &'static CStr, val: bool) -> Self {
        let p = unsafe { sim_new_param_bool(name.as_ptr(), val as u32) };

        Self(p)
    }
}
impl Drop for ParamBool {
    fn drop(&mut self) {
        unsafe { sim_delete_param_bool(self.0) }
    }
}
unsafe impl Sync for ParamBool {}

pub struct ParamString(pub *mut c_void);
impl ParamString {
    pub fn new(name: &'static CStr, val: &'static CStr) -> Self {
        let p = unsafe {
            sim_new_param_string(
                name.as_ptr(),
                val.as_ptr(),
                val.to_bytes_with_nul().len() as _,
            )
        };

        Self(p)
    }
}
impl Drop for ParamString {
    fn drop(&mut self) {
        unsafe { sim_delete_param_string(self.0) }
    }
}
unsafe impl Sync for ParamString {}
