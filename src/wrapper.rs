use bindings;
use std::ffi::{CStr, CString};

/// Checks `input` for SQL injection detection, and returns an option of (is_sqli, fingerprint)
pub fn sqli(input: &str) -> Option<(bool, String)> {
    let mut fingerprint = ['\0'; 8];
    let fingerprint_ptr = fingerprint.as_mut_ptr() as *mut i8;
    let input_cstring = CString::new(input).ok()?;
    let input_ptr = input_cstring.as_ptr();
    let is_sqli =
        unsafe { bindings::libinjection_sqli(input_ptr, input.len() as u64, fingerprint_ptr) };
    let fingerprint = unsafe { CStr::from_ptr(fingerprint_ptr).to_str().ok()?.to_string() };
    Some((is_sqli == 1, fingerprint))
}

/// Checks `input` for XSS detection, and returns an option of is_xss
pub fn xss(input: &str) -> Option<bool> {
    let input_cstring = CString::new(input).ok()?;
    let input_ptr = input_cstring.as_ptr();
    let is_xss = unsafe { bindings::libinjection_xss(input_ptr, input.len() as u64) };
    Some(is_xss == 1)
}
