extern crate libc;

use std::mem;
use std::ptr;
use std::ffi::CStr;

pub const UNKNOWN: &str = "???";

pub fn get_user() -> String { 
    unsafe {
        let uid = libc::getuid();
        let mut result = ptr::null_mut();
        let pwmax = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(pwmax);
        let mut passwd: libc::passwd = mem::zeroed();
        match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(), 
                               buf.capacity() as libc::size_t, &mut result) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_name as *const _;
                CStr::from_ptr(ptr).to_str().unwrap().to_owned()
            },
            _ => String::from(UNKNOWN)
        }
    }
}

pub fn get_host() -> String { 
    unsafe {
        let len = 255;
        let mut buf = Vec::<u8>::with_capacity(len);
        let ptr = buf.as_mut_ptr() as *mut libc::c_char;
        if libc::gethostname(ptr, len as libc::size_t) != 0 {
            String::from(UNKNOWN)
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}
