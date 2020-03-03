use bindings;
use std::ffi::CString;

/// Checks `input` for SQL injection detection, and returns an option of (is_sqli, fingerprint)
pub fn sqli(input: &str) -> Option<(bool, String)> {
    let fingerprint_cstring = CString::new("").ok()?;
    let fingerprint_raw_ptr = fingerprint_cstring.into_raw();
    let input_cstring = CString::new(input).ok()?;
    let input_ptr = input_cstring.as_ptr();
    let is_sqli =
        unsafe { bindings::libinjection_sqli(input_ptr, input.len() as u64, fingerprint_raw_ptr) };
    Some((
        is_sqli == 1,
        unsafe { CString::from_raw(fingerprint_raw_ptr) }
            .into_string()
            .ok()?,
    ))
}

/// Checks `input` for XSS detection, and returns an option of is_xss
pub fn xss(input: &str) -> Option<bool> {
    let input_cstring = CString::new(input).ok()?;
    let input_ptr = input_cstring.as_ptr();
    let is_xss = unsafe { bindings::libinjection_xss(input_ptr, input.len() as u64) };
    Some(is_xss == 1)
}
